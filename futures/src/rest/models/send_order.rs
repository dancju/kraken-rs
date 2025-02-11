use super::Request;
use crate::common::{OrderEvent, OrderType, SendOrderStatus, Side, Symbol, TriggerSignal};
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SendOrderRequest {
    pub order_type: OrderType,
    pub symbol: Symbol,
    pub side: Side,
    pub size: u64,
    pub limit_price: f64,
    pub stop_price: Option<f64>,
    pub trigger_signal: Option<TriggerSignal>,
    pub cli_ord_id: Option<Uuid>,
    pub reduce_only: Option<bool>,
}

impl SendOrderRequest {
    pub fn limit(symbol: Symbol, price: f64, qty: i64) -> Self {
        let side = if qty > 0 { Side::Buy } else { Side::Sell };
        Self {
            order_type: OrderType::Lmt,
            symbol: symbol,
            side: side,
            size: qty.abs() as u64,
            limit_price: price,
            stop_price: None,
            trigger_signal: None,
            cli_ord_id: None,
            reduce_only: None,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct SendOrderResponse {
    #[serde(rename = "sendStatus")]
    pub send_status: SendStatus,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SendStatus {
    Success {
        order_id: Uuid,
        status: SendOrderStatus,
        #[serde(rename = "receivedTime")]
        received_time: DateTime<Utc>,
        #[serde(rename = "orderEvents")]
        order_events: Vec<OrderEvent>,
    },
    Fail {
        status: SendOrderStatus,
    },
}

impl SendStatus {
    pub fn order_id(&self) -> Option<Uuid> {
        match self {
            SendStatus::Success { order_id, .. } => Some(*order_id),
            _ => None,
        }
    }

    pub fn status(&self) -> SendOrderStatus {
        match self {
            SendStatus::Success { status, .. } => *status,
            SendStatus::Fail { status } => *status,
        }
    }
}

impl Request for SendOrderRequest {
    const METHOD: Method = Method::POST;
    const SIGNED: bool = true;
    const ENDPOINT: &'static str = "/sendorder";
    const HAS_PAYLOAD: bool = true;
    type Response = SendOrderResponse;
}
