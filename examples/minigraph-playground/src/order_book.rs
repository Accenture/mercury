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

//! The order-assistant demo's data and its three tools - the twins of the Java example's
//! `OrderBook`, `OrderLookup`, `ShipmentStatus` and `RefundIssue`.
//!
//! A tool of the agent loop is an ordinary composable function. The loop calls the route the
//! graph's catalog maps the tool name to, with the model's validated arguments as the request
//! body, and returns the response to the model as the call's result; an error goes back to the
//! model as that call's error, with its status and message, so a 404 here is something the model
//! reads and answers. The book stands in for the order, logistics and payment systems a real
//! assistant would reach through composable functions of its own. The data is fixed so a run is
//! reproducible: ORD-1001 was delivered, ORD-1002 and ORD-1004 were lost by the carrier, and
//! ORD-1003 is still in transit. A refund is the one write, and it is issued at most once per order.
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use async_trait::async_trait;
use platform_core::{preload, AppError, ComposableFunction, EventEnvelope};
use rmpv::Value;

/// The catalog row `order_lookup` maps to this route: an order by its id.
pub const ORDER_LOOKUP_ROUTE: &str = "demo.order.lookup";
/// The catalog row `shipment_status` maps to this route: the shipment of an order.
pub const SHIPMENT_STATUS_ROUTE: &str = "demo.shipment.status";
/// The catalog row `issue_refund` (marked `write: true`) maps to this route: the one write.
pub const REFUND_ISSUE_ROUTE: &str = "demo.refund.issue";

const ORDER_ID: &str = "order_id";
const CURRENCY: &str = "USD";
const STATUS: &str = "status";

/// A MsgPack map from string keys.
pub fn map(entries: Vec<(&str, Value)>) -> Value {
    Value::Map(
        entries
            .into_iter()
            .map(|(key, value)| (Value::from(key), value))
            .collect(),
    )
}

/// One entry of a MsgPack map, by key.
pub fn field<'a>(body: &'a Value, key: &str) -> Option<&'a Value> {
    match body {
        Value::Map(entries) => entries
            .iter()
            .find(|(name, _)| name.as_str() == Some(key))
            .map(|(_, value)| value),
        _ => None,
    }
}

fn item(sku: &str, name: &str, quantity: i64, price: f64) -> Value {
    map(vec![
        ("sku", Value::from(sku)),
        ("name", Value::from(name)),
        ("quantity", Value::from(quantity)),
        ("price", Value::from(price)),
    ])
}

fn order_record(id: &str, placed_on: &str, total: f64, status: &str, items: Vec<Value>) -> Value {
    map(vec![
        (ORDER_ID, Value::from(id)),
        ("placed_on", Value::from(placed_on)),
        ("items", Value::Array(items)),
        ("total", Value::from(total)),
        ("currency", Value::from(CURRENCY)),
        (STATUS, Value::from(status)),
    ])
}

fn shipment_record(
    id: &str,
    carrier: &str,
    tracking: &str,
    status: &str,
    updated_on: &str,
    note: &str,
) -> Value {
    map(vec![
        (ORDER_ID, Value::from(id)),
        ("carrier", Value::from(carrier)),
        ("tracking", Value::from(tracking)),
        (STATUS, Value::from(status)),
        ("updated_on", Value::from(updated_on)),
        ("note", Value::from(note)),
    ])
}

fn orders() -> &'static HashMap<&'static str, Value> {
    static ORDERS: OnceLock<HashMap<&'static str, Value>> = OnceLock::new();
    ORDERS.get_or_init(|| {
        HashMap::from([
            (
                "ORD-1001",
                order_record(
                    "ORD-1001",
                    "2026-11-03",
                    89.00,
                    "delivered",
                    vec![item("HP-200", "Wireless headphones", 1, 89.00)],
                ),
            ),
            (
                "ORD-1002",
                order_record(
                    "ORD-1002",
                    "2026-11-04",
                    174.00,
                    "shipped",
                    vec![
                        item("KB-77", "Mechanical keyboard", 1, 129.00),
                        item("HUB-4C", "USB-C hub", 1, 45.00),
                    ],
                ),
            ),
            (
                "ORD-1003",
                order_record(
                    "ORD-1003",
                    "2026-11-11",
                    118.00,
                    "shipped",
                    vec![item("MS-12", "Monitor stand", 2, 59.00)],
                ),
            ),
            (
                "ORD-1004",
                order_record(
                    "ORD-1004",
                    "2026-11-05",
                    240.00,
                    "shipped",
                    vec![item("DK-9", "Standing desk mat", 2, 120.00)],
                ),
            ),
        ])
    })
}

