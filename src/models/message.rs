use serde::Deserialize;

/// 发送消息请求
#[derive(Debug, Deserialize)]
pub struct SendMessageRequest { 
    pub group_id: String, 
    pub content: String, 
    pub burn_after: Option<i64>,
    pub msg_type: Option<String>,
    pub file_name: Option<String>,
    pub file_size: Option<i64>,
}
