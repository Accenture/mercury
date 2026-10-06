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

//! The graph packager: a command line over the engine's canonical packager
//! (ADR-0026) and the deployment gate's checks, for build pipelines that
//! deliver graph sets (RFC-0005). It never starts the platform. Rust twin of
//! the Java `helpers/graph-packager` (`com.accenture.minigraph.packager.GraphPackager`):
//! the same commands, messages and exit codes, and the same bytes for the same
//! graphs.
//!
//! ```text
//! graph-packager pack    --set <name> [--manifest key=value]... [--out <dir>] <graph.json>... | <folder>
//! graph-packager pack    --set <name> --from-manifest <graphs.yaml> [--manifest key=value]... [--out <dir>]
//! graph-packager unpack  <file.pack> --out <dir>
//! graph-packager inspect <file.pack> [--json]
//! ```
//!
//! A graph is read with the engine's JSON reader, so what is packed is what an
//! application would have loaded, and the gate's checks run before anything is
//! written: a set holding a graph that the gate would reject at startup is
//! refused here, with every reason. Exit codes: 0 success, 1 a refused input
//! (a rule the set breaks, or a usage error), 2 an I/O or format error (a file
//! that cannot be read or written, a graph file that is not a JSON object, a
//! package that fails the strict read).

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use knowledge_graph::graph_set::{self, GraphSetError};
use knowledge_graph::model_gate;
use platform_core::{ConfigReader, ConfigValue};
use rmpv::Value;
use sha2::{Digest, Sha256};

/// Exit code: success.
pub const OK: i32 = 0;
/// Exit code: a refused input - a rule the set breaks, or a usage error.
pub const REFUSED: i32 = 1;
/// Exit code: an I/O or format error.
pub const FAILED: i32 = 2;

const SET: &str = "--set";
const MANIFEST: &str = "--manifest";
const OUT: &str = "--out";
const FROM_MANIFEST: &str = "--from-manifest";
const JSON: &str = "--json";
const JSON_EXT: &str = ".json";
const FILE: &str = "file:";
const CLASSPATH: &str = "classpath:";
const DEFAULT_LOCATION: &str = "classpath:/graph";
const NODES: &str = "nodes";
const CONNECTIONS: &str = "connections";
const USAGE: &str = "Usage:
  graph-packager pack    --set <name> [--manifest key=value]... [--out <dir>] <graph.json>... | <folder>
  graph-packager pack    --set <name> --from-manifest <graphs.yaml> [--manifest key=value]... [--out <dir>]
  graph-packager unpack  <file.pack> --out <dir>
  graph-packager inspect <file.pack> [--json]

Exit codes: 0 success, 1 a refused input, 2 an I/O or format error";

/// Why a command stops, mapped to its message and exit code in [`run`].
enum Failure {
    Usage(String),
    Refused(Vec<String>),
    Rejected(String),
    NoSuchFile(String),
    Io(String),
}

impl From<GraphSetError> for Failure {
    fn from(e: GraphSetError) -> Self {
        match e {
            GraphSetError::Refused(reasons) => Failure::Refused(reasons),
            GraphSetError::Format(message) => Failure::Io(message),
        }
    }
}

fn io_failure(path: &Path, e: io::Error) -> Failure {
    if e.kind() == io::ErrorKind::NotFound {
        Failure::NoSuchFile(path.display().to_string())
    } else {
        Failure::Io(format!("{} - {e}", path.display()))
    }
}

/// Run one command and return its exit code.
pub fn run(args: &[String], out: &mut dyn Write, err: &mut dyn Write) -> i32 {
    // a write to a closed standard stream has nowhere left to report itself
    match dispatch(args, out) {
        Ok(code) => code,
        Err(Failure::Usage(message)) => {
            let _ = writeln!(err, "{message}");
            let _ = writeln!(err, "{USAGE}");
            REFUSED
        }
        Err(Failure::Refused(reasons)) => {
            let _ = writeln!(err, "Refused:");
            for reason in reasons {
                let _ = writeln!(err, "  {reason}");
            }
            REFUSED
        }
        Err(Failure::Rejected(message)) => {
            let _ = writeln!(err, "Refused: {message}");
            REFUSED
        }
        Err(Failure::NoSuchFile(path)) => {
            let _ = writeln!(err, "Error: no such file - {path}");
            FAILED
        }
        Err(Failure::Io(message)) => {
            let _ = writeln!(err, "Error: {message}");
            FAILED
        }
    }
}

