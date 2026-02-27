use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest { 
    pub group_id: String, 
    pub content: String, 
    pub burn_after: Option<i64>,
    pub msg_type: Option<String>,
    pub file_name: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageInfo {
    pub id: String,
    pub sender_id: String,
    pub sender_nickname: String,
    pub sender_avatar: Option<String>,
    pub group_id: String,
    pub content: String,
    pub msg_type: String,
    pub file_name: Option<String>,
    pub file_size: i64,
    pub burn_after: i64,
    pub created_at: String,
}
