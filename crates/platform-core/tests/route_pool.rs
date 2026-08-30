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

//! Behavior of the route pool registration API (`register_route_pool` /
//! `release_route_pool`) — the twin of the Java engine's `RoutePoolTest`:
//! ordered private singleton members, symmetric release, house reload
//! semantics on re-registration, and tolerance of individual updates to pool
//! members. Design record: the Java repo's
//! draft-design-specs/register-route-pool.md. Parity note: Java's
//! null-lambda rejection has no Rust analog — the type system makes a missing
//! function unrepresentable.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use platform_core::{AppError, ComposableFunction, EventEnvelope, Platform, PostOffice};

struct Echo;

#[async_trait]
impl ComposableFunction for Echo {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        Ok(EventEnvelope::new().set_raw_body(input.body().clone()))
    }
}

fn echo() -> Arc<dyn ComposableFunction> {
    Arc::new(Echo)
}

#[tokio::test]
async fn registers_ordered_private_singleton_members() {
    let platform = Platform::new();
    let members = platform
        .register_route_pool("unit.test.pool.a", echo(), 3)
        .expect("pool registration");
    assert_eq!(
        vec![
            "unit.test.pool.a.0".to_string(),
            "unit.test.pool.a.1".to_string(),
            "unit.test.pool.a.2".to_string()
        ],
        members
    );
    for member in &members {
        assert!(platform.has_route(member), "{member} must be registered");
        assert_eq!(
            Some(true),
            platform.is_private(member),
            "{member} must be private"
        );
        assert_eq!(
            Some(1),
            platform.instances(member),
            "{member} must be a singleton lane"
        );
    }
    // a lane is a normal function - prove liveness with one RPC
    let po = PostOffice::new(&platform);
    let response = po
        .request(
            EventEnvelope::new()
                .set_to(&members[0])
                .set_body("hello")
                .expect("serializable body"),
            Duration::from_secs(5),
        )
        .await
        .expect("lane RPC");
    assert_eq!(Some("hello"), response.body().as_str());
    assert!(platform.release_route_pool("unit.test.pool.a"));
    for member in &members {
        assert!(
            !platform.has_route(member),
            "{member} must be gone after pool release"
        );
    }
}

#[tokio::test]
async fn release_is_symmetric_and_absent_pool_returns_false() {
    let platform = Platform::new();
    assert!(!platform.release_route_pool("unit.test.no.such.pool"));
    platform
        .register_route_pool("unit.test.pool.b", echo(), 2)
        .expect("pool registration");
    assert!(platform.release_route_pool("unit.test.pool.b"));
    assert!(!platform.release_route_pool("unit.test.pool.b"));
    assert!(!platform.has_route("unit.test.pool.b.0"));
    assert!(!platform.has_route("unit.test.pool.b.1"));
}

#[tokio::test]
async fn re_registration_reloads_to_exactly_the_new_set() {
    let platform = Platform::new();
    platform
        .register_route_pool("unit.test.pool.c", echo(), 5)
        .expect("pool registration");
    let members = platform
        .register_route_pool("unit.test.pool.c", echo(), 3)
        .expect("pool reload");
    assert_eq!(3, members.len());
    assert!(platform.has_route("unit.test.pool.c.2"));
    // the reload must not leave orphans beyond the new count
    assert!(!platform.has_route("unit.test.pool.c.3"));
    assert!(!platform.has_route("unit.test.pool.c.4"));
    assert!(platform.release_route_pool("unit.test.pool.c"));
}

#[tokio::test]
async fn invalid_arguments_are_rejected() {
    let platform = Platform::new();
    assert!(platform
        .register_route_pool("unit.test.pool.d", echo(), 0)
        .is_err());
    // member names must validate: "{prefix}.{n}" through the standard rules
    assert!(platform
        .register_route_pool("Unit.Test.Pool", echo(), 1)
        .is_err());
    assert!(platform
        .register_route_pool("unit.test.pool.d.", echo(), 1)
        .is_err());
    assert!(!platform.has_route("unit.test.pool.d.0"));
}

#[tokio::test]
async fn individual_updates_to_members_are_tolerated_and_cleaned_up() {
    let platform = Platform::new();
    platform
        .register_route_pool("unit.test.pool.e", echo(), 3)
        .expect("pool registration");
    // an individual release of a member is warned, never refused (house semantics)
    assert!(platform.release("unit.test.pool.e.1"));
    assert!(!platform.has_route("unit.test.pool.e.1"));
    // an individual re-registration over a member reloads it, also warned
    platform
        .register_private("unit.test.pool.e.2", echo(), 1)
        .expect("member reload");
    assert!(platform.has_route("unit.test.pool.e.2"));
    // pool release still cleans the remainder, holes included
    assert!(platform.release_route_pool("unit.test.pool.e"));
    assert!(!platform.has_route("unit.test.pool.e.0"));
    assert!(!platform.has_route("unit.test.pool.e.2"));
}

#[tokio::test]
async fn neighbor_routes_outside_the_pool_range_are_untouched() {
    let platform = Platform::new();
    platform
        .register_private("unit.test.pool.f.10", echo(), 1)
        .expect("neighbor registration");
    platform
        .register_route_pool("unit.test.pool.f", echo(), 3)
        .expect("pool registration");
    assert!(platform.release_route_pool("unit.test.pool.f"));
    // "{prefix}.10" is outside the pool's range [0, 3) and is not a member
    assert!(platform.has_route("unit.test.pool.f.10"));
    assert!(platform.release("unit.test.pool.f.10"));
}
