use crate::client::Client;
use ureq::{Error};
use serde::{Deserialize, Serialize};

//const ENDPOINT_BASE: &str = "rcs";
const ENDPOINT_MESSAGES: &str =   "rcs/messages";
const ENDPOINT_EVENTS: &str =   "rcs/events";

#[derive(Serialize)]
pub enum RcsEvent {
    #[serde(rename = "IS_TYPING")]
    IsTyping,
    #[serde(rename = "READ")]
    Read,
}

impl RcsEvent {
    fn as_str(&self) -> &'static str {
        match self {
            RcsEvent::IsTyping => "IS_TYPING",
            RcsEvent::Read => "READ",
        }
    }
}

#[derive(Serialize)]
pub struct RcsDeleteParams {
    pub id: u64,
}

#[derive(Serialize)]
pub struct RcsEventParams {
    pub event: RcsEvent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
    pub to: String,
}

#[derive(Serialize)]
pub struct RcsDispatchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub text: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_tracking: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct RcsDeleteResponse {
    pub success: bool,
}

#[derive(Deserialize, Debug)]
pub struct RcsEventResponse {
    pub success: bool,
}

#[derive(Deserialize, Debug)]
pub struct RcsResponse {
    pub balance: f64,
    pub debug: String,
    pub messages: Vec<RcsMessage>,
    pub sms_type: String,
    pub success: String,
    pub total_price: f64,
}

#[derive(Deserialize, Debug)]
pub struct RcsMessage {
    pub channel: String,
    pub encoding: String,
    pub error: Option<String>,
    pub error_text: Option<String>,
    pub id: Option<String>,
    pub messages: Option<Vec<String>>,
    pub parts: u16,
    pub price: f64,
    pub recipient: String,
    pub sender: String,
    pub success: bool,
    pub text: String,
}

pub struct Rcs {
    client: Client
}

impl Rcs {
    pub fn new(client: Client) -> Self {
        Rcs {
            client,
        }
    }

    pub fn delete(&self, params: RcsDeleteParams) -> Result<RcsDeleteResponse, Error> {
        let id = params.id;
        let endpoint = format!("{ENDPOINT_MESSAGES}/{id}");

        Ok(self.client.request("DELETE",  &*endpoint)
            .call()
            .unwrap()
            //.send_form(&*[])?
            .into_json::<RcsDeleteResponse>()?)
    }

    pub fn event(&self, params: RcsEventParams) -> Result<RcsEventResponse, Error> {
        let dirty_data = &[
            ("event", &*params.event.as_str()),
            ("msg_id", &*params.msg_id.unwrap_or_default()),
            ("to", &*params.to),
        ];

        Ok(self.client.request("POST", ENDPOINT_EVENTS)
            .send_form(dirty_data)?
            .into_json::<RcsEventResponse>()?)
    }

    pub fn dispatch(&self, params: RcsDispatchParams) -> Result<RcsResponse, Error> {
        let json = serde_json::to_string(&params).unwrap();

        Ok(self.client.request("POST", ENDPOINT_MESSAGES)
            .set("Content-Type", "application/json")
            .send_string(&*json)?
            .into_json::<RcsResponse>()?)
    }
}