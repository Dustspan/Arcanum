use tokio::sync::broadcast;
use dashmap::DashMap;
use serde::{Serialize, Deserialize};

pub type GroupTx = broadcast::Sender<WsMessage>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    pub event: String,
    pub data: serde_json::Value,
}

/// 频道广播管理器
/// 每个频道有独立的广播通道，用户只接收自己订阅频道的消息
pub struct BroadcastManager {
    /// 频道ID -> 广播发送器
    groups: DashMap<String, GroupTx>,
    /// 用户ID -> 广播发送器（用于个人通知）
    users: DashMap<String, GroupTx>,
    /// 全局广播通道（用于系统消息）
    global: broadcast::Sender<WsMessage>,
}

impl BroadcastManager {
    pub fn new() -> Self {
        let (global, _) = broadcast::channel(1000);
        Self {
            groups: DashMap::new(),
            users: DashMap::new(),
            global,
        }
    }
    
    /// 获取或创建用户的广播通道
    pub fn get_or_create_user(&self, user_id: &str) -> GroupTx {
        self.users.entry(user_id.to_string())
            .or_insert_with(|| {
                let (tx, _) = broadcast::channel(100);
                tx
            })
            .clone()
    }
    
    /// 向用户发送消息
    pub fn broadcast_to_user(&self, user_id: &str, msg: WsMessage) -> Result<usize, broadcast::error::SendError<WsMessage>> {
        let tx = self.get_or_create_user(user_id);
        tx.send(msg)
    }
    
    /// 订阅用户通知
    pub fn subscribe_user(&self, user_id: &str) -> broadcast::Receiver<WsMessage> {
        self.get_or_create_user(user_id).subscribe()
    }
    
    /// 获取或创建频道的广播通道
    pub fn get_or_create_group(&self, group_id: &str) -> GroupTx {
        self.groups.entry(group_id.to_string())
            .or_insert_with(|| {
                let (tx, _) = broadcast::channel(500);
                tx
            })
            .clone()
    }
    
    /// 订阅频道，返回接收器
    pub fn subscribe(&self, group_id: &str) -> broadcast::Receiver<WsMessage> {
        self.get_or_create_group(group_id).subscribe()
    }
    
    /// 向频道广播消息
    pub fn broadcast_to_group(&self, group_id: &str, msg: WsMessage) -> Result<usize, broadcast::error::SendError<WsMessage>> {
        if let Some(tx) = self.groups.get(group_id) {
            tx.send(msg)
        } else {
            Ok(0)
        }
    }
    
    /// 全局广播（系统消息）
    pub fn broadcast_global(&self, msg: WsMessage) -> Result<usize, broadcast::error::SendError<WsMessage>> {
        self.global.send(msg)
    }
    
    /// 订阅全局通道
    pub fn subscribe_global(&self) -> broadcast::Receiver<WsMessage> {
        self.global.subscribe()
    }
    
    /// 清理空频道（可选，用于内存优化）
    pub fn cleanup_empty_groups(&self) {
        self.groups.retain(|_, tx| tx.receiver_count() > 0);
    }
    
    /// 获取活跃频道数量
    pub fn active_groups(&self) -> usize {
        self.groups.len()
    }
    
    /// 获取频道订阅者数量
    pub fn subscriber_count(&self, group_id: &str) -> usize {
        self.groups.get(group_id).map(|tx| tx.receiver_count()).unwrap_or(0)
    }
}

impl Default for BroadcastManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for BroadcastManager {
    fn clone(&self) -> Self {
        Self {
            groups: self.groups.clone(),
            users: self.users.clone(),
            global: self.global.clone(),
        }
    }
}