fn dispatch(args: &[String], out: &mut dyn Write) -> Result<i32, Failure> {
    let Some(command) = args.first() else {
        return Err(Failure::Usage("Name a command".to_string()));
    };
    let rest = &args[1..];
    match command.as_str() {
        "pack" => pack(rest, out),
        "unpack" => unpack(rest, out),
        "inspect" => inspect(rest, out),
        "help" | "--help" | "-h" => {
            let _ = writeln!(out, "{USAGE}");
            Ok(OK)
        }
        other => Err(Failure::Usage(format!("Unknown command '{other}'"))),
    }
}

fn pack(args: &[String], out: &mut dyn Write) -> Result<i32, Failure> {
    let a = parse(args, &[SET, MANIFEST, OUT, FROM_MANIFEST], &[MANIFEST], &[])?;
    let set_name = a
        .single(SET)
        .ok_or_else(|| Failure::Usage("pack needs --set <name>".to_string()))?;
    let fields = manifest_fields(a.manifest_values())?;
    let files = match a.single(FROM_MANIFEST) {
        Some(manifest) => {
            if !a.positional.is_empty() {
                return Err(Failure::Usage(
                    "Give the graphs or --from-manifest, not both".to_string(),
                ));
            }
            files_listed_in(Path::new(manifest))?
        }
        None => {
            if a.positional.is_empty() {
                return Err(Failure::Usage(
                    "Name the graph files or a folder to pack".to_string(),
                ));
            }
            files_named(&a.positional)?
        }
    };
    let mut graphs = BTreeMap::new();
    for (id, file) in &files {
        graphs.insert(id.clone(), read_graph(file)?);
    }
    // every rule is checked before a path is built from the set name
    let bytes = graph_set::pack(set_name, &fields, &graphs)?;
    let dir = PathBuf::from(a.single(OUT).unwrap_or("."));
    fs::create_dir_all(&dir).map_err(|e| io_failure(&dir, e))?;
    let target = dir.join(format!("{set_name}{}", graph_set::EXTENSION));
    fs::write(&target, &bytes).map_err(|e| io_failure(&target, e))?;
    let _ = writeln!(
        out,
        "Packed {} into {} ({} bytes)",
        count(graphs.len(), "graph"),
        target.display(),
        bytes.len()
    );
    let _ = writeln!(out, "SHA-256 {}", sha256(&bytes));
    Ok(OK)
}

fn unpack(args: &[String], out: &mut dyn Write) -> Result<i32, Failure> {
    let a = parse(args, &[OUT], &[], &[])?;
    if a.positional.len() != 1 {
        return Err(Failure::Usage("unpack needs one .pack file".to_string()));
    }
    let dir_name = a
        .single(OUT)
        .ok_or_else(|| Failure::Usage("unpack needs --out <dir>".to_string()))?;
    let file = Path::new(&a.positional[0]);
    // the names are checked before any of them becomes a path
    let contents = graph_set::read(&read_package(file)?)?;
    let dir = PathBuf::from(dir_name);
    fs::create_dir_all(&dir).map_err(|e| io_failure(&dir, e))?;
    for (id, model) in &contents.graphs {
        let target = dir.join(format!("{id}{JSON_EXT}"));
        fs::write(&target, graph_set::to_json(model)).map_err(|e| io_failure(&target, e))?;
    }
    let _ = writeln!(
        out,
        "Unpacked {} from {} into {}",
        count(contents.graphs.len(), "graph"),
        file.display(),
        dir.display()
    );
    for (id, _) in &contents.graphs {
        let _ = writeln!(out, "  {id}{JSON_EXT}");
    }
    Ok(OK)
}

fn inspect(args: &[String], out: &mut dyn Write) -> Result<i32, Failure> {
    let a = parse(args, &[], &[], &[JSON])?;
    if a.positional.len() != 1 {
        return Err(Failure::Usage("inspect needs one .pack file".to_string()));
    }
    let file = Path::new(&a.positional[0]);
    let bytes = read_package(file)?;
    let contents = graph_set::read(&bytes)?;
    let sha = sha256(&bytes);
    if a.json_report() {
        // keys in sorted order, so both engines' packagers print the same report
        let mut manifest = serde_json::Map::new();
        for (k, v) in &contents.manifest {
            manifest.insert(k.clone(), serde_json::Value::from(v.as_str()));
        }
        let graphs: Vec<serde_json::Value> = contents
            .graphs
            .iter()
            .map(|(id, model)| {
                serde_json::json!({
                    "connections": size(model, CONNECTIONS),
                    "id": id,
                    "nodes": size(model, NODES),
                })
            })
            .collect();
        let report = serde_json::json!({
            "file": file.display().to_string(),
            "graphs": graphs,
            "manifest": manifest,
            "sha256": sha,
            "size": bytes.len(),
        });
        let _ = writeln!(
            out,
            "{}",
            serde_json::to_string_pretty(&report).unwrap_or_default()
        );
    } else {
        let _ = writeln!(out, "File      {}", file.display());
        let _ = writeln!(out, "Size      {} bytes", bytes.len());
        let _ = writeln!(out, "SHA-256   {sha}");
        let _ = writeln!(out, "Manifest");
        let key_width = contents
            .manifest
            .iter()
            .map(|(k, _)| k.len())
            .max()
            .unwrap_or(0);
        for (k, v) in &contents.manifest {
            let _ = writeln!(out, "  {k:<key_width$}  {v}");
        }
        let _ = writeln!(out, "Graphs    {}", contents.graphs.len());
        let id_width = contents
            .graphs
            .iter()
            .map(|(id, _)| id.len())
            .max()
            .unwrap_or(0);
        for (id, model) in &contents.graphs {
            let _ = writeln!(
                out,
                "  {id:<id_width$}  {}, {}",
                count(size(model, NODES), "node"),
                count(size(model, CONNECTIONS), "connection")
            );
        }
    }
    Ok(OK)
}

