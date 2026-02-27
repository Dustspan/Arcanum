# ARCANUM

一个轻量级、高性能的实时聊天应用。

## ✨ 特性

### 核心功能
- 🔐 **用户认证** - JWT令牌认证，安全登录
- 👥 **权限管理** - 模块化权限系统，支持细粒度授权
- 💬 **频道聊天** - 创建频道，实时群聊
- 💌 **私聊功能** - 一对一私聊，消息实时送达
- 👫 **好友系统** - 添加好友，管理好友列表

### 消息功能
- 📎 **文件上传** - 支持图片、文件上传
- 🔍 **消息搜索** - 快速搜索历史消息
- ↩️ **消息引用** - 引用回复消息
- 📌 **消息置顶** - 重要消息置顶
- 🔄 **消息转发** - 转发消息到其他频道
- ⏪ **消息撤回** - 撤回已发送消息
- ✅ **已读状态** - 查看消息已读状态
- 📢 **@提及** - @用户提醒

### 管理功能
- 📊 **数据统计** - 用户、消息、频道统计
- 🛡️ **敏感词过滤** - 自动过滤敏感词
- 📝 **操作日志** - 记录所有管理操作
- 🚫 **用户管理** - 封禁、禁言、踢出
- 🌐 **IP管理** - IP封禁管理
- 🔗 **邀请链接** - 生成频道邀请链接

### 用户体验
- 🌓 **主题切换** - 深色/浅色主题
- 📱 **响应式设计** - 完美适配移动端
- 📲 **PWA支持** - 可安装为应用
- ⌨️ **输入提示** - 实时显示输入状态
- 🔔 **消息通知** - 浏览器推送通知

## 🚀 快速开始

### Docker部署（推荐）

```bash
# 克隆仓库
git clone https://github.com/Dustspan/Arcanum.git
cd Arcanum

# 启动服务
docker-compose up -d

# 访问 http://localhost:3000
# 默认管理员: ADMIN / admin123
```

### 手动部署

```bash
# 安装Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 构建
cargo build --release

# 运行
PORT=3000 DATA_DIR=./data JWT_SECRET=your-secret ./target/release/arcanum
```

## ⚙️ 环境变量

| 变量 | 说明 | 默认值 |
|------|------|--------|
| PORT | 服务端口 | 3000 |
| DATA_DIR | 数据目录 | ./data |
| JWT_SECRET | JWT密钥 | 随机生成 |
| ADMIN_UID | 管理员UID | ADMIN |
| ADMIN_PASSWORD | 管理员密码 | admin123 |

## 🌐 免费部署

### Railway
1. 连接GitHub仓库
2. 自动检测Dockerfile
3. 设置环境变量
4. 部署完成

### Render
1. 创建Web Service
2. 连接GitHub仓库
3. 选择Docker环境
4. 部署

### Fly.io
```bash
fly launch
fly deploy
```

## 📡 API文档

### 认证
```
POST /api/auth/login     - 登录
POST /api/auth/logout    - 登出
GET  /api/auth/me        - 获取当前用户
```

### 频道
```
POST /api/groups/enter      - 进入频道
POST /api/groups            - 创建频道
GET  /api/groups            - 获取我的频道
GET  /api/groups/:id        - 获取频道信息
PUT  /api/groups/:id        - 更新频道
GET  /api/groups/:id/members - 获取成员
POST /api/groups/:id/invite - 创建邀请链接
POST /api/invite/:code      - 通过邀请加入
```

### 消息
```
POST /api/messages           - 发送消息
GET  /api/messages/group/:id - 获取消息列表
GET  /api/messages/group/:id/search - 搜索消息
DELETE /api/messages/:id     - 删除消息
POST /api/messages/:id/recall - 撤回消息
POST /api/messages/:id/read  - 标记已读
POST /api/messages/:id/pin   - 置顶消息
POST /api/messages/:id/forward - 转发消息
POST /api/messages/file/:id  - 上传文件
```

### 私聊
```
POST /api/direct/:id    - 发送私聊
GET  /api/direct/:id    - 获取私聊消息
GET  /api/conversations - 获取会话列表
```

### 好友
```
GET  /api/friends           - 获取好友列表
GET  /api/friends/requests  - 获取好友请求
POST /api/friends/:id       - 添加好友
POST /api/friends/:id/accept - 接受好友
```

### 管理
```
GET  /api/admin/users        - 获取用户列表
POST /api/admin/users        - 创建用户
PUT  /api/admin/users/:id/ban - 封禁用户
PUT  /api/admin/users/:id/unban - 解封用户
PUT  /api/admin/users/:id/mute - 禁言用户
PUT  /api/admin/users/:id/unmute - 解除禁言
PUT  /api/admin/users/:id/kick - 踢出用户
POST /api/admin/users/:id/permissions - 授予权限
DELETE /api/admin/users/:id/permissions - 撤销权限

GET  /api/admin/groups       - 获取所有频道
DELETE /api/admin/groups/:id - 删除频道

GET  /api/admin/ips          - 获取封禁IP
POST /api/admin/ips/:ip      - 封禁IP
DELETE /api/admin/ips/:ip    - 解封IP

GET  /api/admin/sensitive-words - 获取敏感词
POST /api/admin/sensitive-words - 添加敏感词
DELETE /api/admin/sensitive-words/:id - 删除敏感词

GET  /api/admin/audit-logs   - 获取操作日志
GET  /api/admin/statistics   - 获取统计数据
GET  /api/admin/permissions  - 获取权限列表
```

### WebSocket
```
WS /ws?token=xxx  - WebSocket连接

事件:
- message      - 新消息
- message_recall - 消息撤回
- message_read - 已读状态
- typing       - 输入状态
- mention      - 提及通知
- direct_message - 私聊消息
- friend_request - 好友请求
```

## 🔒 权限系统

### 权限列表
| 权限 | 说明 |
|------|------|
| user_create | 创建用户 |
| user_view | 查看用户列表 |
| user_ban | 封禁/解封用户 |
| user_kick | 踢出用户 |
| user_mute | 禁言用户 |
| group_create | 创建频道 |
| group_view | 查看所有频道 |
| group_delete | 删除频道 |
| message_delete | 删除消息 |
| ip_ban | 封禁IP |
| permission_grant | 授予权限 |
| file_upload | 上传文件 |

### 权限缓存
系统使用内存缓存权限数据，减少数据库查询，提升性能。

## 🛠️ 技术栈

- **后端**: Rust + Axum
- **数据库**: SQLite (sqlx)
- **实时通信**: WebSocket
- **认证**: JWT
- **前端**: 原生 HTML/CSS/JS

## 📊 性能

- 二进制大小: ~4MB
- 内存占用: ~20MB
- 支持并发: 数千连接
- 启动时间: <1秒

## 📝 开发

```bash
# 开发模式
cargo run

# 运行测试
cargo test

# 代码检查
cargo clippy
```

## 📄 许可证

MIT License
