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

//! The in-process **RESP2 test double** shared by Mercury's Redis-backed
//! suites. It stands in for redis-server, never for the client: real TCP,
//! real protocol frames through the real `redis` crate — only the server
//! side is simulated. Point `redis.host`/`redis.port` at a real server and
//! the same tests run unchanged.
//!
//! This crate exists because the Rust toolchain has no embedded redis-server
//! binary, and Docker is deliberately not a test dependency (VDI-class
//! development machines cannot virtualize). It is `publish = false` — a
//! dev-dependency of the Redis-backed crates, never shipped.
//!
//! **Supported surface**
//!
//! | Family | Commands |
//! |---|---|
//! | strings | `SET` (with `NX`/`XX`, `EX`/`PX`, `KEEPTTL`), `SETEX`, `GET`, `MGET`, `GETDEL`, `TTL`, `DEL`, `EXPIRE`, `EXISTS` |
//! | lists | `RPUSH`, `LPUSH`, `LPOP`, `LLEN` (an emptied list deletes its key, as a real server does) |
//! | pub/sub | `SUBSCRIBE`, `UNSUBSCRIBE`, `PUBLISH` (out-of-band `message` push frames) |
//! | transactions | `MULTI`, `EXEC`, `DISCARD` with per-connection queueing |
//! | housekeeping | `PING`, `INFO server`, `FLUSHALL`, and tolerant handshake chatter |
//! | auth | `AUTH` — see [`start_resp_double_with_password`] for the `requirepass` mode |
//!
//! Parameterized by the `redis_version` its `INFO server` reply reports, so
//! one suite can exercise a native-GETDEL strategy (6.2+) and another the
//! MULTI/EXEC fallback (the redis-standalone Windows binary is 5.0.14).
//! Wrong-type access answers `WRONGTYPE` like a real server. Every dispatched
//! command name is recorded in a journal, so a suite can PROVE which commands
//! went over the wire.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::sync::watch;
use tokio::task::JoinHandle;

/// One stored key: its payload plus the optional native expiry.
#[derive(Clone)]
pub struct StoredValue {
    pub data: StoredData,
    pub expires_at: Option<Instant>,
}

impl StoredValue {
    /// The string payload, or `None` when this key holds a list.
    pub fn text(&self) -> Option<&[u8]> {
        match &self.data {
            StoredData::Text(value) => Some(value),
            StoredData::List(_) => None,
        }
    }
}

/// A key's payload: Redis types the double needs to serve.
#[derive(Clone)]
pub enum StoredData {
    Text(Vec<u8>),
    List(VecDeque<Vec<u8>>),
}

pub type SharedStore = Arc<Mutex<HashMap<Vec<u8>, StoredValue>>>;
pub type CommandJournal = Arc<Mutex<Vec<String>>>;

/// One connection's outbound lane. Command replies and Pub/Sub pushes share
/// it, so a subscriber's frames never interleave mid-frame with a reply.
type Outbound = UnboundedSender<Vec<u8>>;
type Subscribers = Arc<Mutex<HashMap<String, Vec<(u64, Outbound)>>>>;

static CONNECTION_IDS: AtomicU64 = AtomicU64::new(0);

/// Bind an ephemeral port and serve the double, reporting the given
/// `redis_version` from `INFO server`. Values are binary-safe; expiry is
/// honored on read like a real server.
///
/// Returns the port, the shared store (so a suite can inspect wire-visible
/// state), and the command journal.
pub async fn start_resp_double(version: &str) -> (u16, SharedStore, CommandJournal) {
    start_with(version, None).await
}

/// [`start_resp_double`] in `requirepass` mode, like a real server whose
/// configuration demands `AUTH`: every command other than `AUTH` answers
/// `NOAUTH` until the connection authenticates, and a wrong credential
/// answers `WRONGPASS` — the exact server-side signatures a late-credential
/// deployment sees while a vault-published password has not landed yet.
/// (Conversely, `AUTH` against a no-password double answers the real
/// server's `ERR Client sent AUTH ...` — the third waiting signature.)
pub async fn start_resp_double_with_password(
    version: &str,
    password: &str,
) -> (u16, SharedStore, CommandJournal) {
    start_with(version, Some(password.to_string())).await
}

