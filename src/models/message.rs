use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest { pub group_id: String, pub content: String, pub burn_after: Option<i64> }
