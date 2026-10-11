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

//! The API playground - Swagger UI in a Mercury-branded page, the twin of the Java
//! `extensions/api-playground`. REST automation serves the page as static content from
//! `resources/public` and the three endpoints of `rest.yaml` on one port:
//!
//! - `GET /api/specs` - the documents this application offers: the files of the specs folder
//!   (`api.playground.specs`) and the bundled example `resources/sample/yaml/demo.yaml`;
//! - `GET /api/specs/{filename}` - one document as `application/yaml` or `application/json`
//!   (a folder file wins over the example of the same name);
//! - `PUT /api/specs/{filename}` - the page's Deploy: write the document into the specs folder;
//!   an existing name answers 409 unless `?overwrite=true`, so the page asks before replacing it.
//!
//! The Swagger UI assets under `resources/public/swagger-ui` are fetched by
//! `scripts/fetch-swagger-ui.py` or `.js` and never committed; the application refuses to start
//! without them, with the instruction in its message.

use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::PathBuf;

use async_trait::async_trait;
use platform_core::automation::AsyncHttpRequest;
use platform_core::{
    main_application, preload, AppConfigReader, AppError, ComposableFunction, EntryPoint,
    EventEnvelope,
};
use rmpv::Value;

/// The property naming the specs folder.
const SPECS_FOLDER: &str = "api.playground.specs";
/// The bundled example and its resource path.
const EXAMPLE: &str = "demo.yaml";
const EXAMPLE_RESOURCE: &str = "sample/yaml/demo.yaml";
/// The Swagger UI asset the page cannot do without - the fetch scripts' marker file.
const SWAGGER_UI_BUNDLE: &str = "public/swagger-ui/swagger-ui-bundle.js";
const CONTENT_TYPE: &str = "Content-Type";

// ---- the documents this application offers ------------------------------------------------

/// The OpenAPI documents by name: the specs folder's files and the bundled example. A folder
/// file wins over the example of the same name. Only YAML and JSON files are offered (`.yaml`,
/// `.yml` or `.json`), and a name must be a plain file name (letters, digits, dot, dash,
/// underscore).
struct Specs {
    folder: Option<PathBuf>,
}

impl Specs {
    /// The specs folder from `api.playground.specs` (None when the property is unset).
    fn from_config() -> Self {
        let folder = AppConfigReader::get_instance()
            .get_property(SPECS_FOLDER)
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .map(PathBuf::from);
        Specs { folder }
    }

    #[cfg(test)]
    fn with_folder(folder: Option<PathBuf>) -> Self {
        Specs { folder }
    }

    /// The document names, sorted: the folder's files and the example.
    fn list(&self) -> Vec<String> {
        let mut names = BTreeSet::new();
        names.insert(EXAMPLE.to_string());
        if let Some(entries) = self.folder.as_ref().and_then(|f| fs::read_dir(f).ok()) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if entry.path().is_file() && offered(&name) {
                    names.insert(name);
                }
            }
        }
        names.into_iter().collect()
    }

    /// The document's text, or None when no such document is offered.
    fn read(&self, filename: &str) -> Option<String> {
        if !offered(filename) {
            return None;
        }
        if let Some(folder) = &self.folder {
            let file = folder.join(filename);
            if file.is_file() {
                return fs::read_to_string(file).ok();
            }
        }
        if filename == EXAMPLE {
            return platform_core::resources::resolve_classpath(EXAMPLE_RESOURCE)
                .and_then(|file| fs::read_to_string(file).ok());
        }
        None
    }

    /// Write a document into the specs folder. Returns true when an existing file was replaced.
    /// Errors: 400 for a bad name or an empty document, 409 when the file exists and overwrite is
    /// false, 503 when no specs folder is configured or it cannot be created.
    fn write(&self, filename: &str, content: &str, overwrite: bool) -> Result<bool, AppError> {
        if !offered(filename) {
            return Err(AppError::new(
                400,
                "File name must be a plain name ending with .yaml, .yml or .json",
            ));
        }
        if content.trim().is_empty() {
            return Err(AppError::new(
                400,
                "Nothing to deploy - the document is empty",
            ));
        }
        let Some(folder) = &self.folder else {
            return Err(AppError::new(
                503,
                format!("No specs folder - set {SPECS_FOLDER} in application.yml"),
            ));
        };
        if !folder.is_dir() {
            fs::create_dir_all(folder).map_err(|e| {
                AppError::new(
                    503,
                    format!("Cannot create the specs folder {} - {e}", folder.display()),
                )
            })?;
        }
        let target = folder.join(filename);
        let exists = target.is_file();
        if exists && !overwrite {
            return Err(AppError::new(
                409,
                format!("{filename} exists in the specs folder - confirm to overwrite"),
            ));
        }
        fs::write(&target, content)
            .map_err(|e| AppError::new(500, format!("Cannot write {} - {e}", target.display())))?;
        Ok(exists)
    }

    fn folder_path(&self) -> Option<String> {
        self.folder.as_ref().map(|f| f.display().to_string())
    }
}