async fn start_with(
    version: &str,
    required_password: Option<String>,
) -> (u16, SharedStore, CommandJournal) {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let port = listener.local_addr().expect("addr").port();
    let store: SharedStore = Arc::new(Mutex::new(HashMap::new()));
    let journal: CommandJournal = Arc::new(Mutex::new(Vec::new()));
    let subscribers: Subscribers = Arc::new(Mutex::new(HashMap::new()));
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
            let subscribers = subscribers.clone();
            let version = version.clone();
            let required_password = required_password.clone();
            tokio::spawn(async move {
                serve_connection(
                    socket,
                    store,
                    journal,
                    subscribers,
                    version,
                    required_password,
                )
                .await
            });
        }
    });
    (port, store, journal)
}

/// Per-connection state: the shared server state plus this connection's own
/// outbound lane and subscriptions.
struct Connection {
    store: SharedStore,
    journal: CommandJournal,
    subscribers: Subscribers,
    version: String,
    id: u64,
    outbound: Outbound,
    subscribed: HashSet<String>,
    /// `requirepass` mode: the password `AUTH` must present, if any.
    required_password: Option<String>,
    /// Whether this connection has authenticated (always true without
    /// `requirepass`).
    authenticated: bool,
}

async fn serve_connection(
    socket: tokio::net::TcpStream,
    store: SharedStore,
    journal: CommandJournal,
    subscribers: Subscribers,
    version: String,
    required_password: Option<String>,
) {
    let (mut reader, mut writer) = socket.into_split();
    // every frame leaves through this lane, so a PUBLISH from another
    // connection can push a message while this one is idle in a read
    let (outbound, mut outgoing) = mpsc::unbounded_channel::<Vec<u8>>();
    let pump = tokio::spawn(async move {
        while let Some(frame) = outgoing.recv().await {
            if writer.write_all(&frame).await.is_err() {
                return;
            }
        }
    });
    let mut connection = Connection {
        store,
        journal,
        subscribers,
        version,
        id: CONNECTION_IDS.fetch_add(1, Ordering::Relaxed),
        outbound,
        subscribed: HashSet::new(),
        authenticated: required_password.is_none(),
        required_password,
    };
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
            connection
                .journal
                .lock()
                .expect("journal")
                .push(command.clone());
            // requirepass gate: a real server refuses everything except the
            // authentication commands until AUTH succeeds
            if !connection.authenticated
                && !matches!(command.as_str(), "AUTH" | "HELLO" | "QUIT" | "RESET")
            {
                if connection
                    .outbound
                    .send(b"-NOAUTH Authentication required.\r\n".to_vec())
                    .is_err()
                {
                    release_subscriptions(&connection);
                    return;
                }
                continue;
            }
            let reply = match command.as_str() {
                "MULTI" => {
                    queued = Some(Vec::new());
                    b"+OK\r\n".to_vec()
                }
                "EXEC" => match queued.take() {
                    Some(commands) => {
                        let mut out = format!("*{}\r\n", commands.len()).into_bytes();
                        for queued_args in &commands {
                            out.extend(dispatch(queued_args, &mut connection));
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
                    None => dispatch(&args, &mut connection),
                },
            };
            if connection.outbound.send(reply).is_err() {
                release_subscriptions(&connection);
                return;
            }
        }
        match reader.read(&mut chunk).await {
            Ok(0) | Err(_) => {
                release_subscriptions(&connection);
                drop(connection);
                pump.abort();
                return;
            }
            Ok(n) => buffer.extend_from_slice(&chunk[..n]),
        }
    }
}

