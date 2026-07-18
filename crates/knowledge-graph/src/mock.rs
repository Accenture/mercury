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

//! Dev-only mock services — the Rust port of the Java
//! `com.accenture.minigraph.mock` package (`MdmProfile`, `AccountDetails`,
//! `HelloTask`). Each is gated by `#[optional_service("app.env=dev")]` (the
//! Java `@OptionalService` annotation), so it registers **only** when
//! `app.env=dev` — the data-dictionary / API-fetcher tutorials call these over
//! HTTP as stand-in enterprise services. Profile/account fixtures ship in the
//! engine crate's `resources/mock/`.

use std::collections::HashMap;

use async_trait::async_trait;
use platform_core::{preload, AppError, ComposableFunction, EventEnvelope};
use serde_json::Value as JsonValue;

/// Java `MdmProfile` (`mock.mdm.profile`): serves a person profile from
/// `resources/mock/profile-{id}.json`; person id from the POST body or the GET
/// path parameter.
#[preload(route = "mock.mdm.profile", instances = 50)]
#[optional_service("app.env=dev")]
pub struct MdmProfile;

#[async_trait]
impl ComposableFunction for MdmProfile {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request: JsonValue = input.body_as()?;
        if request["headers"]["x-exception"] == "true" {
            return Err(AppError::new(401, "simulated exception"));
        }
        let person_id = if request["method"] == "POST" {
            request["body"]["person_id"]
                .as_i64()
                .map(|n| n.to_string())
                .or_else(|| request["body"]["person_id"].as_str().map(str::to_string))
        } else {
            request["parameters"]["path"]["id"]
                .as_str()
                .map(str::to_string)
        };
        let Some(person_id) = person_id else {
            return Err(AppError::new(400, "Missing person id"));
        };
        serve_mock_json(
            &format!("profile-{person_id}"),
            &format!("Profile {person_id} not found"),
        )
    }
}

/// Java `AccountDetails` (`mock.account.details`): serves an account from
/// `resources/mock/account-{id}.json`; account id from the POST body.
#[preload(route = "mock.account.details", instances = 50)]
#[optional_service("app.env=dev")]
pub struct AccountDetails;

#[async_trait]
impl ComposableFunction for AccountDetails {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request: JsonValue = input.body_as()?;
        log::info!(
            "Got request parameters {} and Authorization='{}'",
            request["body"],
            request["headers"]["authorization"]
                .as_str()
                .unwrap_or_default()
        );
        let account_id = request["body"]["account_id"].as_str().map(str::to_string);
        let (Some(account_id), true) = (account_id, request["method"] == "POST") else {
            return Err(AppError::new(400, "Missing account id"));
        };
        serve_mock_json(
            &format!("account-{account_id}"),
            &format!("Account {account_id} not found"),
        )
    }
}

/// Java `HelloTask` (`v1.hello.task`): the tutorial-13 demo task invoked through
/// the `graph.task` skill.
#[preload(route = "v1.hello.task", instances = 50)]
#[optional_service("app.env=dev")]
pub struct HelloTask;

#[async_trait]
impl ComposableFunction for HelloTask {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: JsonValue = input.body_as().unwrap_or(JsonValue::Null);
        let name = body
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("stranger");
        let mut result = serde_json::json!({"greeting": format!("Hello, {name}")});
        if let Some(amount) = body.get("amount").and_then(|v| v.as_f64()) {
            result["doubled"] = serde_json::json!(amount * 2.0);
        }
        if let Some(app) = headers.get("x-app") {
            result["app"] = serde_json::json!(app);
        }
        EventEnvelope::new().set_body(result)
    }
}

/// Load a mock fixture (`resources/mock/{name}.json`) into a response body, or
/// a 400 with `not_found` if it is missing.
fn serve_mock_json(name: &str, not_found: &str) -> Result<EventEnvelope, AppError> {
    match platform_core::ConfigReader::load(&format!("classpath:/mock/{name}.json")) {
        Ok(reader) => {
            let json =
                platform_core::ConfigValue::Map(reader.get_map().clone().into_map()).to_json();
            Ok(EventEnvelope::new().set_raw_body(event_script::conversions::from_json(&json)))
        }
        Err(_) => Err(AppError::new(400, not_found)),
    }
}