fn content_type(filename: &str) -> &'static str {
    if filename.to_ascii_lowercase().ends_with(".json") {
        "application/json"
    } else {
        "application/yaml"
    }
}

/// A plain yaml, yml or json file name: letters, digits, dot, dash and underscore, not starting
/// with a dot.
fn offered(name: &str) -> bool {
    let mut chars = name.chars();
    let first_ok = chars.next().is_some_and(|c| c.is_ascii_alphanumeric());
    let rest_ok = chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'));
    let lower = name.to_ascii_lowercase();
    first_ok
        && rest_ok
        && (lower.ends_with(".yaml") || lower.ends_with(".yml") || lower.ends_with(".json"))
}

/// The request body as text: text/plain arrives as a string, other types as bytes or a parsed
/// value (rendered back as its text form).
fn body_text(value: &Value) -> Option<String> {
    match value {
        Value::Nil => None,
        Value::String(s) => s.as_str().map(str::to_string),
        Value::Binary(b) => Some(String::from_utf8_lossy(b).to_string()),
        other => Some(other.to_string()),
    }
}

/// The current time as an ISO-8601 UTC timestamp (the Java `new Date()` rendering), without a
/// date-time dependency: civil-from-days per Howard Hinnant's algorithm.
fn now_iso8601() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs() as i64;
    let millis = now.subsec_millis();
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);
    let (h, m, s) = (rem / 3600, (rem % 3600) / 60, rem % 60);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let mo = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if mo <= 2 { y + 1 } else { y };
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{m:02}:{s:02}.{millis:03}Z")
}

// ---- the three endpoints -----------------------------------------------------------------

/// `GET /api/specs` - the Java `CountSwaggerFiles`.
#[preload(route = "v1.list.swagger.files", instances = 10)]
struct ListSpecs;

#[async_trait]
impl ComposableFunction for ListSpecs {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let list = Specs::from_config().list();
        EventEnvelope::new().set_body(serde_json::json!({
            "total": list.len(),
            "list": list,
            "time": now_iso8601(),
        }))
    }
}

/// `GET /api/specs/{filename}` - the Java `ViewSwaggerFile`.
#[preload(route = "v1.view.swagger.file", instances = 10)]
struct ViewSpec;

#[async_trait]
impl ComposableFunction for ViewSpec {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request = AsyncHttpRequest::from_value(input.body());
        let filename = request
            .path_parameter("filename")
            .ok_or_else(|| AppError::new(400, "Missing filename in path parameter"))?;
        let content = Specs::from_config()
            .read(filename)
            .ok_or_else(|| AppError::new(404, "File not found"))?;
        Ok(EventEnvelope::new()
            .set_header(CONTENT_TYPE, content_type(filename))
            .set_raw_body(Value::from(content.as_str())))
    }
}

/// `PUT /api/specs/{filename}` - the Java `DeploySwaggerFile`.
#[preload(route = "v1.deploy.swagger.file", instances = 10)]
struct DeploySpec;

#[async_trait]
impl ComposableFunction for DeploySpec {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request = AsyncHttpRequest::from_value(input.body());
        let filename = request
            .path_parameter("filename")
            .ok_or_else(|| AppError::new(400, "Missing filename in path parameter"))?;
        let overwrite = request
            .query_parameter("overwrite")
            .is_some_and(|v| v.eq_ignore_ascii_case("true"));
        let specs = Specs::from_config();
        let text = body_text(request.body()).unwrap_or_default();
        let replaced = specs.write(filename, &text, overwrite)?;
        EventEnvelope::new().set_body(serde_json::json!({
            "deployed": filename,
            "overwritten": replaced,
            "folder": specs.folder_path(),
        }))
    }
}

// ---- the application ---------------------------------------------------------------------

#[main_application]
struct MainApp;