fn files_named(names: &[String]) -> Result<BTreeMap<String, PathBuf>, Failure> {
    let mut files = BTreeMap::new();
    for name in names {
        let path = Path::new(name);
        if path.is_dir() {
            let mut found: Vec<PathBuf> = fs::read_dir(path)
                .map_err(|e| io_failure(path, e))?
                .filter_map(|entry| entry.ok().map(|e| e.path()))
                .filter(|p| p.is_file() && p.to_string_lossy().ends_with(JSON_EXT))
                .collect();
            found.sort();
            for file in found {
                add_graph_file(&mut files, file)?;
            }
        } else if path.is_file() {
            if !name.ends_with(JSON_EXT) {
                return Err(Failure::Rejected(format!(
                    "{name} is not a .json graph file"
                )));
            }
            add_graph_file(&mut files, path.to_path_buf())?;
        } else {
            return Err(Failure::NoSuchFile(name.clone()));
        }
    }
    Ok(files)
}

fn add_graph_file(files: &mut BTreeMap<String, PathBuf>, file: PathBuf) -> Result<(), Failure> {
    let file_name = file
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let id = file_name
        .strip_suffix(JSON_EXT)
        .unwrap_or(&file_name)
        .to_string();
    if let Some(previous) = files.get(&id) {
        return Err(Failure::Rejected(format!(
            "graph id '{id}' is given twice - {} and {}",
            previous.display(),
            file.display()
        )));
    }
    files.insert(id, file);
    Ok(())
}

fn files_listed_in(manifest: &Path) -> Result<BTreeMap<String, PathBuf>, Failure> {
    if !manifest.is_file() {
        return Err(Failure::NoSuchFile(manifest.display().to_string()));
    }
    let absolute = std::path::absolute(manifest).map_err(|e| io_failure(manifest, e))?;
    let reader = ConfigReader::load(&format!("{FILE}{}", absolute.display()))
        .map_err(|e| Failure::Io(format!("Unable to read {} - {e}", manifest.display())))?;
    // the manifest's own location, as the deployment reads it; a classpath location lives inside an
    // application, which a pipeline does not have, so the folder must be on the file system
    let location = reader
        .get_property("location")
        .unwrap_or_else(|| DEFAULT_LOCATION.to_string());
    if location.starts_with(CLASSPATH) {
        return Err(Failure::Rejected(format!(
            "the location of {} is '{location}', inside an application - pass the folder that holds \
             the graphs instead, or a manifest whose location is a file: folder",
            manifest.display()
        )));
    }
    let folder = match location.strip_prefix(FILE) {
        Some(rest) => PathBuf::from(rest),
        None => absolute
            .parent()
            .map(|p| p.join(&location))
            .unwrap_or_else(|| PathBuf::from(&location)),
    };
    let mut files = BTreeMap::new();
    let mut reasons = Vec::new();
    if let Some(ConfigValue::List(list)) = reader.get("graphs") {
        for i in 0..list.len() {
            let id = reader
                .get_property(&format!("graphs[{i}]"))
                .unwrap_or_default();
            // the id is checked before it becomes a path
            if !model_gate::is_valid_graph_id(&id) {
                reasons.push(format!(
                    "graph id '{id}' in {} - use letters, digits, '_' and '-' only",
                    manifest.display()
                ));
            } else if files
                .insert(id.clone(), folder.join(format!("{id}{JSON_EXT}")))
                .is_some()
            {
                reasons.push(format!(
                    "graph id '{id}' is listed twice in {}",
                    manifest.display()
                ));
            }
        }
    }
    if !reasons.is_empty() {
        return Err(Failure::Refused(reasons));
    }
    for file in files.values() {
        if !file.is_file() {
            return Err(Failure::NoSuchFile(file.display().to_string()));
        }
    }
    Ok(files)
}

