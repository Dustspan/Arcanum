use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EnterGroupRequest { pub name: String }

#[derive(Debug, Deserialize)]
pub struct CreateGroupRequest { pub name: String }