#[async_trait]
impl EntryPoint for MainApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        // the Java module fails its build without the assets; this binary refuses to start
        if platform_core::resources::resolve_classpath(SWAGGER_UI_BUNDLE).is_none() {
            return Err(AppError::new(
                500,
                "Swagger UI is not in resources/public/swagger-ui - run \
                 python3 scripts/fetch-swagger-ui.py or node scripts/fetch-swagger-ui.js first \
                 (see README.md)",
            ));
        }
        let config = AppConfigReader::get_instance();
        let port = config.get_property_or("rest.server.port", "8200");
        match Specs::from_config().folder_path() {
            Some(folder) => log::info!("Specs folder {folder}"),
            None => {
                log::warn!("No specs folder - {SPECS_FOLDER} is not set; Deploy is unavailable")
            }
        }
        log::info!("API playground ready - open http://127.0.0.1:{port}/");
        Ok(())
    }
}

platform_core::auto_start_main!();

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_specs(label: &str) -> (Specs, PathBuf) {
        let folder = test_support::temp_path(&format!("api-playground-{label}"));
        (Specs::with_folder(Some(folder.clone())), folder)
    }

    #[test]
    fn the_list_holds_the_example_and_the_folder_files_sorted() {
        let (specs, folder) = temp_specs("list");
        fs::create_dir_all(&folder).unwrap();
        fs::write(folder.join("zeta.json"), "{}").unwrap();
        fs::write(folder.join("alpha.yaml"), "openapi: 3.0.3").unwrap();
        fs::write(folder.join("notes.txt"), "not offered").unwrap();
        fs::write(folder.join(".hidden.yaml"), "not offered").unwrap();
        assert_eq!(specs.list(), vec!["alpha.yaml", "demo.yaml", "zeta.json"]);
        // no folder at all: the example alone
        assert_eq!(Specs::with_folder(None).list(), vec!["demo.yaml"]);
    }

    #[test]
    fn a_folder_file_wins_over_the_example_and_bad_names_are_refused() {
        platform_core::resources::prepend_resource_root(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources"
        ));
        let (specs, folder) = temp_specs("read");
        let example = specs.read("demo.yaml").expect("the bundled example");
        assert!(example.contains("openapi"));
        fs::create_dir_all(&folder).unwrap();
        fs::write(
            folder.join("demo.yaml"),
            "openapi: 3.0.3\ninfo: {title: FOLDER}\n",
        )
        .unwrap();
        assert!(specs.read("demo.yaml").unwrap().contains("FOLDER"));
        assert_eq!(specs.read("missing.yaml"), None);
        assert_eq!(specs.read("../demo.yaml"), None);
        assert_eq!(specs.read("demo.txt"), None);
        assert_eq!(content_type("a.json"), "application/json");
        assert_eq!(content_type("a.YAML"), "application/yaml");
    }

    #[test]
    fn deploy_creates_the_folder_refuses_a_duplicate_and_overwrites_on_request() {
        let (specs, folder) = temp_specs("write");
        assert!(!folder.exists());
        assert!(!specs.write("t.yaml", "openapi: 3.0.3", false).unwrap());
        assert!(folder.join("t.yaml").is_file());
        let dup = specs.write("t.yaml", "openapi: 3.0.3", false).unwrap_err();
        assert_eq!(dup.status(), 409);
        assert!(specs.write("t.yaml", "openapi: 3.0.4", true).unwrap());
        assert_eq!(
            fs::read_to_string(folder.join("t.yaml")).unwrap(),
            "openapi: 3.0.4"
        );
        assert_eq!(
            specs.write("evil.txt", "x", false).unwrap_err().status(),
            400
        );
        assert_eq!(
            specs.write("../x.yaml", "x", false).unwrap_err().status(),
            400
        );
        assert_eq!(
            specs
                .write("empty.yaml", "  \n", false)
                .unwrap_err()
                .status(),
            400
        );
        assert_eq!(
            Specs::with_folder(None)
                .write("t.yaml", "x", false)
                .unwrap_err()
                .status(),
            503
        );
    }

    #[test]
    fn body_text_reads_text_bytes_and_values() {
        assert_eq!(body_text(&Value::Nil), None);
        assert_eq!(body_text(&Value::from("a: 1")).as_deref(), Some("a: 1"));
        assert_eq!(
            body_text(&Value::Binary(b"b: 2".to_vec())).as_deref(),
            Some("b: 2")
        );
        assert!(body_text(&Value::from(7)).is_some());
        let stamp = now_iso8601();
        assert!(stamp.ends_with('Z') && stamp.len() == 24, "{stamp}");
        assert!(stamp.starts_with("20"));
    }
}
