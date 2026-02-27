# ARCANUM - 加密聊天应用

一个轻量级、高性能的实时聊天应用，支持频道聊天、私聊、好友系统等功能。

## 特性

- 🔐 用户认证与权限管理
- 💬 频道聊天与私聊
- 👥 好友系统
- 📎 文件上传
- 🔍 消息搜索
- 📌 消息置顶、引用、转发
- @提及功能
- 🌓 深色/浅色主题
- 📱 移动端响应式
- 🚀 PWA支持
- 🔒 敏感词过滤
- 📊 操作日志

## 快速开始

### Docker部署（推荐）

```bash
# 克隆仓库
git clone https://github.com/Dustspan/Arcanum.git
cd Arcanum

# 使用Docker Compose启动
docker-compose up -d

# 访问 http://localhost:3000
```

### 手动部署

```bash
# 安装Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 构建
cargo build --release

# 运行
PORT=3000 DATA_DIR=./data ./target/release/arcanum
```

## 环境变量

| 变量 | 说明 | 默认值 |
|------|------|--------|
| PORT | 服务端口 | 3000 |
| DATA_DIR | 数据目录 | ./data |
| JWT_SECRET | JWT密钥 | 随机生成 |
| ADMIN_UID | 管理员UID | ADMIN |
| ADMIN_PASSWORD | 管理员密码 | admin123 |

## 免费部署选项

### Railway
1. 连接GitHub仓库
2. 自动检测Dockerfile
3. 部署完成

### Render
1. 创建新的Web Service
2. 连接GitHub仓库
3. 选择Docker环境
4. 部署

### Fly.io
```bash
fly launch
fly deploy
```

## API文档

### 认证
- `POST /api/auth/login` - 登录
- `POST /api/auth/logout` - 登出
- `GET /api/auth/me` - 获取当前用户

### 频道
- `POST /api/groups/enter` - 进入频道
- `POST /api/groups` - 创建频道
- `GET /api/groups` - 获取我的频道
- `GET /api/groups/:id` - 获取频道信息
- `PUT /api/groups/:id` - 更新频道
- `GET /api/groups/:id/members` - 获取成员列表
- `POST /api/groups/:id/invite` - 创建邀请链接
- `POST /api/invite/:code` - 通过邀请加入

### 消息
- `POST /api/messages` - 发送消息
- `GET /api/messages/group/:id` - 获取消息列表
- `GET /api/messages/group/:id/search` - 搜索消息
- `DELETE /api/messages/:id` - 删除消息
- `POST /api/messages/:id/recall` - 撤回消息
- `POST /api/messages/:id/read` - 标记已读
- `POST /api/messages/:id/pin` - 置顶消息
- `POST /api/messages/:id/forward` - 转发消息

### 私聊
- `POST /api/direct/:id` - 发送私聊
- `GET /api/direct/:id` - 获取私聊消息
- `GET /api/conversations` - 获取会话列表

### 好友
- `GET /api/friends` - 获取好友列表
- `GET /api/friends/requests` - 获取好友请求
- `POST /api/friends/:id` - 添加好友
- `POST /api/friends/:id/accept` - 接受好友

### 管理
- `GET /api/admin/users` - 获取用户列表
- `POST /api/admin/users` - 创建用户
- `PUT /api/admin/users/:id/ban` - 封禁用户
- `PUT /api/admin/users/:id/mute` - 禁言用户
- `GET /api/admin/statistics` - 获取统计
- `GET /api/admin/audit-logs` - 获取日志
- `GET /api/admin/sensitive-words` - 获取敏感词
- `POST /api/admin/sensitive-words` - 添加敏感词

## 技术栈

- **后端**: Rust + Axum
- **数据库**: SQLite
- **实时通信**: WebSocket
- **前端**: 原生HTML/CSS/JS

## 许可证

MIT License