/// Drop this connection's subscriptions when it goes away, so a later
/// PUBLISH does not try to push into a closed lane.
fn release_subscriptions(connection: &Connection) {
    if connection.subscribed.is_empty() {
        return;
    }
    let mut registry = connection.subscribers.lock().expect("subscribers");
    for channel in &connection.subscribed {
        if let Some(lanes) = registry.get_mut(channel) {
            lanes.retain(|(id, _)| *id != connection.id);
            if lanes.is_empty() {
                registry.remove(channel);
            }
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

fn dispatch(args: &[Vec<u8>], connection: &mut Connection) -> Vec<u8> {
    let command = command_name(args);
    match command.as_str() {
        // pub/sub touches the subscriber registry, not the key space
        "SUBSCRIBE" => return subscribe(args, connection),
        "UNSUBSCRIBE" => return unsubscribe(args, connection),
        "PUBLISH" if args.len() == 3 => return publish(args, connection),
        _ => {}
    }
    let mut map = connection.store.lock().expect("store");
    match command.as_str() {
        "PING" => b"+PONG\r\n".to_vec(),
        // handshake chatter the client may send (CLIENT SETINFO, SELECT 0...)
        "CLIENT" | "SELECT" => b"+OK\r\n".to_vec(),
        // real-server AUTH semantics: `AUTH pass` or `AUTH user pass`
        "AUTH" => match (&connection.required_password, args.last()) {
            (None, _) => {
                b"-ERR Client sent AUTH, but no password is set. Did you mean AUTH <username> <password>?\r\n"
                    .to_vec()
            }
            (Some(required), Some(supplied)) if supplied.as_slice() == required.as_bytes() => {
                connection.authenticated = true;
                b"+OK\r\n".to_vec()
            }
            (Some(_), _) => {
                b"-WRONGPASS invalid username-password pair or user is disabled.\r\n".to_vec()
            }
        },
        "INFO" => {
            let body = format!("# Server\r\nredis_version:{}\r\n", connection.version);
            bulk(body.as_bytes())
        }
        "FLUSHALL" => {
            map.clear();
            b"+OK\r\n".to_vec()
        }
        // the real server's option grammar after the value: NX | XX, EX seconds |
        // PX millis | KEEPTTL - the cache's atomic put-if-absent is `SET k v NX EX ttl`
        "SET" if args.len() >= 3 => {
            let (mut nx, mut xx, mut keep_ttl) = (false, false, false);
            let mut expires_at = None;
            let mut i = 3;
            while i < args.len() {
                let option = String::from_utf8_lossy(&args[i]).to_ascii_uppercase();
                match option.as_str() {
                    "NX" => nx = true,
                    "XX" => xx = true,
                    "KEEPTTL" => keep_ttl = true,
                    "EX" | "PX" if i + 1 < args.len() => {
                        let amount: u64 = String::from_utf8_lossy(&args[i + 1]).parse().unwrap_or(0);
                        let ttl = if option == "EX" {
                            Duration::from_secs(amount)
                        } else {
                            Duration::from_millis(amount)
                        };
                        expires_at = Some(Instant::now() + ttl);
                        i += 1;
                    }
                    _ => return b"-ERR syntax error\r\n".to_vec(),
                }
                i += 1;
            }
            let existing = live_entry(&mut map, &args[1]).map(|entry| entry.expires_at);
            if (nx && existing.is_some()) || (xx && existing.is_none()) {
                // the condition failed: nothing stored, a null reply
                return null_bulk();
            }
            if keep_ttl {
                expires_at = existing.flatten();
            }
            map.insert(
                args[1].clone(),
                StoredValue {
                    data: StoredData::Text(args[2].clone()),
                    expires_at,
                },
            );
            b"+OK\r\n".to_vec()
        }
        // one bulk (or null) per requested key; a list-typed key answers null
        // like a real server, never WRONGTYPE
        "MGET" if args.len() >= 2 => {
            let mut reply = format!("*{}\r\n", args.len() - 1).into_bytes();
            for key in &args[1..] {
                match live_entry(&mut map, key) {
                    Some(entry) => match &entry.data {
                        StoredData::Text(value) => reply.extend(bulk(value)),
                        StoredData::List(_) => reply.extend(null_bulk()),
                    },
                    None => reply.extend(null_bulk()),
                }
            }
            reply
        }
        "SETEX" if args.len() == 4 => {
            let seconds: u64 = String::from_utf8_lossy(&args[2]).parse().unwrap_or(0);
            map.insert(
                args[1].clone(),
                StoredValue {
                    data: StoredData::Text(args[3].clone()),
                    expires_at: Some(Instant::now() + Duration::from_secs(seconds)),
                },
            );
            b"+OK\r\n".to_vec()
        }
        "GETDEL" | "GET" if args.len() == 2 => {
            let live = match live_entry(&mut map, &args[1]) {
                Some(entry) => match &entry.data {
                    StoredData::Text(value) => Some(value.clone()),
                    StoredData::List(_) => return wrong_type(),
                },
                None => None,
            };
            match live {
                Some(value) => {
                    if command == "GETDEL" {
                        map.remove(&args[1]);
                    }
                    bulk(&value)
                }
                None => null_bulk(),
            }
        }
        "EXISTS" if args.len() == 2 => match live_entry(&mut map, &args[1]) {
            Some(_) => integer(1),
            None => integer(0),
        },
        "TTL" if args.len() == 2 => match live_entry(&mut map, &args[1]) {
            Some(entry) => {
                let remaining = entry
                    .expires_at
                    .map(|at| at.saturating_duration_since(Instant::now()).as_secs() as i64)
                    .unwrap_or(-1);
                integer(remaining)
            }
            None => integer(-2),
        },
        // EXPIRE on an existing key of either type - the second half of the
        // append-with-TTL contract, so no key is ever left without one
        "EXPIRE" if args.len() == 3 => {
            let seconds: u64 = String::from_utf8_lossy(&args[2]).parse().unwrap_or(0);
            match live_entry(&mut map, &args[1]) {
                Some(entry) => {
                    entry.expires_at = Some(Instant::now() + Duration::from_secs(seconds));
                    integer(1)
                }
                None => integer(0),
            }
        }
        "RPUSH" | "LPUSH" if args.len() >= 3 => {
            let entry = map.entry(args[1].clone()).or_insert_with(|| StoredValue {
                data: StoredData::List(VecDeque::new()),
                expires_at: None,
            });
            // an expired list is reborn empty, exactly like a real server
            if expired(entry) {
                entry.data = StoredData::List(VecDeque::new());
                entry.expires_at = None;
            }
            let StoredData::List(list) = &mut entry.data else {
                return wrong_type();
            };
            for value in &args[2..] {
                if command == "RPUSH" {
                    list.push_back(value.clone());
                } else {
                    list.push_front(value.clone());
                }
            }
            integer(list.len() as i64)
        }
        "LPOP" if args.len() == 2 => {
            let Some(entry) = live_entry(&mut map, &args[1]) else {
                return null_bulk();
            };
            let StoredData::List(list) = &mut entry.data else {
                return wrong_type();
            };
            match list.pop_front() {
                Some(value) => {
                    // a drained list ceases to exist, as a real server does
                    if list.is_empty() {
                        map.remove(&args[1]);
                    }
                    bulk(&value)
                }
                None => {
                    map.remove(&args[1]);
                    null_bulk()
                }
            }
        }
        "LLEN" if args.len() == 2 => match live_entry(&mut map, &args[1]) {
            Some(entry) => match &entry.data {
                StoredData::List(list) => integer(list.len() as i64),
                StoredData::Text(_) => wrong_type(),
            },
            None => integer(0),
        },
        "DEL" => {
            let mut removed = 0;
            for key in &args[1..] {
                if map.remove(key).is_some() {
                    removed += 1;
                }
            }
            integer(removed)
        }
        _ => format!("-ERR unknown command '{command}'\r\n").into_bytes(),
    }
}

fn subscribe(args: &[Vec<u8>], connection: &mut Connection) -> Vec<u8> {
    let mut reply = Vec::new();
    let mut registry = connection.subscribers.lock().expect("subscribers");
    for channel in &args[1..] {
        let name = String::from_utf8_lossy(channel).to_string();
        let lanes = registry.entry(name.clone()).or_default();
        if !lanes.iter().any(|(id, _)| *id == connection.id) {
            lanes.push((connection.id, connection.outbound.clone()));
        }
        connection.subscribed.insert(name);
        reply.extend(subscription_confirmation(
            b"subscribe",
            channel,
            connection.subscribed.len() as i64,
        ));
    }
    reply
}

fn unsubscribe(args: &[Vec<u8>], connection: &mut Connection) -> Vec<u8> {
    let channels: Vec<String> = if args.len() > 1 {
        args[1..]
            .iter()
            .map(|c| String::from_utf8_lossy(c).to_string())
            .collect()
    } else {
        connection.subscribed.iter().cloned().collect()
    };
    let mut reply = Vec::new();
    let mut registry = connection.subscribers.lock().expect("subscribers");
    for name in channels {
        if let Some(lanes) = registry.get_mut(&name) {
            lanes.retain(|(id, _)| *id != connection.id);
            if lanes.is_empty() {
                registry.remove(&name);
            }
        }
        connection.subscribed.remove(&name);
        reply.extend(subscription_confirmation(
            b"unsubscribe",
            name.as_bytes(),
            connection.subscribed.len() as i64,
        ));
    }
    reply
}

/// Push the payload to every live subscriber lane and report the count.
/// A lane whose connection has gone is dropped here (belt and braces with
/// [`release_subscriptions`]).
fn publish(args: &[Vec<u8>], connection: &mut Connection) -> Vec<u8> {
    let channel = String::from_utf8_lossy(&args[1]).to_string();
    let frame = message_frame(&args[1], &args[2]);
    let mut registry = connection.subscribers.lock().expect("subscribers");
    let mut delivered = 0;
    if let Some(lanes) = registry.get_mut(&channel) {
        lanes.retain(|(_, lane)| match lane.send(frame.clone()) {
            Ok(()) => {
                delivered += 1;
                true
            }
            Err(_) => false,
        });
        if lanes.is_empty() {
            registry.remove(&channel);
        }
    }
    integer(delivered)
}

/// The entry for this key when it is present and unexpired; an expired key is
/// removed on access, exactly as a real server reports it gone.
fn live_entry<'a>(
    map: &'a mut HashMap<Vec<u8>, StoredValue>,
    key: &[u8],
) -> Option<&'a mut StoredValue> {
    let present = match map.get(key) {
        Some(entry) => !expired(entry),
        None => false,
    };
    if !present {
        map.remove(key);
        return None;
    }
    map.get_mut(key)
}

fn expired(entry: &StoredValue) -> bool {
    entry
        .expires_at
        .map(|at| Instant::now() >= at)
        .unwrap_or(false)
}

fn bulk(value: &[u8]) -> Vec<u8> {
    let mut reply = format!("${}\r\n", value.len()).into_bytes();
    reply.extend_from_slice(value);
    reply.extend_from_slice(b"\r\n");
    reply
}

fn null_bulk() -> Vec<u8> {
    b"$-1\r\n".to_vec()
}

fn integer(value: i64) -> Vec<u8> {
    format!(":{value}\r\n").into_bytes()
}

fn wrong_type() -> Vec<u8> {
    b"-WRONGTYPE Operation against a key holding the wrong kind of value\r\n".to_vec()
}

/// `["subscribe"|"unsubscribe", channel, count]` - the confirmation a client
/// waits for before it considers the subscription live.
fn subscription_confirmation(kind: &[u8], channel: &[u8], count: i64) -> Vec<u8> {
    let mut frame = b"*3\r\n".to_vec();
    frame.extend(bulk(kind));
    frame.extend(bulk(channel));
    frame.extend(integer(count));
    frame
}

/// `["message", channel, payload]` - the out-of-band push frame.
fn message_frame(channel: &[u8], payload: &[u8]) -> Vec<u8> {
    let mut frame = b"*3\r\n".to_vec();
    frame.extend(bulk(b"message"));
    frame.extend(bulk(channel));
    frame.extend(bulk(payload));
    frame
}

/// A TCP relay in front of the double (or any server) whose links a test can
/// sever on demand — the wire shape of a **server bounce** (every connection
/// dropped, the port back at once) or a **full outage** (reconnects refused).
/// Shared by the crates that prove their bounce-recovery behaviour: the Redis
/// foundation's lifecycle-aware retry and the sync-over-async store's
/// idempotent-only retry both drive their client through it.
pub struct BounceProxy {
    port: u16,
    links: Arc<Mutex<Vec<JoinHandle<()>>>>,
    stop_accepting: watch::Sender<bool>,
    acceptor: Mutex<Option<JoinHandle<()>>>,
}

impl BounceProxy {
    /// Bind an ephemeral port and relay every accepted connection to
    /// `127.0.0.1:target_port`.
    pub async fn start(target_port: u16) -> BounceProxy {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("proxy bind");
        let port = listener.local_addr().expect("proxy addr").port();
        let links: Arc<Mutex<Vec<JoinHandle<()>>>> = Arc::new(Mutex::new(Vec::new()));
        let (stop_accepting, mut stopped) = watch::channel(false);
        let live = links.clone();
        let acceptor = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = stopped.changed() => return, // refuse(): drop the listener
                    accepted = listener.accept() => {
                        let Ok((inbound, _)) = accepted else { return };
                        let Ok(outbound) = TcpStream::connect(("127.0.0.1", target_port)).await
                        else {
                            continue;
                        };
                        let link = tokio::spawn(relay(inbound, outbound));
                        live.lock().expect("links").push(link);
                    }
                }
            }
        });
        BounceProxy {
            port,
            links,
            stop_accepting,
            acceptor: Mutex::new(Some(acceptor)),
        }
    }

    /// The port a client connects to.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Sever every live link — a server bounce (back immediately). Returns only
    /// once every relay task has been torn down, i.e. once both ends of every
    /// link are closed: `abort()` alone merely schedules the cancellation, and a
    /// command written before the runtime polls the aborted task would still
    /// travel the old link and be answered (a flake seen on a starved CI runner).
    pub async fn bounce(&self) {
        let links: Vec<JoinHandle<()>> = self.links.lock().expect("links").drain(..).collect();
        for link in links {
            link.abort();
            // an aborted task resolves (cancelled) once it has been dropped - its
            // sockets with it; a task that already finished resolves at once
            let _ = link.await;
        }
    }

    /// Sever the links AND refuse reconnects — a full outage. The listener is
    /// gone when this returns: the acceptor is told to stop and awaited first,
    /// so no link can be added behind the severing, then every link is severed.
    pub async fn refuse(&self) {
        let _ = self.stop_accepting.send(true);
        let acceptor = self.acceptor.lock().expect("acceptor").take();
        if let Some(acceptor) = acceptor {
            let _ = acceptor.await;
        }
        self.bounce().await;
    }
}

async fn relay(inbound: TcpStream, outbound: TcpStream) {
    let (mut client_read, mut client_write) = inbound.into_split();
    let (mut server_read, mut server_write) = outbound.into_split();
    let up = async {
        let mut buf = [0u8; 4096];
        loop {
            match client_read.read(&mut buf).await {
                Ok(0) | Err(_) => return,
                Ok(n) => {
                    if server_write.write_all(&buf[..n]).await.is_err() {
                        return;
                    }
                }
            }
        }
    };
    let down = async {
        let mut buf = [0u8; 4096];
        loop {
            match server_read.read(&mut buf).await {
                Ok(0) | Err(_) => return,
                Ok(n) => {
                    if client_write.write_all(&buf[..n]).await.is_err() {
                        return;
                    }
                }
            }
        }
    };
    tokio::join!(up, down);
}