fn read_graph(file: &Path) -> Result<Value, Failure> {
    let text = fs::read_to_string(file).map_err(|e| io_failure(file, e))?;
    let json: serde_json::Value = serde_json::from_str(&text).map_err(|e| {
        Failure::Io(format!(
            "{} is not a JSON graph model - {e}",
            file.display()
        ))
    })?;
    if !json.is_object() {
        return Err(Failure::Io(format!(
            "{} is not a JSON object",
            file.display()
        )));
    }
    Ok(event_script::conversions::from_json(&json))
}

fn read_package(file: &Path) -> Result<Vec<u8>, Failure> {
    if !file.is_file() {
        return Err(Failure::NoSuchFile(file.display().to_string()));
    }
    fs::read(file).map_err(|e| io_failure(file, e))
}

fn manifest_fields(pairs: &[String]) -> Result<Vec<(String, String)>, Failure> {
    let mut fields: Vec<(String, String)> = Vec::new();
    for pair in pairs {
        let eq = match pair.find('=') {
            Some(eq) if eq > 0 => eq,
            _ => {
                return Err(Failure::Usage(format!(
                    "--manifest takes key=value, not '{pair}'"
                )))
            }
        };
        let key = &pair[..eq];
        if fields.iter().any(|(k, _)| k == key) {
            return Err(Failure::Usage(format!(
                "The manifest field '{key}' is given twice"
            )));
        }
        fields.push((key.to_string(), pair[eq + 1..].to_string()));
    }
    Ok(fields)
}

fn size(model: &Value, key: &str) -> usize {
    match model {
        Value::Map(entries) => entries
            .iter()
            .find(|(k, _)| k.as_str() == Some(key))
            .and_then(|(_, v)| v.as_array().map(Vec::len))
            .unwrap_or(0),
        _ => 0,
    }
}

fn count(n: usize, noun: &str) -> String {
    format!("{n} {noun}{}", if n == 1 { "" } else { "s" })
}

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

struct Arguments {
    options: HashMap<String, Vec<String>>,
    positional: Vec<String>,
}

impl Arguments {
    fn single(&self, name: &str) -> Option<&str> {
        self.options
            .get(name)
            .and_then(|values| values.first())
            .map(String::as_str)
    }

    fn manifest_values(&self) -> &[String] {
        self.options.get(MANIFEST).map(Vec::as_slice).unwrap_or(&[])
    }

    fn json_report(&self) -> bool {
        self.options.contains_key(JSON)
    }
}

fn parse(
    args: &[String],
    valued: &[&str],
    repeatable: &[&str],
    flags: &[&str],
) -> Result<Arguments, Failure> {
    let mut options: HashMap<String, Vec<String>> = HashMap::new();
    let mut positional = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let arg = args[i].as_str();
        if !arg.starts_with("--") {
            positional.push(arg.to_string());
        } else if flags.contains(&arg) {
            if options.insert(arg.to_string(), Vec::new()).is_some() {
                return Err(Failure::Usage(format!("Give {arg} once")));
            }
        } else if valued.contains(&arg) {
            if i + 1 >= args.len() {
                return Err(Failure::Usage(format!("{arg} needs a value")));
            }
            let values = options.entry(arg.to_string()).or_default();
            if !values.is_empty() && !repeatable.contains(&arg) {
                return Err(Failure::Usage(format!("Give {arg} once")));
            }
            i += 1;
            values.push(args[i].clone());
        } else {
            return Err(Failure::Usage(format!("Unknown option '{arg}'")));
        }
        i += 1;
    }
    Ok(Arguments {
        options,
        positional,
    })
}

/// A command line keeps standard output for its results (pipelines read the
/// JSON report of inspect), so the engine's log goes to standard error,
/// warnings and errors only; `RUST_LOG` overrides the level.
struct StderrLogger;

impl log::Log for StderrLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            eprintln!("{:<5} {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: StderrLogger = StderrLogger;

/// Install the standard-error logger (warnings and errors unless `RUST_LOG`
/// names another level). Called once by the binary; tests leave it out.
pub fn init_logging() {
    let level = match std::env::var("RUST_LOG")
        .unwrap_or_default()
        .to_ascii_lowercase()
        .as_str()
    {
        "error" => log::LevelFilter::Error,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        "off" => log::LevelFilter::Off,
        _ => log::LevelFilter::Warn,
    };
    if log::set_logger(&LOGGER).is_ok() {
        log::set_max_level(level);
    }
}
