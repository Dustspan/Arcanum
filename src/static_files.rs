pub const INDEX_HTML: &str = r##"<!DOCTYPE html>
<html lang="zh">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1.0,maximum-scale=1.0,user-scalable=no">
<meta name="theme-color" content="#000000">
<title>ARCANUM</title>
<link rel="manifest" href="/manifest.json">
<style>
[v-cloak]{display:none!important}
#app{display:none}
#app.loaded{display:block}
.loading-screen{position:fixed;inset:0;background:var(--bg);display:flex;align-items:center;justify-content:center;color:var(--accent);font-size:18px}

:root{--bg:#0a0a0f;--bg2:#12121a;--card:#16161f;--text:#e0e0e8;--muted:#6a6a7a;--accent:#00f0ff;--accent2:#ff00aa;--border:#2a2a3a;--error:#ff3366;--success:#00ff88;--warn:#ffaa00}
[data-theme="light"]{--bg:#f0f0f5;--bg2:#e8e8f0;--card:#fff;--text:#1a1a2e;--muted:#6a6a7a;--accent:#0088aa;--border:#d0d0da}
*{margin:0;padding:0;box-sizing:border-box}
body{background:var(--bg);color:var(--text);font-family:system-ui,sans-serif;min-height:100vh}
.container{max-width:540px;margin:0 auto;padding:12px;min-height:100vh}

.btn{padding:10px 20px;background:transparent;border:1px solid var(--accent);color:var(--accent);border-radius:4px;font-size:14px;cursor:pointer;transition:all .2s}
.btn:hover{background:var(--accent);color:#000}
.btn:disabled{opacity:.5;cursor:not-allowed}
.btn.full{width:100%}
.btn.sm{padding:6px 12px;font-size:12px}
.btn.danger{border-color:var(--error);color:var(--error)}
.btn.danger:hover{background:var(--error);color:#fff}
.btn.xs{padding:4px 8px;font-size:10px}

.input{width:100%;padding:12px;background:var(--bg2);border:1px solid var(--border);color:var(--text);border-radius:4px;font-size:14px;outline:none}
.input:focus{border-color:var(--accent)}
textarea.input{resize:none;min-height:80px}

.card{background:var(--card);border:1px solid var(--border);border-radius:8px;padding:16px;margin-bottom:12px}
.err{color:var(--error);font-size:13px;padding:8px;background:rgba(255,51,102,.1);border-radius:4px;margin-top:8px}
.success-msg{color:var(--success);font-size:13px;padding:8px;background:rgba(0,255,136,.1);border-radius:4px;margin-top:8px}

.login-logo{font-size:28px;font-weight:700;text-align:center;margin:60px 0 30px;color:var(--accent)}
.login-form{display:flex;flex-direction:column;gap:12px}

.header{display:flex;justify-content:space-between;align-items:center;padding:8px 0;margin-bottom:12px}
.header h1{font-size:16px;color:var(--accent);cursor:pointer}
.header-info{font-size:11px;color:var(--muted)}
.header-actions{display:flex;gap:6px}

.tab-bar{display:flex;gap:4px;margin-bottom:12px;border-bottom:1px solid var(--border);padding-bottom:8px}
.tab-item{flex:1;text-align:center;padding:8px;color:var(--muted);cursor:pointer;font-size:13px;border-bottom:2px solid transparent}
.tab-item.active{color:var(--accent);border-bottom-color:var(--accent)}

.channel-input{display:flex;gap:8px;margin-bottom:16px}
.channel-list{display:flex;flex-direction:column;gap:8px}
.channel-card{background:var(--card);border:1px solid var(--border);border-radius:8px;padding:14px;cursor:pointer;position:relative}
.channel-card:hover{border-color:var(--accent)}
.channel-card h3{font-size:14px;margin-bottom:4px}
.channel-card p{font-size:12px;color:var(--muted)}
.channel-card .unread-badge{position:absolute;top:8px;right:8px;background:var(--error);color:#fff;font-size:10px;padding:2px 6px;border-radius:10px;min-width:18px;text-align:center}

.chat-wrap{display:flex;flex-direction:column;height:calc(100vh - 100px)}
.chat-header{display:flex;justify-content:space-between;align-items:center;padding:12px;background:var(--card);border-bottom:1px solid var(--border)}
.chat-header h3{font-size:14px}
.chat-header-info{font-size:11px;color:var(--muted);display:flex;gap:8px}
.chat-header-info span{cursor:pointer}
.chat-msgs{flex:1;overflow-y:auto;padding:12px;display:flex;flex-direction:column;gap:10px;background:var(--bg2)}
.msg-row{display:flex;gap:8px;position:relative}
.msg-row.me{flex-direction:row-reverse}
.msg-avatar{width:32px;height:32px;border-radius:6px;background:linear-gradient(135deg,var(--accent),var(--accent2));display:flex;align-items:center;justify-content:center;font-size:12px;font-weight:600;color:#000;flex-shrink:0;cursor:pointer;overflow:hidden}
.msg-avatar img{width:100%;height:100%;object-fit:cover}
.msg-content{max-width:75%;display:flex;flex-direction:column}
.msg-bubble{padding:8px 12px;border-radius:10px;font-size:13px;position:relative}
.msg-bubble.in{background:var(--card);border:1px solid var(--border)}
.msg-bubble.out{background:var(--accent);color:#000}
.msg-bubble.pinned{border-left:3px solid var(--warn)}
.msg-nick{font-size:10px;color:var(--accent);margin-bottom:2px}
.msg-time{font-size:9px;color:var(--muted);margin-top:2px;text-align:right}
.msg-actions{display:none;position:absolute;top:-20px;right:0;background:var(--card);border:1px solid var(--border);border-radius:4px;padding:2px}
.msg-row:hover .msg-actions{display:flex;gap:2px}
.msg-action{background:none;border:none;color:var(--muted);font-size:10px;padding:2px 6px;cursor:pointer}
.msg-action:hover{color:var(--accent)}
.msg-img{max-width:180px;border-radius:6px;cursor:pointer}
.msg-file{display:flex;align-items:center;gap:8px;padding:8px;background:var(--bg);border-radius:6px}
.msg-file-icon{font-size:20px}
.msg-file-info{font-size:12px}
.msg-mention{color:var(--accent);cursor:pointer}
.msg-mention:hover{text-decoration:underline}
.reply-preview{background:var(--bg);padding:4px 8px;border-radius:4px;margin-bottom:4px;font-size:11px;border-left:2px solid var(--accent);max-width:200px;overflow:hidden}
.reply-preview .reply-nick{color:var(--accent)}
.reply-preview .reply-content{color:var(--muted);white-space:nowrap;overflow:hidden;text-overflow:ellipsis}
.forward-tag{font-size:10px;color:var(--warn);margin-bottom:2px}

.chat-input-wrap{background:var(--bg);border-top:1px solid var(--border)}
.reply-bar{display:flex;align-items:center;justify-content:space-between;padding:6px 12px;background:var(--bg2);font-size:12px}
.reply-bar .reply-close{background:none;border:none;color:var(--muted);cursor:pointer;font-size:16px}
.chat-input{display:flex;gap:8px;padding:12px;align-items:flex-end}
.chat-input textarea{flex:1;padding:8px 12px;background:var(--card);border:1px solid var(--border);color:var(--text);border-radius:16px;font-size:13px;outline:none;resize:none;max-height:80px}
.chat-input input[type="file"]{display:none}
.chat-tools{display:flex;gap:4px;flex-direction:column}
.tool-btn{background:var(--bg2);border:1px solid var(--border);border-radius:50%;width:28px;height:28px;display:flex;align-items:center;justify-content:center;cursor:pointer;color:var(--muted);font-size:14px}
.tool-btn:hover{border-color:var(--accent);color:var(--accent)}

.emoji-picker{position:absolute;bottom:100%;left:0;background:var(--card);border:1px solid var(--border);border-radius:8px;padding:8px;display:none;flex-wrap:wrap;gap:4px;width:200px;z-index:10}
.emoji-picker.show{display:flex}
.emoji-item{font-size:18px;cursor:pointer;padding:4px;border-radius:4px}
.emoji-item:hover{background:var(--bg2)}

.members-panel{position:absolute;top:0;right:0;width:200px;height:100%;background:var(--card);border-left:1px solid var(--border);transform:translateX(100%);transition:transform .3s;z-index:100}
.members-panel.show{transform:translateX(0)}
.members-panel h4{padding:12px;border-bottom:1px solid var(--border);font-size:13px}
.members-list{overflow-y:auto;max-height:calc(100% - 45px)}
.member-item{display:flex;align-items:center;gap:8px;padding:8px 12px;cursor:pointer}
.member-item:hover{background:var(--bg2)}
.member-avatar{width:24px;height:24px;border-radius:4px;background:linear-gradient(135deg,var(--accent),var(--accent2));display:flex;align-items:center;justify-content:center;font-size:10px;color:#000}
.member-avatar img{width:100%;height:100%;object-fit:cover;border-radius:4px}
.member-info{font-size:12px}
.member-status{width:6px;height:6px;border-radius:50%;background:var(--success)}
.member-status.offline{background:var(--muted)}

.admin-tabs{display:flex;gap:4px;margin-bottom:12px;flex-wrap:wrap}
.admin-tab{flex:1;min-width:50px;padding:8px;background:transparent;border:1px solid var(--border);color:var(--muted);border-radius:4px;font-size:11px;cursor:pointer}
.admin-tab.active{border-color:var(--accent);color:var(--accent)}
.admin-section{display:none}
.admin-section.active{display:block}

.item-card{background:var(--bg2);border:1px solid var(--border);border-radius:6px;padding:10px;margin-bottom:6px}
.item-header{display:flex;justify-content:space-between;align-items:center}
.item-title{font-size:13px}
.item-info{font-size:10px;color:var(--muted);margin-top:4px}

.stats-grid{display:grid;grid-template-columns:repeat(2,1fr);gap:10px}
.stat-card{background:var(--bg2);border:1px solid var(--border);border-radius:6px;padding:12px;text-align:center}
.stat-value{font-size:20px;font-weight:600;color:var(--accent)}
.stat-label{font-size:10px;color:var(--muted)}

.modal-mask{position:fixed;inset:0;background:rgba(0,0,0,.8);display:flex;align-items:center;justify-content:center;z-index:1000;padding:12px}
.modal{background:var(--card);border:1px solid var(--border);border-radius:8px;max-width:400px;width:100%;max-height:90vh;overflow-y:auto}
.modal-header{display:flex;justify-content:space-between;align-items:center;padding:12px;border-bottom:1px solid var(--border)}
.modal-header h3{font-size:14px}
.modal-close{background:none;border:none;color:var(--muted);font-size:20px;cursor:pointer}
.modal-body{padding:12px}
.modal-footer{padding:12px;border-top:1px solid var(--border);display:flex;gap:8px;justify-content:flex-end}

.user-menu{position:fixed;background:var(--card);border:1px solid var(--border);border-radius:6px;padding:6px;z-index:1001;min-width:150px;box-shadow:0 4px 20px rgba(0,0,0,.5)}
.user-menu-header{padding:6px;border-bottom:1px solid var(--border);margin-bottom:6px;display:flex;align-items:center;gap:8px}
.user-menu-item{display:block;width:100%;padding:6px 10px;background:none;border:none;color:var(--text);text-align:left;cursor:pointer;border-radius:4px;font-size:12px}
.user-menu-item:hover{background:var(--bg2)}
.user-menu-item.danger{color:var(--error)}

.badge{display:inline-block;padding:2px 6px;border-radius:8px;font-size:9px;margin-left:4px}
.badge.success{background:rgba(0,255,136,.2);color:var(--success)}
.badge.error{background:rgba(255,51,102,.2);color:var(--error)}
.badge.warn{background:rgba(255,170,0,.2);color:var(--warn)}

.perm-list{display:flex;flex-wrap:wrap;gap:4px;margin-top:6px}
.perm-tag{font-size:10px;padding:2px 6px;background:var(--bg);border:1px solid var(--border);border-radius:4px}

.friend-item{display:flex;align-items:center;gap:8px;padding:10px;background:var(--bg2);border-radius:6px;margin-bottom:6px;cursor:pointer}
.friend-item:hover{background:var(--card)}
.friend-avatar{width:36px;height:36px;border-radius:6px;background:linear-gradient(135deg,var(--accent),var(--accent2));display:flex;align-items:center;justify-content:center;font-size:14px;color:#000;overflow:hidden}
.friend-avatar img{width:100%;height:100%;object-fit:cover}
.friend-info{flex:1}
.friend-name{font-size:13px}
.friend-status{font-size:10px;color:var(--muted)}
.friend-status.online{color:var(--success)}
.friend-request-badge{background:var(--error);color:#fff;font-size:10px;padding:2px 6px;border-radius:10px}

.search-box{display:flex;gap:8px;margin-bottom:12px}
.search-results{max-height:200px;overflow-y:auto}

.profile-section{display:flex;flex-direction:column;align-items:center;gap:12px;margin-bottom:16px}
.profile-avatar{width:80px;height:80px;border-radius:50%;background:linear-gradient(135deg,var(--accent),var(--accent2));display:flex;align-items:center;justify-content:center;font-size:28px;color:#000;cursor:pointer;overflow:hidden;position:relative}
.profile-avatar img{width:100%;height:100%;object-fit:cover}
.profile-avatar .avatar-edit{position:absolute;bottom:0;left:0;right:0;background:rgba(0,0,0,.7);font-size:10px;text-align:center;padding:2px}

/* 私聊界面 */
.dm-header{display:flex;align-items:center;gap:8px;padding:12px;background:var(--card);border-bottom:1px solid var(--border)}
.dm-back{background:none;border:none;color:var(--accent);font-size:20px;cursor:pointer}
.dm-title{font-size:14px}
.dm-status{font-size:10px;color:var(--success)}
.dm-status.offline{color:var(--muted)}

/* 图片预览 */
.image-preview{position:fixed;inset:0;background:rgba(0,0,0,.95);display:flex;align-items:center;justify-content:center;z-index:2000;cursor:zoom-out}
.image-preview img{max-width:90%;max-height:90%;object-fit:contain}

/* 消息搜索 */
.search-panel{background:var(--card);border-bottom:1px solid var(--border);padding:8px}
.search-panel input{width:100%}

/* 置顶消息 */
.pinned-msg{background:var(--card);border-bottom:1px solid var(--border);padding:8px 12px;font-size:12px;display:flex;align-items:center;gap:8px}
.pinned-msg .pin-icon{color:var(--warn)}
.pinned-msg .pin-content{flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.pinned-msg .pin-close{background:none;border:none;color:var(--muted);cursor:pointer}
</style>
</head>
<body>
<div class="loading-screen" id="loading">ARCANUM</div>

<div id="app">
<div class="container">

<!-- 登录页 -->
<div v-if="!loggedIn">
<div class="login-logo">ARCANUM</div>
<div class="card">
<form class="login-form" @submit.prevent="doLogin">
<input class="input" v-model="loginForm.uid" placeholder="UID" autocomplete="username">
<input class="input" v-model="loginForm.pwd" type="password" placeholder="密码" autocomplete="current-password">
<div class="err" v-if="loginError">{{ loginError }}</div>
<button class="btn full" type="submit" :disabled="loginLoading">{{ loginLoading ? '登录中...' : '进入' }}</button>
</form>
</div>
</div>

<!-- 主页 -->
<div v-else>
<!-- 头部 -->
<div class="header">
<div @click="showProfile = true">
<h1>{{ user.nickname }}</h1>
<div class="header-info">{{ user.uid }} <span v-if="isAdmin" class="badge error">管理员</span></div>
</div>
<div class="header-actions">
<button class="btn sm" @click="toggleTheme">{{ theme === 'dark' ? '☀' : '🌙' }}</button>
<button class="btn sm" v-if="canAccessAdmin" @click="openAdmin">⚙</button>
<button class="btn sm danger" @click="doLogout">退出</button>
</div>
</div>

<!-- 标签栏 -->
<div class="tab-bar" v-if="!currentGroup && !dmTarget">
<div class="tab-item" :class="{active: mainTab === 'channels'}" @click="mainTab = 'channels'">频道</div>
<div class="tab-item" :class="{active: mainTab === 'friends'}" @click="mainTab = 'friends'; loadFriends()">
好友<span v-if="friendRequestCount > 0" class="friend-request-badge">{{ friendRequestCount }}</span>
</div>
</div>

<!-- 私聊界面 -->
<div v-if="dmTarget" class="card chat-wrap">
<div class="dm-header">
<button class="dm-back" @click="closeDM">←</button>
<div class="msg-avatar">{{ dmTarget.nickname?.charAt(0) }}</div>
<div>
<div class="dm-title">{{ dmTarget.nickname }}</div>
<div class="dm-status" :class="{offline: !dmTarget.online}">{{ dmTarget.online ? '在线' : '离线' }}</div>
</div>
</div>
<div class="chat-msgs" ref="dmMsgsBox">
<div class="msg-row" v-for="m in dmMessages" :key="m.id" :class="{me: m.senderId === user.id}">
<div class="msg-avatar">{{ m.senderNickname?.charAt(0) }}</div>
<div class="msg-content">
<div class="msg-bubble" :class="m.senderId === user.id ? 'out' : 'in'">
<div class="msg-nick" v-if="m.senderId !== user.id">{{ m.senderNickname }}</div>
<div v-if="m.msgType === 'image'"><img class="msg-img" :src="m.content" @click="previewImage(m.content)"></div>
<div v-else>{{ m.content }}</div>
<div class="msg-time">{{ formatTime(m.createdAt) }}</div>
</div>
</div>
</div>
</div>
<div class="chat-input">
<textarea v-model="dmInput" placeholder="私聊消息..." @keyup.enter="sendDM" rows="1"></textarea>
<button class="btn" @click="sendDM">→</button>
</div>
</div>

<!-- 频道列表 -->
<div v-else-if="!currentGroup && mainTab === 'channels'">
<div class="channel-input">
<input class="input" v-model="channelInput" placeholder="输入频道名进入" @keyup.enter="doEnterChannel">
<button class="btn" @click="doEnterChannel" :disabled="channelLoading">{{ channelLoading ? '...' : '进入' }}</button>
</div>
<div class="channel-list">
<div class="channel-card" v-for="g in groups" :key="g.id" @click="doJoinGroup(g.id)">
<h3>{{ g.name }}</h3>
<p>成员: {{ g.memberCount }}</p>
<span v-if="g.unread > 0" class="unread-badge">{{ g.unread > 99 ? '99+' : g.unread }}</span>
</div>
<div class="card" v-if="groups.length === 0" style="text-align:center;color:var(--muted);font-size:13px">
暂无频道<br><small>请联系管理员创建频道</small>
</div>
</div>
</div>

<!-- 好友列表 -->
<div v-else-if="!currentGroup && mainTab === 'friends'">
<div class="card">
<div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:8px">
<span style="font-size:13px">好友请求</span>
<span v-if="friendRequests.length > 0" class="badge warn">{{ friendRequests.length }}</span>
</div>
<div v-for="r in friendRequests" :key="r.requestId" class="friend-item">
<div class="friend-avatar">{{ r.nickname?.charAt(0) }}</div>
<div class="friend-info">
<div class="friend-name">{{ r.nickname }}</div>
</div>
<button class="btn xs" @click="acceptFriend(r.userId)">接受</button>
</div>
<div v-if="friendRequests.length === 0" style="color:var(--muted);font-size:12px">暂无好友请求</div>
</div>
<div class="card">
<div style="font-size:13px;margin-bottom:8px">好友列表</div>
<div v-for="f in friends" :key="f.id" class="friend-item" @click="startDM(f)">
<div class="friend-avatar"><img v-if="f.avatar" :src="f.avatar"><span v-else>{{ f.nickname?.charAt(0) }}</span></div>
<div class="friend-info">
<div class="friend-name">{{ f.nickname }}</div>
<div class="friend-status" :class="{online: f.online}">{{ f.online ? '在线' : '离线' }}</div>
</div>
</div>
<div v-if="friends.length === 0" style="color:var(--muted);font-size:12px">暂无好友，点击用户头像添加</div>
</div>
</div>

<!-- 聊天界面 -->
<div v-else-if="currentGroup" class="card chat-wrap" style="position:relative">
<div class="chat-header">
<div>
<h3>{{ currentGroup.name }}</h3>
<div class="chat-header-info">
<span>成员: {{ currentGroup.memberCount }}</span>
<span @click="showMembers = !showMembers">👥</span>
<span @click="showSearch = !showSearch">🔍</span>
</div>
</div>
<div>
<button class="btn sm" @click="doLeaveGroup">← 返回</button>
</div>
</div>

<!-- 置顶消息 -->
<div v-if="pinnedMessage" class="pinned-msg">
<span class="pin-icon">📌</span>
<span class="pin-content"><b>{{ pinnedMessage.senderNickname }}:</b> {{ pinnedMessage.content }}</span>
<button class="pin-close" @click="pinnedMessage = null">×</button>
</div>

<!-- 搜索面板 -->
<div v-if="showSearch" class="search-panel">
<input class="input" v-model="searchQuery" placeholder="搜索消息..." @keyup.enter="searchMessages">
</div>

<div class="chat-msgs" ref="msgsBox" @scroll="onMsgScroll">
<div class="msg-row" v-for="m in messages" :key="m.id" :class="{me: m.senderId === user.id}">
<div class="msg-avatar" @click="openUserMenu($event, m.senderId, m.senderNickname)">
<img v-if="m.senderAvatar" :src="m.senderAvatar"><span v-else>{{ m.senderNickname?.charAt(0) }}</span>
</div>
<div class="msg-content">
<div class="msg-bubble" :class="{in: m.senderId !== user.id, out: m.senderId === user.id, pinned: m.pinned}">
<div class="msg-nick" v-if="m.senderId !== user.id">{{ m.senderNickname }}</div>
<div v-if="m.forwarded" class="forward-tag">转发自 {{ m.originalSender }}</div>
<div v-if="m.replyInfo" class="reply-preview">
<span class="reply-nick">{{ m.replyInfo.senderNickname }}</span>: <span class="reply-content">{{ m.replyInfo.content }}</span>
</div>
<div v-html="renderMsg(m)"></div>
<div class="msg-time">{{ formatTime(m.createdAt) }}</div>
</div>
<div class="msg-actions">
<button class="msg-action" @click="replyTo = m">↩</button>
<button class="msg-action" @click="forwardMsg = m">↗</button>
<button class="msg-action" v-if="m.senderId === user.id" @click="recallMessage(m.id)">撤回</button>
<button class="msg-action" v-if="canPinMsg" @click="togglePin(m.id)">{{ m.pinned ? '取消置顶' : '置顶' }}</button>
</div>
</div>
</div>
</div>

<div class="chat-input-wrap">
<div v-if="replyTo" class="reply-bar">
<span>回复 <b>{{ replyTo.senderNickname }}</b>: {{ replyTo.content?.substring(0,30) }}...</span>
<button class="reply-close" @click="replyTo = null">×</button>
</div>
<div class="chat-input">
<div class="chat-tools">
<label class="tool-btn" v-if="hasPerm('file_upload')">📷<input type="file" accept="image/*,.txt" @change="uploadFile"></label>
<div style="position:relative">
<button class="tool-btn" @click="showEmoji = !showEmoji">😀</button>
<div class="emoji-picker" :class="{show: showEmoji}">
<span v-for="e in emojis" :key="e" class="emoji-item" @click="insertEmoji(e)">{{ e }}</span>
</div>
</div>
</div>
<textarea v-model="msgInput" placeholder="消息... @提及用户" @keyup.enter="doSendMsg" rows="1"></textarea>
<button class="btn" @click="doSendMsg">→</button>
</div>
</div>

<!-- 成员面板 -->
<div class="members-panel" :class="{show: showMembers}">
<h4>成员 ({{ members.length }})</h4>
<div class="members-list">
<div class="member-item" v-for="m in members" :key="m.id" @click="openUserMenu($event, m.id, m.nickname)">
<div class="member-avatar"><img v-if="m.avatar" :src="m.avatar"><span v-else>{{ m.nickname?.charAt(0) }}</span></div>
<div class="member-info">{{ m.nickname }}</div>
<div class="member-status" :class="{offline: !m.isOnline}"></div>
</div>
</div>
</div>
</div>
</div>

<!-- 管理面板 -->
<div class="modal-mask" v-if="showAdmin" @click.self="showAdmin = false">
<div class="modal" style="max-width:500px">
<div class="modal-header">
<h3>管理面板</h3>
<button class="modal-close" @click="showAdmin = false">×</button>
</div>
<div class="modal-body">
<div class="admin-tabs">
<button class="admin-tab" :class="{active: adminTab === 'users'}" @click="adminTab = 'users'; loadUsers()">用户</button>
<button class="admin-tab" :class="{active: adminTab === 'groups'}" @click="adminTab = 'groups'; loadAllGroups()">频道</button>
<button class="admin-tab" :class="{active: adminTab === 'words'}" @click="adminTab = 'words'; loadWords()">敏感词</button>
<button class="admin-tab" :class="{active: adminTab === 'stats'}" @click="adminTab = 'stats'; loadStats()">统计</button>
</div>

<!-- 用户管理 -->
<div class="admin-section" :class="{active: adminTab === 'users'}">
<div class="card" v-if="hasPerm('user_create')">
<input class="input" v-model="newUser.uid" placeholder="UID (留空自动生成)" style="margin-bottom:8px">
<input class="input" v-model="newUser.nickname" placeholder="昵称" style="margin-bottom:8px">
<input class="input" v-model="newUser.password" type="password" placeholder="密码" style="margin-bottom:8px">
<button class="btn full" @click="doCreateUser" :disabled="createUserLoading">{{ createUserLoading ? '创建中...' : '创建用户' }}</button>
</div>
<div class="item-card" v-for="u in users" :key="u.id">
<div class="item-header">
<span class="item-title">{{ u.nickname }} <span class="badge" :class="u.status === 'banned' ? 'error' : (u.online ? 'success' : '')">{{ u.status === 'banned' ? '已封禁' : (u.online ? '在线' : '离线') }}</span></span>
</div>
<div class="item-info">{{ u.uid }}</div>
<div class="perm-list"><span class="perm-tag" v-for="p in (u.permissions || []).slice(0,5)" :key="p">{{ p }}</span></div>
<div style="display:flex;gap:4px;margin-top:6px;flex-wrap:wrap">
<button class="btn sm" v-if="hasPerm('user_ban') && u.status !== 'banned' && u.role !== 'admin'" @click="doBanUser(u.uid)">封禁</button>
<button class="btn sm" v-if="hasPerm('user_ban') && u.status === 'banned'" @click="doUnbanUser(u.uid)">解封</button>
<button class="btn sm" v-if="hasPerm('user_mute')" @click="doMuteUser(u.uid)">禁言</button>
<button class="btn sm" v-if="hasPerm('permission_grant') && u.role !== 'admin'" @click="openPermModal(u)">权限</button>
</div>
</div>
</div>

<!-- 频道管理 -->
<div class="admin-section" :class="{active: adminTab === 'groups'}">
<div class="card" v-if="hasPerm('group_create')">
<input class="input" v-model="newGroup.name" placeholder="频道名称" style="margin-bottom:8px">
<button class="btn full" @click="doCreateGroup" :disabled="createGroupLoading">{{ createGroupLoading ? '创建中...' : '创建频道' }}</button>
</div>
<div class="item-card" v-for="g in allGroups" :key="g.id">
<div class="item-header">
<span class="item-title">{{ g.name }}</span>
<span class="badge success">{{ g.memberCount }}人</span>
</div>
<div class="item-info">{{ g.id }}</div>
<div style="margin-top:6px;display:flex;gap:4px;flex-wrap:wrap">
<button class="btn sm danger" v-if="hasPerm('group_delete')" @click="doDeleteGroup(g.id)">删除</button>
<button class="btn sm" v-if="hasPerm('message_delete')" @click="doClearGroupMessages(g.id)">清空消息</button>
</div>
</div>
</div>

<!-- 敏感词管理 -->
<div class="admin-section" :class="{active: adminTab === 'words'}">
<div class="card" v-if="isAdmin">
<input class="input" v-model="newWord.word" placeholder="敏感词" style="margin-bottom:8px">
<input class="input" v-model="newWord.replacement" placeholder="替换为 (默认***)" style="margin-bottom:8px">
<button class="btn full" @click="doAddWord">添加</button>
</div>
<div class="item-card" v-for="w in words" :key="w.id">
<div class="item-header">
<span class="item-title">{{ w.word }}</span>
<button class="btn sm danger" v-if="isAdmin" @click="doDeleteWord(w.id)">删除</button>
</div>
<div class="item-info">替换为: {{ w.replacement }}</div>
</div>
</div>

<!-- 统计 -->
<div class="admin-section" :class="{active: adminTab === 'stats'}">
<div class="stats-grid">
<div class="stat-card"><div class="stat-value">{{ stats.users?.total || 0 }}</div><div class="stat-label">用户总数</div></div>
<div class="stat-card"><div class="stat-value">{{ stats.users?.online || 0 }}</div><div class="stat-label">在线用户</div></div>
<div class="stat-card"><div class="stat-value">{{ stats.groups?.total || 0 }}</div><div class="stat-label">频道总数</div></div>
<div class="stat-card"><div class="stat-value">{{ stats.messages?.total || 0 }}</div><div class="stat-label">消息总数</div></div>
</div>
</div>
</div>
</div>
</div>

<!-- 个人资料 -->
<div class="modal-mask" v-if="showProfile" @click.self="showProfile = false">
<div class="modal">
<div class="modal-header">
<h3>个人设置</h3>
<button class="modal-close" @click="showProfile = false">×</button>
</div>
<div class="modal-body">
<div class="profile-section">
<div class="profile-avatar" @click="uploadAvatar">
<img v-if="user.avatar" :src="user.avatar"><span v-else>{{ user.nickname?.charAt(0) }}</span>
<div class="avatar-edit">更换</div>
<input type="file" ref="avatarInput" accept="image/*" @change="doUploadAvatar" style="display:none">
</div>
</div>
<div style="margin-bottom:12px">
<label style="font-size:12px;color:var(--muted)">昵称</label>
<input class="input" v-model="profileForm.nickname" placeholder="昵称">
</div>
<div style="margin-bottom:12px">
<label style="font-size:12px;color:var(--muted)">新密码 (留空不修改)</label>
<input class="input" v-model="profileForm.newPassword" type="password" placeholder="新密码">
</div>
<div style="margin-bottom:12px">
<label style="font-size:12px;color:var(--muted)">确认密码</label>
<input class="input" v-model="profileForm.confirmPassword" type="password" placeholder="确认新密码">
</div>
<button class="btn full" @click="saveProfile" :disabled="profileLoading">{{ profileLoading ? '保存中...' : '保存' }}</button>
</div>
</div>
</div>

<!-- 权限管理 -->
<div class="modal-mask" v-if="showPermModal" @click.self="showPermModal = false">
<div class="modal">
<div class="modal-header">
<h3>权限管理 - {{ permTarget?.nickname }}</h3>
<button class="modal-close" @click="showPermModal = false">×</button>
</div>
<div class="modal-body">
<div v-for="p in allPermissions" :key="p.name" style="margin-bottom:6px">
<label style="display:flex;align-items:center;gap:8px;cursor:pointer">
<input type="checkbox" :checked="hasUserPerm(p.name)" @change="togglePerm(p.name)">
<span style="font-size:12px">{{ p.name }}</span>
</label>
</div>
<button class="btn full" @click="savePerms" style="margin-top:12px">保存</button>
</div>
</div>
</div>

<!-- 转发消息 -->
<div class="modal-mask" v-if="forwardMsg" @click.self="forwardMsg = null">
<div class="modal">
<div class="modal-header">
<h3>转发消息</h3>
<button class="modal-close" @click="forwardMsg = null">×</button>
</div>
<div class="modal-body">
<div v-for="g in groups" :key="g.id" class="item-card" style="cursor:pointer" @click="doForward(g.id)">
<span class="item-title">{{ g.name }}</span>
</div>
</div>
</div>
</div>

<!-- 用户菜单 -->
<div class="user-menu" v-if="userMenu.show" :style="{left: userMenu.x + 'px', top: userMenu.y + 'px'}" @click.stop>
<div class="user-menu-header">
<div class="msg-avatar"><img v-if="userMenu.avatar" :src="userMenu.avatar"><span v-else>{{ userMenu.nickname?.charAt(0) }}</span></div>
<div>
<div style="font-weight:500">{{ userMenu.nickname }}</div>
<div class="item-info" :style="{color: userMenu.online ? 'var(--success)' : 'var(--muted)'}">{{ userMenu.online ? '在线' : '离线' }}</div>
</div>
</div>
<button class="user-menu-item" @click="startDMFromMenu">私聊</button>
<button class="user-menu-item" @click="doAddFriend">添加好友</button>
<template v-if="canManageUser && userMenu.userId !== user.id">
<button class="user-menu-item" @click="doMuteUser(userMenu.uid)">禁言</button>
<button class="user-menu-item danger" @click="doBanUser(userMenu.uid)">封禁</button>
<button class="user-menu-item danger" @click="doKickUser(userMenu.uid)">踢出</button>
</template>
</div>

</div>
</div>

<!-- 图片预览 -->
<div class="image-preview" v-if="previewImageUrl" @click="previewImageUrl = null">
<img :src="previewImageUrl">
</div>

<script src="https://unpkg.com/vue@3/dist/vue.global.prod.js"></script>
<script>
const { createApp, ref, reactive, computed, onMounted, nextTick, watch } = Vue;

createApp({
setup() {
// 基础状态
const loggedIn = ref(false);
const user = ref({});
const token = ref('');
const theme = ref('dark');
const isAdmin = ref(false);
const userPerms = ref([]);
const mainTab = ref('channels');

// 频道
const groups = ref([]);
const currentGroup = ref(null);
const channelInput = ref('');
const channelLoading = ref(false);
const members = ref([]);
const showMembers = ref(false);

// 消息
const messages = ref([]);
const msgInput = ref('');
const msgsBox = ref(null);
const replyTo = ref(null);
const forwardMsg = ref(null);
const pinnedMessage = ref(null);
const showSearch = ref(false);
const searchQuery = ref('');

// 私聊
const dmTarget = ref(null);
const dmMessages = ref([]);
const dmInput = ref('');
const dmMsgsBox = ref(null);

// 好友
const friends = ref([]);
const friendRequests = ref([]);
const friendRequestCount = ref(0);

// 管理面板
const showAdmin = ref(false);
const adminTab = ref('users');
const users = ref([]);
const allGroups = ref([]);
const words = ref([]);
const stats = ref({});
const newUser = reactive({ uid: '', nickname: '', password: '' });
const createUserLoading = ref(false);
const newGroup = reactive({ name: '' });
const createGroupLoading = ref(false);
const newWord = reactive({ word: '', replacement: '***' });

// 个人资料
const showProfile = ref(false);
const profileForm = reactive({ nickname: '', newPassword: '', confirmPassword: '' });
const profileLoading = ref(false);
const avatarInput = ref(null);

// 权限
const showPermModal = ref(false);
const permTarget = ref(null);
const permTargetPerms = ref([]);
const allPermissions = ref([]);

// 用户菜单
const userMenu = reactive({ show: false, x: 0, y: 0, uid: '', userId: '', nickname: '', avatar: '', online: false });

// 登录
const loginForm = reactive({ uid: '', pwd: '' });
const loginError = ref('');
const loginLoading = ref(false);

// 表情
const showEmoji = ref(false);
const emojis = ['😀','😂','🤣','😊','😍','🥰','😘','😜','🤔','😎','👍','👎','❤️','🔥','🎉','👏','🙏','💪','🤝','👋'];

let ws = null;

// 计算属性
const canAccessAdmin = computed(() => isAdmin.value || userPerms.value.length > 0);
const canManageUser = computed(() => hasPerm('user_ban') || hasPerm('user_mute') || isAdmin.value);
const canPinMsg = computed(() => isAdmin.value || (currentGroup.value && currentGroup.value.ownerId === user.value.id));

// 权限检查
function hasPerm(name) {
  if (isAdmin.value) return true;
  return userPerms.value.includes(name);
}

function hasUserPerm(name) {
  return permTargetPerms.value.includes(name);
}

// API
const API = location.origin;

async function api(path, options = {}) {
  const headers = { 'Content-Type': 'application/json', ...options.headers };
  if (token.value) headers['Authorization'] = 'Bearer ' + token.value;
  try {
    const r = await fetch(API + path, { ...options, headers });
    const text = await r.text();
    try {
      const data = JSON.parse(text);
      if (!data.success && data.error && path.startsWith('/api/admin')) {
        alert('错误: ' + data.error);
      }
      return data;
    } catch (e) {
      return { success: false, error: 'Invalid JSON' };
    }
  } catch (e) {
    return { success: false, error: e.message };
  }
}

// 登录
async function doLogin() {
  if (!loginForm.uid || !loginForm.pwd) {
    loginError.value = '请输入UID和密码';
    return;
  }
  loginLoading.value = true;
  loginError.value = '';
  try {
    const r = await fetch(API + '/api/auth/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ uid: loginForm.uid.toUpperCase(), password: loginForm.pwd })
    });
    const d = await r.json();
    if (d.success) {
      token.value = d.data.token;
      user.value = d.data.user;
      isAdmin.value = d.data.user.role === 'admin';
      userPerms.value = d.data.user.permissions || [];
      profileForm.nickname = d.data.user.nickname;
      localStorage.setItem('t', token.value);
      localStorage.setItem('u', JSON.stringify(user.value));
      loggedIn.value = true;
      connectWS();
      loadGroups();
      loadFriendRequests();
    } else {
      loginError.value = d.error || '登录失败';
    }
  } catch (e) {
    loginError.value = '网络错误';
  }
  loginLoading.value = false;
}

function doLogout() {
  api('/api/auth/logout', { method: 'POST' });
  localStorage.clear();
  token.value = '';
  user.value = {};
  loggedIn.value = false;
  isAdmin.value = false;
  userPerms.value = [];
  if (ws) ws.close();
}

// 频道
async function loadGroups() {
  const d = await api('/api/groups');
  if (d.success) groups.value = d.data;
}

async function doEnterChannel() {
  if (!channelInput.value.trim()) return;
  channelLoading.value = true;
  const d = await api('/api/groups/enter', {
    method: 'POST',
    body: JSON.stringify({ name: channelInput.value.trim() })
  });
  channelLoading.value = false;
  if (d.success) {
    channelInput.value = '';
    loadGroups();
  } else {
    alert(d.error || '进入失败');
  }
}

async function doJoinGroup(id) {
  const d = await api('/api/groups/' + id);
  if (d.success) {
    currentGroup.value = d.data;
    loadMessages();
    loadMembers();
  } else {
    alert(d.error || '加入失败');
  }
}

function doLeaveGroup() {
  currentGroup.value = null;
  messages.value = [];
  members.value = [];
  showMembers.value = false;
}

async function loadMembers() {
  if (!currentGroup.value) return;
  const d = await api('/api/groups/' + currentGroup.value.id + '/members');
  if (d.success) members.value = d.data;
}

// 消息
async function loadMessages() {
  if (!currentGroup.value) return;
  const d = await api('/api/messages/group/' + currentGroup.value.id);
  if (d.success) {
    messages.value = d.data;
    // 找置顶消息
    const pinned = messages.value.find(m => m.pinned);
    if (pinned) pinnedMessage.value = pinned;
    nextTick(scrollToBottom);
  }
}

async function doSendMsg() {
  if (!msgInput.value.trim() || !currentGroup.value) return;
  const content = msgInput.value;
  msgInput.value = '';
  
  const body = { content, groupId: currentGroup.value.id };
  if (replyTo.value) {
    body.reply_to = replyTo.value.id;
    replyTo.value = null;
  }
  
  const d = await api('/api/messages', {
    method: 'POST',
    body: JSON.stringify(body)
  });
  
  if (!d.success) {
    msgInput.value = content;
    alert(d.error || '发送失败');
  }
}

async function uploadFile(e) {
  const file = e.target.files[0];
  if (!file || !currentGroup.value) return;
  
  const formData = new FormData();
  formData.append('file', file);
  
  try {
    const r = await fetch(API + '/api/messages/file/' + currentGroup.value.id, {
      method: 'POST',
      headers: { 'Authorization': 'Bearer ' + token.value },
      body: formData
    });
    const d = await r.json();
    if (!d.success) alert(d.error || '上传失败');
  } catch (e) {
    alert('上传失败: ' + e.message);
  }
  e.target.value = '';
}

async function recallMessage(id) {
  if (!confirm('确定撤回该消息?')) return;
  const d = await api('/api/messages/' + id + '/recall', { method: 'POST' });
  if (d.success) {
    const idx = messages.value.findIndex(m => m.id === id);
    if (idx >= 0) messages.value.splice(idx, 1);
  } else {
    alert(d.error || '撤回失败');
  }
}

async function togglePin(id) {
  const d = await api('/api/messages/' + id + '/pin', { method: 'POST' });
  if (d.success) {
    loadMessages();
  } else {
    alert(d.error || '操作失败');
  }
}

async function doForward(targetGroupId) {
  if (!forwardMsg.value) return;
  const d = await api('/api/messages/' + forwardMsg.value.id + '/forward', {
    method: 'POST',
    body: JSON.stringify({ target_group_id: targetGroupId })
  });
  if (d.success) {
    alert('转发成功');
  } else {
    alert(d.error || '转发失败');
  }
  forwardMsg.value = null;
}

async function searchMessages() {
  if (!searchQuery.value.trim() || !currentGroup.value) return;
  const d = await api('/api/messages/group/' + currentGroup.value.id + '/search?q=' + encodeURIComponent(searchQuery.value));
  if (d.success && d.data.length > 0) {
    messages.value = d.data;
  } else {
    alert('未找到匹配消息');
  }
}

function renderMsg(m) {
  if (m.msgType === 'image') return '<img class="msg-img" src="' + m.content + '" onclick="window._previewImage(\'' + m.content + '\')">';
  if (m.msgType === 'file') return '<div class="msg-file"><span class="msg-file-icon">📄</span><div class="msg-file-info">' + (m.fileName || '文件') + '</div></div>';
  // 处理@提及
  let content = m.content;
  content = content.replace(/@([\u4e00-\u9fa5\w]+)/g, '<span class="msg-mention">@$1</span>');
  const el = document.createElement('div');
  el.textContent = content;
  return el.innerHTML.replace(/&lt;span class="msg-mention"&gt;/g, '<span class="msg-mention">').replace(/&lt;\/span&gt;/g, '</span>');
}

function formatTime(t) {
  return new Date(t).toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
}

function scrollToBottom() {
  if (msgsBox.value) msgsBox.value.scrollTop = msgsBox.value.scrollHeight;
}

function onMsgScroll() {
  // 可以实现加载更多历史消息
}

function insertEmoji(e) {
  msgInput.value += e;
  showEmoji.value = false;
}

function previewImage(url) {
  window._previewImage(url);
}

window._previewImage = (url) => {
  const app = document.querySelector('#app').__vue_app__;
  if (app) {
    const root = app._instance.proxy;
    root.previewImageUrl = url;
  }
};

const previewImageUrl = ref(null);

// WebSocket
function connectWS() {
  const proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
  ws = new WebSocket(proto + '//' + location.host + '/ws?token=' + token.value);
  
  ws.onmessage = (e) => {
    const m = JSON.parse(e.data);
    if (m.event === 'message' && m.data.groupId === currentGroup.value?.id) {
      messages.value.push(m.data);
      nextTick(scrollToBottom);
    }
    if (m.event === 'direct_message') {
      if (dmTarget.value && (m.data.senderId === dmTarget.value.id || m.data.receiverId === dmTarget.value.id)) {
        dmMessages.value.push(m.data);
        nextTick(scrollToBottom);
      } else if (m.data.senderId !== user.value.id) {
        // 通知
      }
    }
    if (m.event === 'message_recall' && m.data.groupId === currentGroup.value?.id) {
      const idx = messages.value.findIndex(msg => msg.id === m.data.id);
      if (idx >= 0) messages.value.splice(idx, 1);
    }
    if (m.event === 'message_pin' && m.data.groupId === currentGroup.value?.id) {
      loadMessages();
    }
    if (m.event === 'friend_request') {
      loadFriendRequests();
    }
  };
  
  ws.onclose = () => setTimeout(connectWS, 3000);
}

// 主题
function toggleTheme() {
  theme.value = theme.value === 'dark' ? 'light' : 'dark';
  document.documentElement.setAttribute('data-theme', theme.value === 'light' ? 'light' : '');
  localStorage.setItem('theme', theme.value);
}

// 用户菜单
function openUserMenu(e, userId, nickname) {
  e.stopPropagation();
  api('/api/users/' + userId).then(d => {
    if (d.success) {
      userMenu.online = d.data.online;
      userMenu.avatar = d.data.avatar;
    }
  });
  userMenu.show = true;
  userMenu.x = Math.min(e.clientX, window.innerWidth - 160);
  userMenu.y = Math.min(e.clientY, window.innerHeight - 180);
  userMenu.userId = userId;
  userMenu.nickname = nickname;
  userMenu.uid = userId;
}

function startDMFromMenu() {
  dmTarget.value = { id: userMenu.userId, nickname: userMenu.nickname, online: userMenu.online, avatar: userMenu.avatar };
  dmMessages.value = [];
  userMenu.show = false;
}

function closeDM() {
  dmTarget.value = null;
  dmMessages.value = [];
}

async function sendDM() {
  if (!dmInput.value.trim() || !dmTarget.value) return;
  const content = dmInput.value;
  dmInput.value = '';
  const d = await api('/api/direct/' + dmTarget.value.id, {
    method: 'POST',
    body: JSON.stringify({ content })
  });
  if (!d.success) {
    dmInput.value = content;
    alert(d.error || '发送失败');
  }
}

// 好友
async function loadFriends() {
  const d = await api('/api/friends');
  if (d.success) friends.value = d.data;
}

async function loadFriendRequests() {
  const d = await api('/api/friends/requests');
  if (d.success) {
    friendRequests.value = d.data;
    friendRequestCount.value = d.data.length;
  }
}

async function acceptFriend(userId) {
  const d = await api('/api/friends/' + userId + '/accept', { method: 'POST' });
  if (d.success) {
    loadFriendRequests();
    loadFriends();
  } else {
    alert(d.error || '失败');
  }
}

function doAddFriend() {
  api('/api/friends/' + userMenu.userId, { method: 'POST' }).then(d => {
    alert(d.success ? '好友请求已发送' : (d.error || '失败'));
  });
  userMenu.show = false;
}

function startDM(friend) {
  dmTarget.value = { id: friend.id, nickname: friend.nickname, online: friend.online, avatar: friend.avatar };
  dmMessages.value = [];
}

// 个人资料
function uploadAvatar() {
  avatarInput.value?.click();
}

async function doUploadAvatar(e) {
  const file = e.target.files[0];
  if (!file) return;
  
  const formData = new FormData();
  formData.append('avatar', file);
  
  try {
    const r = await fetch(API + '/api/users/avatar', {
      method: 'POST',
      headers: { 'Authorization': 'Bearer ' + token.value },
      body: formData
    });
    const d = await r.json();
    if (d.success) {
      user.value.avatar = d.data.avatar;
      localStorage.setItem('u', JSON.stringify(user.value));
    } else {
      alert(d.error || '上传失败');
    }
  } catch (e) {
    alert('上传失败');
  }
  e.target.value = '';
}

async function saveProfile() {
  if (profileForm.newPassword && profileForm.newPassword !== profileForm.confirmPassword) {
    alert('两次密码不一致');
    return;
  }
  profileLoading.value = true;
  
  // 更新昵称
  if (profileForm.nickname !== user.value.nickname) {
    await api('/api/users/profile', {
      method: 'PUT',
      body: JSON.stringify({ nickname: profileForm.nickname })
    });
    user.value.nickname = profileForm.nickname;
  }
  
  // 更新密码
  if (profileForm.newPassword) {
    const d = await api('/api/users/password', {
      method: 'PUT',
      body: JSON.stringify({ new_password: profileForm.newPassword })
    });
    if (!d.success) {
      alert(d.error || '密码修改失败');
      profileLoading.value = false;
      return;
    }
  }
  
  localStorage.setItem('u', JSON.stringify(user.value));
  profileLoading.value = false;
  showProfile.value = false;
  alert('保存成功');
}

// 管理面板
function openAdmin() {
  showAdmin.value = true;
  loadAllPermissions();
  if (adminTab.value === 'users') loadUsers();
}

async function loadAllPermissions() {
  const d = await api('/api/admin/permissions');
  if (d.success) allPermissions.value = d.data;
}

async function loadUsers() {
  const d = await api('/api/admin/users');
  if (d.success) users.value = d.data;
}

async function doCreateUser() {
  if (!newUser.nickname || !newUser.password) {
    alert('请填写昵称和密码');
    return;
  }
  createUserLoading.value = true;
  const d = await api('/api/admin/users', { method: 'POST', body: JSON.stringify(newUser) });
  createUserLoading.value = false;
  if (d.success) {
    newUser.uid = '';
    newUser.nickname = '';
    newUser.password = '';
    loadUsers();
    alert('创建成功');
  } else {
    alert(d.error || '创建失败');
  }
}

async function doBanUser(uid) {
  if (!confirm('确定封禁该用户?')) return;
  const d = await api('/api/admin/users/' + uid + '/ban', { method: 'PUT' });
  alert(d.success ? '已封禁' : (d.error || '失败'));
  loadUsers();
  userMenu.show = false;
}

async function doUnbanUser(uid) {
  const d = await api('/api/admin/users/' + uid + '/unban', { method: 'PUT' });
  alert(d.success ? '已解封' : (d.error || '失败'));
  loadUsers();
}

async function doMuteUser(uid) {
  const d = await api('/api/admin/users/' + uid + '/mute', {
    method: 'PUT',
    body: JSON.stringify({ duration_minutes: 30 })
  });
  alert(d.success ? '已禁言30分钟' : (d.error || '失败'));
  loadUsers();
  userMenu.show = false;
}

async function doKickUser(uid) {
  if (!confirm('确定踢出该用户?')) return;
  const d = await api('/api/admin/users/' + uid + '/kick', { method: 'PUT' });
  alert(d.success ? '已踢出' : (d.error || '失败'));
  loadUsers();
  userMenu.show = false;
}

async function loadAllGroups() {
  const d = await api('/api/admin/groups');
  if (d.success) allGroups.value = d.data;
}

async function doCreateGroup() {
  if (!newGroup.name.trim()) {
    alert('请输入频道名称');
    return;
  }
  createGroupLoading.value = true;
  const d = await api('/api/groups', { method: 'POST', body: JSON.stringify({ name: newGroup.name.trim() }) });
  createGroupLoading.value = false;
  if (d.success) {
    alert('频道创建成功');
    newGroup.name = '';
    loadAllGroups();
    loadGroups();
  } else {
    alert(d.error || '创建失败');
  }
}

async function doDeleteGroup(id) {
  if (!confirm('确定删除该频道?')) return;
  const d = await api('/api/admin/groups/' + id, { method: 'DELETE' });
  if (d.success) loadAllGroups();
}

async function doClearGroupMessages(id) {
  if (!confirm('确定清空该频道所有消息?')) return;
  const d = await api('/api/messages/group/' + id, { method: 'DELETE' });
  alert(d.success ? '消息已清空' : (d.error || '失败'));
}

async function loadWords() {
  const d = await api('/api/admin/sensitive-words');
  if (d.success) words.value = d.data;
}

async function doAddWord() {
  if (!newWord.word) { alert('请输入敏感词'); return; }
  const d = await api('/api/admin/sensitive-words', { method: 'POST', body: JSON.stringify(newWord) });
  if (d.success) {
    newWord.word = '';
    newWord.replacement = '***';
    loadWords();
  } else {
    alert(d.error || '失败');
  }
}

async function doDeleteWord(id) {
  const d = await api('/api/admin/sensitive-words/' + id, { method: 'DELETE' });
  if (d.success) loadWords();
}

async function loadStats() {
  const d = await api('/api/admin/statistics');
  if (d.success) stats.value = d.data;
}

// 权限管理
function openPermModal(u) {
  permTarget.value = u;
  permTargetPerms.value = [...(u.permissions || [])];
  showPermModal.value = true;
}

function togglePerm(name) {
  const idx = permTargetPerms.value.indexOf(name);
  if (idx >= 0) permTargetPerms.value.splice(idx, 1);
  else permTargetPerms.value.push(name);
}

async function savePerms() {
  if (!permTarget.value) return;
  const currentPerms = permTarget.value.permissions || [];
  const toGrant = permTargetPerms.value.filter(p => !currentPerms.includes(p));
  const toRevoke = currentPerms.filter(p => !permTargetPerms.value.includes(p));
  
  for (const p of toGrant) {
    await api('/api/admin/users/' + permTarget.value.uid + '/permissions', {
      method: 'POST',
      body: JSON.stringify({ permission_name: p })
    });
  }
  for (const p of toRevoke) {
    await api('/api/admin/users/' + permTarget.value.uid + '/permissions', {
      method: 'DELETE',
      body: JSON.stringify({ permission_name: p })
    });
  }
  
  showPermModal.value = false;
  loadUsers();
  alert('权限已更新');
}

// 初始化
onMounted(() => {
  document.addEventListener('click', () => userMenu.show = false);
  
  const savedTheme = localStorage.getItem('theme');
  if (savedTheme === 'light') {
    theme.value = 'light';
    document.documentElement.setAttribute('data-theme', 'light');
  }
  
  const t = localStorage.getItem('t');
  const u = localStorage.getItem('u');
  if (t && u) {
    token.value = t;
    try {
      user.value = JSON.parse(u);
      profileForm.nickname = user.value.nickname;
      api('/api/auth/me').then(me => {
        if (me.success) {
          user.value = me.data;
          profileForm.nickname = me.data.nickname;
          isAdmin.value = me.data.role === 'admin';
          userPerms.value = me.data.permissions || [];
          loggedIn.value = true;
          connectWS();
          loadGroups();
          loadFriendRequests();
        } else {
          localStorage.clear();
          token.value = '';
          user.value = {};
        }
      });
    } catch (e) {
      localStorage.clear();
    }
  }
  
  setTimeout(() => {
    document.getElementById('loading').style.display = 'none';
    document.getElementById('app').classList.add('loaded');
  }, 100);
});

return {
  loggedIn, user, token, theme, isAdmin, userPerms, mainTab,
  groups, currentGroup, channelInput, channelLoading, members, showMembers,
  messages, msgInput, msgsBox, replyTo, forwardMsg, pinnedMessage, showSearch, searchQuery,
  dmTarget, dmMessages, dmInput, dmMsgsBox,
  friends, friendRequests, friendRequestCount,
  showAdmin, adminTab, users, allGroups, words, stats,
  newUser, createUserLoading, newGroup, createGroupLoading, newWord,
  showProfile, profileForm, profileLoading, avatarInput,
  showPermModal, permTarget, permTargetPerms, allPermissions,
  userMenu, showEmoji, emojis, previewImageUrl,
  canAccessAdmin, canManageUser, canPinMsg, hasPerm, hasUserPerm,
  doLogin, doLogout, loadGroups, doEnterChannel, doJoinGroup, doLeaveGroup, loadMembers,
  loadMessages, doSendMsg, uploadFile, recallMessage, togglePin, doForward, searchMessages, renderMsg, formatTime, insertEmoji, previewImage,
  openUserMenu, startDMFromMenu, closeDM, sendDM,
  loadFriends, loadFriendRequests, acceptFriend, doAddFriend, startDM,
  uploadAvatar, doUploadAvatar, saveProfile,
  toggleTheme, openAdmin,
  loadUsers, doCreateUser, doBanUser, doUnbanUser, doMuteUser, doKickUser,
  loadAllGroups, doCreateGroup, doDeleteGroup, doClearGroupMessages, loadWords, doAddWord, doDeleteWord, loadStats,
  openPermModal, togglePerm, savePerms,
  onMsgScroll
};
}
}).mount('#app');
</script>
</body>
</html>
"##;

pub const MANIFEST_JSON: &str = r##"{"name":"ARCANUM","short_name":"ARCANUM","start_url":"/","display":"standalone","background_color":"#000000","theme_color":"#000000","icons":[{"src":"/icon-192.png","sizes":"192x192","type":"image/png"}]}"##;

pub const SERVICE_WORKER_JS: &str = r##"const CACHE_NAME='arcanum-v1';self.addEventListener('install',e=>e.waitUntil(caches.open(CACHE_NAME).then(c=>c.addAll(['/', '/manifest.json']))));self.addEventListener('fetch',e=>e.respondWith(caches.match(e.request).then(r=>r||fetch(e.request))));"##;
