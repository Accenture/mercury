//
// Copyright 2018-2026 Accenture Technology
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! A RESP2 test double for the graph state store's Redis - the same approach as the
//! `minigraph-state-redis` crate's contract tests: the double stands in for redis-server, never
//! the client, so the client wire path (TCP, RESP2 frames, SETEX/GETDEL) stays fully real. Shared
//! by the suspend/resume tests of this example through `#[path = "common/redis_double.rs"]`.
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

// ---- the RESP2 test double (same approach as the minigraph-state-redis
// contract tests — the double stands in for redis-server, never the client) ----

#[derive(Clone)]
struct StoredValue {
    value: Vec<u8>,
    expires_at: Option<Instant>,
}

type SharedStore = Arc<Mutex<HashMap<Vec<u8>, StoredValue>>>;

pub async fn start_resp_double() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let port = listener.local_addr().expect("addr").port();
    let store: SharedStore = Arc::new(Mutex::new(HashMap::new()));
    tokio::spawn(async move {
        loop {
            let Ok((socket, _)) = listener.accept().await else {
                return;
            };
            let store = store.clone();
            tokio::spawn(async move { serve_connection(socket, store).await });
        }
    });
    port
}

async fn serve_connection(mut socket: tokio::net::TcpStream, store: SharedStore) {
    let mut buffer: Vec<u8> = Vec::new();
    let mut chunk = [0u8; 4096];
    loop {
        while let Some((args, consumed)) = parse_resp_array(&buffer) {
            buffer.drain(..consumed);
            let reply = dispatch(&args, &store);
            if socket.write_all(&reply).await.is_err() {
                return;
            }
        }
        match socket.read(&mut chunk).await {
            Ok(0) | Err(_) => return,
            Ok(n) => buffer.extend_from_slice(&chunk[..n]),
        }
    }
}

fn parse_resp_array(buffer: &[u8]) -> Option<(Vec<Vec<u8>>, usize)> {
    let mut pos = 0;
    let (count, next) = parse_prefixed_int(buffer, pos, b'*')?;
    pos = next;
    let mut args = Vec::with_capacity(count.max(0) as usize);
    for _ in 0..count {
        let (len, next) = parse_prefixed_int(buffer, pos, b'$')?;
        pos = next;
        let end = pos + len as usize;
        if buffer.len() < end + 2 {
            return None;
        }
        args.push(buffer[pos..end].to_vec());
        pos = end + 2;
    }
    Some((args, pos))
}

fn parse_prefixed_int(buffer: &[u8], pos: usize, prefix: u8) -> Option<(i64, usize)> {
    if buffer.len() <= pos || buffer[pos] != prefix {
        return None;
    }
    let line_end = buffer[pos..].windows(2).position(|w| w == b"\r\n")? + pos;
    let text = std::str::from_utf8(&buffer[pos + 1..line_end]).ok()?;
    Some((text.parse().ok()?, line_end + 2))
}

fn dispatch(args: &[Vec<u8>], store: &SharedStore) -> Vec<u8> {
    let command = args
        .first()
        .map(|c| String::from_utf8_lossy(c).to_ascii_uppercase())
        .unwrap_or_default();
    let mut map = store.lock().expect("store");
    match command.as_str() {
        "PING" => b"+PONG\r\n".to_vec(),
        "CLIENT" | "SELECT" | "AUTH" => b"+OK\r\n".to_vec(),
        // the store detects its consume strategy from INFO server once per
        // process; report a 6.2+ version so this suite keeps exercising the
        // native GETDEL path (the transactional fallback has its own suite
        // in the store crate)
        "INFO" => {
            let body = "# Server\r\nredis_version:7.4.1\r\n";
            let mut reply = format!("${}\r\n", body.len()).into_bytes();
            reply.extend_from_slice(body.as_bytes());
            reply.extend_from_slice(b"\r\n");
            reply
        }
        "SETEX" if args.len() == 4 => {
            let seconds: u64 = String::from_utf8_lossy(&args[2]).parse().unwrap_or(0);
            map.insert(
                args[1].clone(),
                StoredValue {
                    value: args[3].clone(),
                    expires_at: Some(Instant::now() + Duration::from_secs(seconds)),
                },
            );
            b"+OK\r\n".to_vec()
        }
        "GETDEL" | "GET" if args.len() == 2 => {
            let live = match map.get(&args[1]) {
                Some(entry) if !expired(entry) => Some(entry.value.clone()),
                Some(_) => {
                    map.remove(&args[1]);
                    None
                }
                None => None,
            };
            match live {
                Some(value) => {
                    if command == "GETDEL" {
                        map.remove(&args[1]);
                    }
                    let mut reply = format!("${}\r\n", value.len()).into_bytes();
                    reply.extend_from_slice(&value);
                    reply.extend_from_slice(b"\r\n");
                    reply
                }
                None => b"$-1\r\n".to_vec(),
            }
        }
        "DEL" => {
            let mut removed = 0;
            for key in &args[1..] {
                if map.remove(key).is_some() {
                    removed += 1;
                }
            }
            format!(":{removed}\r\n").into_bytes()
        }
        _ => format!("-ERR unknown command '{command}'\r\n").into_bytes(),
    }
}

fn expired(entry: &StoredValue) -> bool {
    entry
        .expires_at
        .map(|at| Instant::now() >= at)
        .unwrap_or(false)
}
