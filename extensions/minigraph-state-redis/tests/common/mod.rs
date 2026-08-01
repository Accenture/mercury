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

//! The in-process **RESP2 test double** shared by the state-store contract
//! suites. It stands in for redis-server, never for the client: real TCP,
//! real protocol frames through the real `redis` crate — only the server
//! side is simulated. Point `redis.host`/`redis.port` at a real server and
//! the same functions run unchanged.
//!
//! Parameterized by the `redis_version` its `INFO server` reply reports, so
//! one suite exercises the native-GETDEL strategy (6.2+) and another the
//! MULTI/EXEC GET+DEL fallback (the redis-standalone Windows binary is
//! 5.0.14). Supports SETEX/GETDEL/GET/TTL/DEL/PING/INFO, MULTI/EXEC/DISCARD
//! with per-connection queueing, and tolerant handshake chatter. Every
//! dispatched command name is recorded in a journal so a suite can PROVE
//! which strategy ran on the wire.

#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct StoredValue {
    pub value: Vec<u8>,
    pub expires_at: Option<Instant>,
}

pub type SharedStore = Arc<Mutex<HashMap<Vec<u8>, StoredValue>>>;
pub type CommandJournal = Arc<Mutex<Vec<String>>>;

/// Bind an ephemeral port and serve the double, reporting the given
/// `redis_version` from `INFO server`. Values are binary-safe; expiry is
/// honored on read like a real server.
pub async fn start_resp_double(version: &str) -> (u16, SharedStore, CommandJournal) {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let port = listener.local_addr().expect("addr").port();
    let store: SharedStore = Arc::new(Mutex::new(HashMap::new()));
    let journal: CommandJournal = Arc::new(Mutex::new(Vec::new()));
    let shared = store.clone();
    let shared_journal = journal.clone();
    let version = version.to_string();
    tokio::spawn(async move {
        loop {
            let Ok((socket, _)) = listener.accept().await else {
                return;
            };
            let store = shared.clone();
            let journal = shared_journal.clone();
            let version = version.clone();
            tokio::spawn(async move { serve_connection(socket, store, journal, version).await });
        }
    });
    (port, store, journal)
}

async fn serve_connection(
    mut socket: tokio::net::TcpStream,
    store: SharedStore,
    journal: CommandJournal,
    version: String,
) {
    let mut buffer: Vec<u8> = Vec::new();
    let mut chunk = [0u8; 4096];
    // per-connection MULTI state: Some(queue) while a transaction is open -
    // queued commands reply +QUEUED and execute together on EXEC
    let mut queued: Option<Vec<Vec<Vec<u8>>>> = None;
    loop {
        // parse as many complete commands as the buffer holds
        while let Some((args, consumed)) = parse_resp_array(&buffer) {
            buffer.drain(..consumed);
            let command = command_name(&args);
            journal.lock().expect("journal").push(command.clone());
            let reply = match command.as_str() {
                "MULTI" => {
                    queued = Some(Vec::new());
                    b"+OK\r\n".to_vec()
                }
                "EXEC" => match queued.take() {
                    Some(commands) => {
                        let mut out = format!("*{}\r\n", commands.len()).into_bytes();
                        for queued_args in &commands {
                            out.extend(dispatch(queued_args, &store, &version));
                        }
                        out
                    }
                    None => b"-ERR EXEC without MULTI\r\n".to_vec(),
                },
                "DISCARD" => {
                    queued = None;
                    b"+OK\r\n".to_vec()
                }
                _ => match queued.as_mut() {
                    Some(queue) => {
                        queue.push(args.clone());
                        b"+QUEUED\r\n".to_vec()
                    }
                    None => dispatch(&args, &store, &version),
                },
            };
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

fn command_name(args: &[Vec<u8>]) -> String {
    args.first()
        .map(|c| String::from_utf8_lossy(c).to_ascii_uppercase())
        .unwrap_or_default()
}

/// Parse one RESP2 array of bulk strings; None when incomplete.
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
        pos = end + 2; // skip trailing \r\n
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

fn dispatch(args: &[Vec<u8>], store: &SharedStore, version: &str) -> Vec<u8> {
    let command = command_name(args);
    let mut map = store.lock().expect("store");
    match command.as_str() {
        "PING" => b"+PONG\r\n".to_vec(),
        // handshake chatter the client may send (CLIENT SETINFO, SELECT 0...)
        "CLIENT" | "SELECT" | "AUTH" => b"+OK\r\n".to_vec(),
        "INFO" => {
            let body = format!("# Server\r\nredis_version:{version}\r\n");
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
        "TTL" if args.len() == 2 => match map.get(&args[1]) {
            Some(entry) if !expired(entry) => {
                let remaining = entry
                    .expires_at
                    .map(|at| at.saturating_duration_since(Instant::now()).as_secs() as i64)
                    .unwrap_or(-1);
                format!(":{remaining}\r\n").into_bytes()
            }
            _ => b":-2\r\n".to_vec(),
        },
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