fn shipments() -> &'static HashMap<&'static str, Value> {
    static SHIPMENTS: OnceLock<HashMap<&'static str, Value>> = OnceLock::new();
    SHIPMENTS.get_or_init(|| {
        HashMap::from([
            (
                "ORD-1001",
                shipment_record(
                    "ORD-1001",
                    "Northwind Express",
                    "NWX-5531-0071",
                    "delivered",
                    "2026-11-08",
                    "Delivered to the front desk and signed for.",
                ),
            ),
            (
                "ORD-1002",
                shipment_record(
                    "ORD-1002",
                    "Northwind Express",
                    "NWX-5531-0188",
                    "lost",
                    "2026-11-10",
                    "The parcel left the sorting hub on 2026-11-06 and has not been scanned since; \
                     the carrier declared it lost.",
                ),
            ),
            (
                "ORD-1003",
                shipment_record(
                    "ORD-1003",
                    "Blue Parcel",
                    "BP-00912-4471",
                    "in_transit",
                    "2026-11-13",
                    "In transit; the estimated delivery date is 2026-11-16.",
                ),
            ),
            (
                "ORD-1004",
                shipment_record(
                    "ORD-1004",
                    "Blue Parcel",
                    "BP-00912-4502",
                    "lost",
                    "2026-11-12",
                    "The carrier closed the trace without finding the parcel.",
                ),
            ),
        ])
    })
}

fn refunds() -> &'static Mutex<HashMap<String, Value>> {
    static REFUNDS: OnceLock<Mutex<HashMap<String, Value>>> = OnceLock::new();
    REFUNDS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// The order id of a tool call, as the catalog's schema requires it: 400 when missing.
pub fn order_id(input: &Value) -> Result<String, AppError> {
    match field(input, ORDER_ID).and_then(Value::as_str) {
        Some(id) if !id.trim().is_empty() => Ok(id.to_string()),
        _ => Err(AppError::new(400, format!("Missing {ORDER_ID}"))),
    }
}

/// An order by its id: the id, the date it was placed, its items, the total, the currency and
/// the status; 404 when the order does not exist.
pub fn order(order_id: &str) -> Result<Value, AppError> {
    orders()
        .get(order_id)
        .cloned()
        .ok_or_else(|| AppError::new(404, format!("Order not found: {order_id}")))
}

/// The shipment of an order: the carrier, the tracking number, the status, the date of the last
/// update and the carrier's note; 404 when the order does not exist.
pub fn shipment(order_id: &str) -> Result<Value, AppError> {
    shipments()
        .get(order_id)
        .cloned()
        .ok_or_else(|| AppError::new(404, format!("No shipment for order {order_id}")))
}

/// Issue a refund for an order, once: 404 when the order does not exist, 400 when the amount is
/// out of range or the reason is missing, 409 when the order was already refunded.
pub fn refund(order_id: &str, amount: f64, reason: Option<&str>) -> Result<Value, AppError> {
    let ordered = order(order_id)?;
    let total = field(&ordered, "total")
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    if amount <= 0.0 || amount > total {
        return Err(AppError::new(
            400,
            format!("The refund must be above 0 and at most the order total of {total} {CURRENCY}"),
        ));
    }
    let reason = match reason {
        Some(text) if !text.trim().is_empty() => text,
        _ => return Err(AppError::new(400, "Missing reason")),
    };
    let reference = uuid::Uuid::new_v4().simple().to_string()[..8].to_uppercase();
    let issued = map(vec![
        ("refund_id", Value::from(format!("RF-{reference}"))),
        (ORDER_ID, Value::from(order_id)),
        ("amount", Value::from(amount)),
        ("currency", Value::from(CURRENCY)),
        ("reason", Value::from(reason)),
        (STATUS, Value::from("issued")),
    ]);
    let mut ledger = refunds().lock().expect("refund ledger");
    if let Some(earlier) = ledger.get(order_id) {
        let earlier_id = field(earlier, "refund_id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        return Err(AppError::new(
            409,
            format!("Order {order_id} was already refunded as {earlier_id}"),
        ));
    }
    ledger.insert(order_id.to_string(), issued.clone());
    Ok(issued)
}

/// `demo.order.lookup` - the catalog row `order_lookup`.
#[preload(route = "demo.order.lookup", instances = 10)]
struct OrderLookup;

#[async_trait]
impl ComposableFunction for OrderLookup {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let id = order_id(input.body())?;
        Ok(EventEnvelope::new().set_raw_body(order(&id)?))
    }
}

/// `demo.shipment.status` - the catalog row `shipment_status`; the status is one of
/// `in_transit`, `delivered` or `lost`.
#[preload(route = "demo.shipment.status", instances = 10)]
struct ShipmentStatus;

#[async_trait]
impl ComposableFunction for ShipmentStatus {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let id = order_id(input.body())?;
        Ok(EventEnvelope::new().set_raw_body(shipment(&id)?))
    }
}

/// `demo.refund.issue` - the catalog row `issue_refund`, marked `write: true`. The mark on the
/// row, not anything in this code, is what makes the loop hold the call for a person: in the
/// node's default mode a marked write waits for an approval, and the graph suspends until a
/// supervisor decides. This function runs only after that decision, with the arguments the
/// supervisor saw.
#[preload(route = "demo.refund.issue", instances = 10)]
struct RefundIssue;

#[async_trait]
impl ComposableFunction for RefundIssue {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body = input.body();
        let id = order_id(body)?;
        let amount = field(body, "amount")
            .and_then(Value::as_f64)
            .ok_or_else(|| AppError::new(400, "The amount must be a number"))?;
        let reason = field(body, "reason").and_then(Value::as_str);
        Ok(EventEnvelope::new().set_raw_body(refund(&id, amount, reason)?))
    }
}
