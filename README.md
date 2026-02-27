# ARCANUM

加密聊天室系统 - 支持实时WebSocket通信、频道管理、用户管理、权限系统、文件上传等功能。

## 功能特性

### 核心功能
- 🔐 **安全认证**: JWT令牌认证，单点登录
- 💬 **实时聊天**: WebSocket实时通信
- 📁 **文件上传**: 支持图片和文本文件上传
- 👤 **头像系统**: 用户自定义头像

### 权限系统（12个模块化权限）
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

### 管理功能
- 用户封禁/解封
- 用户禁言（可设置时长）
- 踢出用户
- IP封禁
- 在线状态显示
- 权限下放（模块化授权）

### 性能优化
- 消息发送节流（300ms）
- API请求防抖
- 速率限制（可配置）

## 快速开始

### 本地运行
```bash
cargo build --release
./target/release/arcanum
```

### Docker部署
```bash
docker build -t arcanum .
docker run -p 3000:3000 -v arcanum-data:/app/data arcanum
```

### 环境变量
| 变量 | 默认值 | 说明 |
|------|--------|------|
| JWT_SECRET | arcanum-secret-change-in-production | JWT密钥 |
| JWT_EXPIRES | 604800 | 令牌有效期（秒） |
| ADMIN_UID | ARCANUM-ADMIN-0000 | 管理员UID |
| ADMIN_PASSWORD | admin123456 | 管理员密码 |
| DATABASE_URL | sqlite:arcanum.db?mode=rwc | 数据库连接 |
| MAX_FILE_SIZE | 5242880 | 最大文件大小（字节） |
| RATE_LIMIT_MESSAGES | 10 | 速率限制消息数 |
| RATE_LIMIT_WINDOW | 60 | 速率限制窗口（秒） |
| PORT | 3000 | 服务端口 |

## 默认管理员
- UID: `ARCANUM-ADMIN-0000`
- 密码: `admin123456`

**⚠️ 生产环境请务必修改默认密码！**

## API接口

### 认证
- `POST /api/auth/login` - 登录
- `POST /api/auth/logout` - 登出
- `GET /api/auth/me` - 获取当前用户信息

### 频道
- `GET /api/groups` - 我的频道列表
- `POST /api/groups` - 创建频道
- `POST /api/groups/enter` - 进入频道

### 消息
- `POST /api/messages` - 发送消息
- `GET /api/messages/group/:id` - 获取频道消息
- `POST /api/messages/file/:id` - 上传文件

### 管理接口
- `GET /api/admin/users` - 用户列表
- `POST /api/admin/users` - 创建用户
- `PUT /api/admin/users/:uid/ban` - 封禁用户
- `PUT /api/admin/users/:uid/unban` - 解封用户
- `PUT /api/admin/users/:uid/kick` - 踢出用户
- `PUT /api/admin/users/:uid/mute` - 禁言用户
- `PUT /api/admin/users/:uid/unmute` - 解除禁言
- `POST /api/admin/users/:uid/permissions` - 授予权限
- `DELETE /api/admin/users/:uid/permissions` - 撤销权限
- `GET /api/admin/permissions` - 权限列表
- `GET /api/admin/groups` - 所有频道
- `DELETE /api/admin/groups/:id` - 删除频道
- `GET /api/admin/ips` - IP封禁列表
- `POST /api/admin/ips/:ip` - 封禁IP
- `DELETE /api/admin/ips/:ip` - 解封IP

### WebSocket
- `GET /ws?token=xxx` - WebSocket连接

## 技术栈
- **后端**: Rust + Axum + SQLx + SQLite
- **前端**: 纯HTML/CSS/JavaScript（单文件）
- **认证**: JWT（HS256）
- **实时**: WebSocket
- **部署**: Docker

## License
MIT
