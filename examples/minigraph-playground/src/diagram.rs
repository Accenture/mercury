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

//! The diagram-assistant graph's tool, `diagram.render` - the twin of the Java example's
//! `DiagramRender`: a function of this application in front of `mermaid.render`, a function of the
//! MCP binding of a language pack, bound in the binding's code to the tool of a Mermaid MCP server
//! (a route names the capability, never the transport).
//!
//! The server answers the verdict together with the rendered image, and an agent loop returns a
//! tool's whole reply to the model. This function keeps the payload out of band: it calls the
//! binding, stores the SVG under a reference id in the folder `diagram.store.folder` (default
//! `/tmp/rendered_diagrams`, the file named `<reference>.svg`) beside an HTML page,
//! `<reference>.html`, that inlines the SVG so a browser renders it from the file system - the
//! server's SVG starts with a `<style>` element and uses `foreignObject` labels, which an `<img>`
//! tag may refuse - and returns the verdict, the code, the reference and the image's size, never
//! the image and never a path: the model cannot open one, and the folder is configuration. An
//! invalid diagram stores nothing and returns the server's error for the model to fix.
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

use async_trait::async_trait;
use platform_core::{
    preload, AppConfigReader, AppError, ComposableFunction, EventEnvelope, Platform, PostOffice,
};
use rmpv::Value;

use crate::order_book::{field, map};

/// The route the diagram-assistant graph's catalog row `render_diagram` maps to.
pub const DIAGRAM_RENDER_ROUTE: &str = "diagram.render";
/// The pack's MCP-bound function this one calls.
pub const MCP_ROUTE: &str = "mermaid.render";
/// The configuration key of the store folder.
pub const FOLDER_KEY: &str = "diagram.store.folder";
/// The store folder when the key is not set.
pub const DEFAULT_FOLDER: &str = "/tmp/rendered_diagrams";

const TIMEOUT: Duration = Duration::from_millis(25_000);
const DIAGRAM_CODE: &str = "diagramCode";
const RAW_SVG: &str = "rawSVG";
const VALID: &str = "valid";
const TITLE: &str = "title";
const ERROR: &str = "error";

/// `diagram.render`: validate and render a Mermaid diagram through the pack's binding, keep the
/// image out of band and answer with the verdict, the code and a reference.
#[preload(route = "diagram.render", instances = 10)]
struct DiagramRender;

#[async_trait]
impl ComposableFunction for DiagramRender {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body = input.body();
        let code = match field(body, DIAGRAM_CODE).and_then(Value::as_str) {
            Some(text) if !text.trim().is_empty() => text.to_string(),
            _ => return Err(AppError::new(400, format!("{DIAGRAM_CODE} is required"))),
        };
        let platform = Platform::get_instance();
        // the trace-aware PostOffice carries the loop's trace into the binding's function
        let reply = PostOffice::new(&platform)
            .request(
                EventEnvelope::new()
                    .set_to(MCP_ROUTE)
                    .set_raw_body(body.clone()),
                TIMEOUT,
            )
            .await?;
        if reply.has_error() {
            let message = reply
                .body()
                .as_str()
                .map(str::to_string)
                .unwrap_or_else(|| format!("{MCP_ROUTE} failed"));
            return Err(AppError::new(reply.status(), message));
        }
        let rendered = reply.body();
        if !matches!(rendered, Value::Map(_)) {
            return Err(AppError::new(
                502,
                format!("{MCP_ROUTE} answered outside the tool contract"),
            ));
        }
        let valid = field(rendered, VALID)
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let mut result: Vec<(&str, Value)> = vec![
            (VALID, Value::Boolean(valid)),
            (
                DIAGRAM_CODE,
                field(rendered, DIAGRAM_CODE)
                    .filter(|value| !value.is_nil())
                    .cloned()
                    .unwrap_or_else(|| Value::from(code.as_str())),
            ),
        ];
        for key in [TITLE, ERROR] {
            if let Some(value) = field(rendered, key).filter(|value| !value.is_nil()) {
                result.push((key, value.clone()));
            }
        }
        if valid {
            if let Some(svg) = field(rendered, RAW_SVG)
                .and_then(Value::as_str)
                .filter(|svg| !svg.trim().is_empty())
            {
                let title = field(rendered, TITLE)
                    .and_then(Value::as_str)
                    .filter(|title| !title.trim().is_empty());
                let (reference, bytes) = store(title, &code, svg)?;
                result.push(("reference", Value::from(reference)));
                result.push(("bytes", Value::from(bytes)));
            }
        }
        Ok(EventEnvelope::new().set_raw_body(map(result)))
    }
}

/// Keep the image out of band: the SVG under a reference id in the store folder, beside the HTML
/// page that inlines it; the caller gets the reference and the image's size, never a path.
fn store(title: Option<&str>, code: &str, svg: &str) -> Result<(String, u64), AppError> {
    let reference = uuid::Uuid::new_v4().simple().to_string();
    let folder =
        PathBuf::from(AppConfigReader::get_instance().get_property_or(FOLDER_KEY, DEFAULT_FOLDER));
    std::fs::create_dir_all(&folder)
        .map_err(|e| AppError::new(500, format!("Unable to create {}: {e}", folder.display())))?;
    let file = folder.join(format!("{reference}.svg"));
    std::fs::write(&file, svg)
        .map_err(|e| AppError::new(500, format!("Unable to write {}: {e}", file.display())))?;
    let page_file = folder.join(format!("{reference}.html"));
    let page_title = title.unwrap_or(reference.as_str());
    std::fs::write(&page_file, page(page_title, &reference, code, svg))
        .map_err(|e| AppError::new(500, format!("Unable to write {}: {e}", page_file.display())))?;
    Ok((reference, svg.len() as u64))
}

/// The HTML page beside the SVG: the title, the reference, the Mermaid code and the inlined image.
pub fn page(title: &str, reference: &str, code: &str, svg: &str) -> String {
    format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<title>{title}</title>\n\
         <style>\n\
         \x20 body {{ font-family: -apple-system, Helvetica, Arial, sans-serif; margin: 2rem; color: #222; background: #fafafa; }}\n\
         \x20 h1 {{ font-size: 1.3rem; }}\n\
         \x20 .meta {{ color: #666; font-size: .85rem; font-family: Menlo, monospace; }}\n\
         \x20 .diagram {{ background: #fff; border: 1px solid #ddd; border-radius: 8px; padding: 1rem; max-width: 960px; overflow: auto; }}\n\
         \x20 .diagram svg {{ width: 100%; height: auto; }}\n\
         \x20 pre {{ background: #f3f3f3; border-radius: 6px; padding: .8rem 1rem; max-width: 960px; overflow: auto; font-size: .85rem; }}\n\
         </style>\n</head>\n<body>\n\
         <h1>{title}</h1>\n\
         <p class=\"meta\">reference {reference} &middot; <a href=\"{reference}.svg\">{reference}.svg</a>\
         \x20&middot; rendered by the diagram assistant through its MCP-bound tool</p>\n\
         <div class=\"diagram\">{svg}</div>\n\
         <h2>Mermaid code</h2>\n<pre>{code}</pre>\n\
         </body>\n</html>\n",
        title = escape(title),
        code = escape(code),
    )
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
