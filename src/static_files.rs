pub const INDEX_HTML: &str = r##"<!DOCTYPE html>
<html lang="zh">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1.0,maximum-scale=1.0,user-scalable=no">
<meta name="theme-color" content="#000000">
<meta name="apple-mobile-web-app-capable" content="yes">
<meta name="description" content="ARCANUM">
<link rel="manifest" href="/manifest.json">
<title>ARCANUM</title>
<style>
/* === 赛博朋克主题 === */
:root{--bg:#0a0a0f;--bg2:#12121a;--card:#16161f;--card2:#1a1a25;--text:#e0e0e8;--muted:#6a6a7a;--accent:#00f0ff;--accent2:#ff00aa;--border:#2a2a3a;--error:#ff3366;--success:#00ff88;--warn:#ffaa00;--glow:0 0 10px var(--accent),0 0 20px rgba(0,240,255,.3)}
[data-theme="light"]{--bg:#f0f0f5;--bg2:#e8e8f0;--card:#fff;--card2:#f8f8fc;--text:#1a1a2e;--muted:#6a6a7a;--accent:#0088aa;--accent2:#aa0066;--border:#d0d0da;--glow:none}
*{margin:0;padding:0;box-sizing:border-box}
body{background:var(--bg);color:var(--text);font-family:'Segoe UI',system-ui,sans-serif;min-height:100vh;min-height:100dvh;line-height:1.5}
.container{max-width:540px;margin:0 auto;padding:12px;min-height:100vh}
.hidden{display:none!important}

/* === 赛博朋克效果 === */
.cyber-border{border:1px solid var(--border);position:relative}
.cyber-border::before{content:'';position:absolute;top:-1px;left:10px;right:10px;height:1px;background:linear-gradient(90deg,transparent,var(--accent),transparent)}
.cyber-glow{box-shadow:var(--glow)}
.cyber-text{background:linear-gradient(90deg,var(--accent),var(--accent2));-webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text}
.scanline{position:fixed;inset:0;background:repeating-linear-gradient(0deg,rgba(0,0,0,.03),rgba(0,0,0,.03) 1px,transparent 1px,transparent 2px);pointer-events:none;z-index:9999}

/* === 组件 === */
.btn{padding:10px 20px;background:transparent;border:1px solid var(--accent);color:var(--accent);border-radius:4px;font-size:14px;cursor:pointer;transition:all .2s;position:relative;overflow:hidden}
.btn:hover{background:var(--accent);color:#000;box-shadow:var(--glow)}
.btn:disabled{opacity:.5;cursor:not-allowed}
.btn.full{width:100%}
.btn.sm{padding:6px 12px;font-size:12px}
.btn.danger{border-color:var(--error);color:var(--error)}
.btn.danger:hover{background:var(--error);color:#fff}
.btn.success{border-color:var(--success);color:var(--success)}
.btn.success:hover{background:var(--success);color:#000}

.input{width:100%;padding:12px;background:var(--bg2);border:1px solid var(--border);color:var(--text);border-radius:4px;font-size:14px;outline:none;transition:border-color .2s}
.input:focus{border-color:var(--accent);box-shadow:0 0 0 2px rgba(0,240,255,.1)}

.card{background:var(--card);border:1px solid var(--border);border-radius:8px;padding:16px;margin-bottom:12px}
.card-header{display:flex;justify-content:space-between;align-items:center;margin-bottom:12px;padding-bottom:12px;border-bottom:1px solid var(--border)}

.err{color:var(--error);font-size:13px;padding:8px;background:rgba(255,51,102,.1);border-radius:4px;margin-top:8px}
.success{color:var(--success);font-size:13px;padding:8px;background:rgba(0,255,136,.1);border-radius:4px;margin-top:8px}

/* === 登录页 === */
.login-logo{font-size:32px;font-weight:700;text-align:center;margin:60px 0 40px;letter-spacing:8px}
.login-form{display:flex;flex-direction:column;gap:12px}

/* === 主页 === */
.header{display:flex;justify-content:space-between;align-items:center;padding:8px 0;margin-bottom:12px}
.header h1{font-size:18px;font-weight:600}
.header-actions{display:flex;gap:8px}

.sidebar{display:flex;flex-direction:column;gap:8px;margin-bottom:16px}
.channel-list{display:flex;flex-direction:column;gap:8px}
.channel-card{background:var(--card);border:1px solid var(--border);border-radius:8px;padding:16px;cursor:pointer;transition:all .2s}
.channel-card:hover{border-color:var(--accent);box-shadow:0 0 10px rgba(0,240,255,.1)}
.channel-card h3{font-size:15px;margin-bottom:4px}
.channel-card p{font-size:12px;color:var(--muted)}

/* === 聊天 === */
.chat-header{display:flex;justify-content:space-between;align-items:center;padding:12px;background:var(--card);border-bottom:1px solid var(--border)}
.chat-msgs{flex:1;overflow-y:auto;padding:12px;display:flex;flex-direction:column;gap:12px}
.msg-row{display:flex;gap:10px}
.msg-row.me{flex-direction:row-reverse}
.msg-avatar{width:36px;height:36px;border-radius:6px;background:linear-gradient(135deg,var(--accent),var(--accent2));display:flex;align-items:center;justify-content:center;font-size:14px;font-weight:600;color:#000;flex-shrink:0;overflow:hidden}
.msg-avatar img{width:100%;height:100%;object-fit:cover}
.msg-bubble{max-width:70%;padding:10px 14px;border-radius:12px;font-size:14px}
.msg-bubble.in{background:var(--card);border:1px solid var(--border)}
.msg-bubble.out{background:var(--accent);color:#000}
.msg-nick{font-size:11px;color:var(--accent);margin-bottom:2px}
.msg-time{font-size:10px;color:var(--muted);margin-top:4px;text-align:right}
.msg-input-wrap{display:flex;gap:8px;padding:12px;background:var(--bg);border-top:1px solid var(--border)}
.msg-input-wrap textarea{flex:1;padding:10px 14px;background:var(--card);border:1px solid var(--border);color:var(--text);border-radius:20px;font-size:14px;outline:none;resize:none;max-height:80px}

/* === 管理面板 === */
.admin-tabs{display:flex;gap:4px;margin-bottom:12px;flex-wrap:wrap}
.admin-tab{flex:1;min-width:60px;padding:10px;background:transparent;border:1px solid var(--border);color:var(--muted);border-radius:4px;font-size:12px;cursor:pointer;text-align:center}
.admin-tab.active{border-color:var(--accent);color:var(--accent)}
.admin-section{display:none}
.admin-section.active{display:block}

.item-card{background:var(--card2);border:1px solid var(--border);border-radius:8px;padding:12px;margin-bottom:8px}
.item-header{display:flex;justify-content:space-between;align-items:center}
.item-title{font-size:14px;font-weight:500}
.item-info{font-size:11px;color:var(--muted);margin-top:4px}

.stats-grid{display:grid;grid-template-columns:repeat(2,1fr);gap:12px}
.stat-card{background:linear-gradient(135deg,rgba(0,240,255,.05),rgba(255,0,170,.05));border:1px solid var(--border);border-radius:8px;padding:16px;text-align:center}
.stat-value{font-size:24px;font-weight:600;color:var(--accent)}
.stat-label{font-size:11px;color:var(--muted);margin-top:4px}

/* === 模态框 === */
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,.8);display:flex;align-items:center;justify-content:center;z-index:1000;padding:12px}
.modal{background:var(--card);border:1px solid var(--border);border-radius:12px;max-width:400px;width:100%;max-height:90vh;overflow-y:auto}
.modal-header{display:flex;justify-content:space-between;align-items:center;padding:16px;border-bottom:1px solid var(--border)}
.modal-header h3{font-size:16px;font-weight:600}
.modal-close{background:none;border:none;color:var(--muted);font-size:24px;cursor:pointer}
.modal-body{padding:16px}

/* === 用户菜单 === */
.user-menu{position:fixed;background:var(--card);border:1px solid var(--border);border-radius:8px;padding:8px;z-index:1001;min-width:180px;box-shadow:0 4px 20px rgba(0,0,0,.5)}
.user-menu-header{padding:8px;border-bottom:1px solid var(--border);margin-bottom:8px;display:flex;align-items:center;gap:10px}
.user-menu-item{display:block;width:100%;padding:8px 12px;background:none;border:none;color:var(--text);text-align:left;cursor:pointer;border-radius:4px}
.user-menu-item:hover{background:var(--bg2)}

/* === 徽章 === */
.badge{display:inline-block;padding:2px 8px;border-radius:10px;font-size:10px;font-weight:500}
.badge.accent{background:rgba(0,240,255,.2);color:var(--accent)}
.badge.warn{background:rgba(255,170,0,.2);color:var(--warn)}
.badge.error{background:rgba(255,51,102,.2);color:var(--error)}
.badge.success{background:rgba(0,255,136,.2);color:var(--success)}

/* === 响应式 === */
@media(max-width:480px){.container{padding:8px}.login-logo{font-size:24px;margin:40px 0 30px}}
</style>
</head>
<body class="scanline" x-data="app()" x-init="init()">
<div class="container">

<!-- 登录页 -->
<div x-show="!loggedIn" x-cloak>
<div class="login-logo cyber-text">ARCANUM</div>
<div class="card">
<form class="login-form" @submit.prevent="login()">
<input class="input" placeholder="UID" x-model="loginForm.uid" autocapitalize="characters">
<input class="input" type="password" placeholder="密码" x-model="loginForm.pwd" @keydown.enter="login()">
<div class="err" x-show="loginError" x-text="loginError"></div>
<button class="btn full cyber-glow" type="submit" :disabled="loginLoading">
<template x-if="!loginLoading">进入</template>
<template x-if="loginLoading">登录中...</template>
</button>
</form>
</div>
</div>

<!-- 主页 -->
<div x-show="loggedIn" x-cloak>
<!-- 头部 -->
<div class="header">
<div>
<h1 class="cyber-text" x-text="user?.nickname || 'ARCANUM'"></h1>
<div style="font-size:11px;color:var(--muted)" x-text="user?.uid"></div>
</div>
<div class="header-actions">
<button class="btn sm" @click="toggleTheme()" x-text="theme === 'dark' ? '☀' : '🌙'"></button>
<button class="btn sm" @click="showAdmin = true" x-show="isAdmin">⚙</button>
<button class="btn sm danger" @click="logout()">退出</button>
</div>
</div>

<!-- 频道列表 -->
<div x-show="!currentGroup">
<div class="sidebar">
<input class="input" placeholder="输入频道名进入或创建" x-model="channelInput" @keydown.enter="enterChannel()">
<button class="btn full" @click="enterChannel()">进入频道</button>
</div>
<div class="channel-list">
<template x-for="g in groups" :key="g.id">
<div class="channel-card" @click="joinGroup(g.id)">
<h3 x-text="g.name"></h3>
<p x-text="'成员: ' + g.memberCount"></p>
</div>
</template>
</div>
</div>

<!-- 聊天界面 -->
<div x-show="currentGroup" class="card" style="display:flex;flex-direction:column;height:calc(100vh - 100px)">
<div class="chat-header">
<div>
<h3 x-text="currentGroup?.name"></h3>
<div style="font-size:11px;color:var(--muted)" x-text="'成员: ' + (currentGroup?.memberCount || 0)"></div>
</div>
<div>
<button class="btn sm" @click="showGroupInfo = true">ℹ</button>
<button class="btn sm" @click="leaveGroup()">←</button>
</div>
</div>
<div class="chat-msgs" id="msgs" @scroll="onScroll($event)">
<template x-for="m in messages" :key="m.id">
<div class="msg-row" :class="{me: m.senderId === user?.id}">
<div class="msg-avatar" @click="showUserMenu($event, m.senderId, m.senderNickname)">
<template x-if="m.senderAvatar">
<img :src="m.senderAvatar">
</template>
<template x-if="!m.senderAvatar" x-text="m.senderNickname?.charAt(0)"></template>
</div>
<div class="msg-bubble" :class="m.senderId === user?.id ? 'out' : 'in'">
<div class="msg-nick" x-text="m.senderNickname"></div>
<div x-html="renderMessage(m)"></div>
<div class="msg-time" x-text="formatTime(m.createdAt)"></div>
</div>
</div>
</template>
</div>
<div class="msg-input-wrap">
<textarea placeholder="消息..." x-model="msgInput" @keydown.enter.prevent="sendMessage()" rows="1"></textarea>
<button class="btn" @click="sendMessage()">→</button>
</div>
</div>
</div>
</div>

<!-- 管理面板 -->
<div class="modal-overlay" x-show="showAdmin" x-cloak @click.self="showAdmin = false">
<div class="modal" style="max-width:540px">
<div class="modal-header">
<h3>管理面板</h3>
<button class="modal-close" @click="showAdmin = false">×</button>
</div>
<div class="modal-body">
<div class="admin-tabs">
<button class="admin-tab" :class="{active: adminTab === 'users'}" @click="adminTab = 'users'">用户</button>
<button class="admin-tab" :class="{active: adminTab === 'groups'}" @click="adminTab = 'groups'">频道</button>
<button class="admin-tab" :class="{active: adminTab === 'words'}" @click="adminTab = 'words'">敏感词</button>
<button class="admin-tab" :class="{active: adminTab === 'stats'}" @click="adminTab = 'stats'">统计</button>
</div>

<!-- 用户管理 -->
<div class="admin-section" :class="{active: adminTab === 'users'}">
<div class="card">
<h4 style="margin-bottom:12px">创建用户</h4>
<input class="input" placeholder="UID" x-model="newUser.uid" style="margin-bottom:8px">
<input class="input" placeholder="昵称" x-model="newUser.nickname" style="margin-bottom:8px">
<input class="input" type="password" placeholder="密码" x-model="newUser.password" style="margin-bottom:8px">
<button class="btn full" @click="createUser()">创建</button>
</div>
<template x-for="u in users" :key="u.id">
<div class="item-card">
<div class="item-header">
<span class="item-title" x-text="u.nickname"></span>
<span class="badge" :class="u.online ? 'success' : ''" x-text="u.online ? '在线' : '离线'"></span>
</div>
<div class="item-info" x-text="u.uid"></div>
<div style="display:flex;gap:4px;margin-top:8px">
<button class="btn sm" @click="banUser(u.uid)" x-show="u.status !== 'banned'">封禁</button>
<button class="btn sm" @click="unbanUser(u.uid)" x-show="u.status === 'banned'">解封</button>
<button class="btn sm warn" @click="muteUser(u.uid)">禁言</button>
</div>
</div>
</template>
</div>

<!-- 频道管理 -->
<div class="admin-section" :class="{active: adminTab === 'groups'}">
<template x-for="g in allGroups" :key="g.id">
<div class="item-card">
<div class="item-header">
<span class="item-title" x-text="g.name"></span>
<span class="badge accent" x-text="'成员: ' + g.memberCount"></span>
</div>
<div class="item-info" x-text="g.id"></div>
<div style="margin-top:8px">
<button class="btn sm danger" @click="deleteGroup(g.id)">删除</button>
</div>
</div>
</template>
</div>

<!-- 敏感词管理 -->
<div class="admin-section" :class="{active: adminTab === 'words'}">
<div class="card">
<h4 style="margin-bottom:12px">添加敏感词</h4>
<input class="input" placeholder="敏感词" x-model="newWord.word" style="margin-bottom:8px">
<input class="input" placeholder="替换为" x-model="newWord.replacement" style="margin-bottom:8px">
<button class="btn full" @click="addWord()">添加</button>
</div>
<template x-for="w in sensitiveWords" :key="w.id">
<div class="item-card">
<div class="item-header">
<span class="item-title" x-text="w.word"></span>
<button class="btn sm danger" @click="deleteWord(w.id)">删除</button>
</div>
<div class="item-info" x-text="'替换为: ' + w.replacement"></div>
</div>
</template>
</div>

<!-- 统计 -->
<div class="admin-section" :class="{active: adminTab === 'stats'}">
<div class="stats-grid">
<div class="stat-card">
<div class="stat-value" x-text="stats.users?.total || 0"></div>
<div class="stat-label">用户总数</div>
</div>
<div class="stat-card">
<div class="stat-value" x-text="stats.users?.online || 0"></div>
<div class="stat-label">在线用户</div>
</div>
<div class="stat-card">
<div class="stat-value" x-text="stats.groups?.total || 0"></div>
<div class="stat-label">频道总数</div>
</div>
<div class="stat-card">
<div class="stat-value" x-text="stats.messages?.total || 0"></div>
<div class="stat-label">消息总数</div>
</div>
</div>
</div>
</div>
</div>
</div>

<!-- 用户菜单 -->
<div class="user-menu" x-show="userMenu.show" x-cloak :style="{left: userMenu.x + 'px', top: userMenu.y + 'px'}" @click.away="userMenu.show = false">
<div class="user-menu-header">
<div class="msg-avatar" x-text="userMenu.nickname?.charAt(0)"></div>
<div>
<div style="font-weight:500" x-text="userMenu.nickname"></div>
<div style="font-size:11px;color:var(--muted)" x-text="userMenu.uid"></div>
</div>
</div>
<button class="user-menu-item" @click="addFriend(userMenu.userId)">添加好友</button>
<button class="user-menu-item" @click="openDirectChat(userMenu.userId, userMenu.nickname)">私聊</button>
<template x-if="isAdmin">
<div>
<button class="user-menu-item" @click="muteUser(userMenu.uid)">禁言</button>
<button class="user-menu-item" style="color:var(--error)" @click="banUser(userMenu.uid)">封禁</button>
</div>
</template>
</div>

</div>

<!-- Alpine.js -->
<script defer src="https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js"></script>
<script>
function app(){
return {
// 状态
loggedIn: false,
user: null,
token: '',
theme: 'dark',
currentGroup: null,
groups: [],
messages: [],
msgInput: '',
channelInput: '',
ws: null,
isAdmin: false,
showAdmin: false,
adminTab: 'users',
users: [],
allGroups: [],
sensitiveWords: [],
stats: {},
newUser: {uid: '', nickname: '', password: ''},
newWord: {word: '', replacement: '***'},
loginForm: {uid: '', pwd: ''},
loginError: '',
loginLoading: false,
userMenu: {show: false, x: 0, y: 0, uid: '', userId: '', nickname: ''},

// 初始化
async init(){
console.log('ARCANUM initializing...');
this.loadTheme();
const t = localStorage.getItem('t');
const u = localStorage.getItem('u');
if(t && u){
this.token = t;
try{
this.user = JSON.parse(u);
const me = await this.api('/api/auth/me');
if(me.success){
this.user = me.data;
this.isAdmin = this.user.role === 'admin';
this.loggedIn = true;
this.connectWS();
this.loadGroups();
}else{
this.clearAuth();
}
}catch(e){
this.clearAuth();
}
}
},

// API调用
async api(path, options = {}){
const url = location.origin + path;
const headers = {
'Content-Type': 'application/json',
...options.headers
};
if(this.token) headers['Authorization'] = 'Bearer ' + this.token;
try{
const r = await fetch(url, {...options, headers});
return await r.json();
}catch(e){
console.error('API error:', e);
return {error: e.message};
}
},

// 认证
async login(){
if(!this.loginForm.uid || !this.loginForm.pwd){
this.loginError = '请输入UID和密码';
return;
}
this.loginLoading = true;
this.loginError = '';
try{
const r = await fetch(location.origin + '/api/auth/login', {
method: 'POST',
headers: {'Content-Type': 'application/json'},
body: JSON.stringify({uid: this.loginForm.uid.toUpperCase(), password: this.loginForm.pwd})
});
const d = await r.json();
if(d.success){
this.token = d.data.token;
this.user = d.data.user;
this.isAdmin = this.user.role === 'admin';
localStorage.setItem('t', this.token);
localStorage.setItem('u', JSON.stringify(this.user));
this.loggedIn = true;
this.connectWS();
this.loadGroups();
}else{
this.loginError = d.error || '登录失败';
}
}catch(e){
this.loginError = '网络错误';
}
this.loginLoading = false;
},

logout(){
this.api('/api/auth/logout', {method: 'POST'});
this.clearAuth();
},

clearAuth(){
localStorage.clear();
this.token = '';
this.user = null;
this.loggedIn = false;
this.isAdmin = false;
if(this.ws) this.ws.close();
},

// 频道
async loadGroups(){
const d = await this.api('/api/groups');
if(d.success) this.groups = d.data;
},

async enterChannel(){
if(!this.channelInput) return;
const d = await this.api('/api/groups/enter', {
method: 'POST',
body: JSON.stringify({name: this.channelInput})
});
if(d.success){
this.channelInput = '';
this.loadGroups();
}else{
alert(d.error || '进入失败');
}
},

async joinGroup(id){
const d = await this.api('/api/groups/' + id);
if(d.success){
this.currentGroup = d.data;
this.loadMessages();
}
},

leaveGroup(){
this.currentGroup = null;
this.messages = [];
},

// 消息
async loadMessages(){
if(!this.currentGroup) return;
const d = await this.api('/api/messages/group/' + this.currentGroup.id);
if(d.success){
this.messages = d.data;
this.$nextTick(() => this.scrollToBottom());
}
},

async sendMessage(){
if(!this.msgInput.trim() || !this.currentGroup) return;
const d = await this.api('/api/messages', {
method: 'POST',
body: JSON.stringify({
content: this.msgInput,
groupId: this.currentGroup.id
})
});
if(d.success){
this.msgInput = '';
}
},

renderMessage(m){
if(m.msgType === 'image') return '<img src="' + m.content + '" style="max-width:200px;border-radius:8px">';
return this.escapeHtml(m.content);
},

escapeHtml(t){
const d = document.createElement('div');
d.textContent = t;
return d.innerHTML;
},

formatTime(t){
return new Date(t).toLocaleTimeString('zh-CN', {hour: '2-digit', minute: '2-digit'});
},

scrollToBottom(){
const el = document.getElementById('msgs');
if(el) el.scrollTop = el.scrollHeight;
},

// WebSocket
connectWS(){
const proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
this.ws = new WebSocket(proto + '//' + location.host + '/ws?token=' + this.token);
this.ws.onmessage = (e) => {
const m = JSON.parse(e.data);
if(m.event === 'message' && m.data.groupId === this.currentGroup?.id){
this.messages.push(m.data);
this.$nextTick(() => this.scrollToBottom());
}
};
this.ws.onclose = () => setTimeout(() => this.connectWS(), 3000);
},

// 主题
loadTheme(){
this.theme = localStorage.getItem('theme') || 'dark';
if(this.theme === 'light') document.documentElement.setAttribute('data-theme', 'light');
},

toggleTheme(){
this.theme = this.theme === 'dark' ? 'light' : 'dark';
if(this.theme === 'light'){
document.documentElement.setAttribute('data-theme', 'light');
}else{
document.documentElement.removeAttribute('data-theme');
}
localStorage.setItem('theme', this.theme);
},

// 用户菜单
showUserMenu(e, userId, nickname){
e.stopPropagation();
this.userMenu = {
show: true,
x: Math.min(e.clientX, window.innerWidth - 200),
y: Math.min(e.clientY, window.innerHeight - 200),
userId: userId,
nickname: nickname
};
},

// 好友
async addFriend(userId){
const d = await this.api('/api/friends/' + userId, {method: 'POST'});
alert(d.success ? '好友请求已发送' : (d.error || '添加失败'));
this.userMenu.show = false;
},

// 私聊
openDirectChat(userId, nickname){
alert('私聊功能: ' + nickname);
this.userMenu.show = false;
},

// 管理功能
async loadUsers(){
const d = await this.api('/api/admin/users');
if(d.success) this.users = d.data;
},

async createUser(){
if(!this.newUser.nickname || !this.newUser.password){
alert('请填写昵称和密码');
return;
}
const d = await this.api('/api/admin/users', {
method: 'POST',
body: JSON.stringify(this.newUser)
});
if(d.success){
this.newUser = {uid: '', nickname: '', password: ''};
this.loadUsers();
}else{
alert(d.error || '创建失败');
}
},

async banUser(uid){
if(!confirm('确定封禁该用户?')) return;
const d = await this.api('/api/admin/users/' + uid + '/ban', {method: 'PUT'});
alert(d.success ? '已封禁' : (d.error || '操作失败'));
this.loadUsers();
},

async unbanUser(uid){
const d = await this.api('/api/admin/users/' + uid + '/unban', {method: 'PUT'});
alert(d.success ? '已解封' : (d.error || '操作失败'));
this.loadUsers();
},

async muteUser(uid){
const d = await this.api('/api/admin/users/' + uid + '/mute', {
method: 'PUT',
body: JSON.stringify({duration_minutes: 30})
});
alert(d.success ? '已禁言30分钟' : (d.error || '操作失败'));
this.loadUsers();
},

async loadAllGroups(){
const d = await this.api('/api/admin/groups');
if(d.success) this.allGroups = d.data;
},

async deleteGroup(id){
if(!confirm('确定删除该频道?')) return;
const d = await this.api('/api/admin/groups/' + id, {method: 'DELETE'});
if(d.success) this.loadAllGroups();
},

async loadSensitiveWords(){
const d = await this.api('/api/admin/sensitive-words');
if(d.success) this.sensitiveWords = d.data;
},

async addWord(){
if(!this.newWord.word){
alert('请输入敏感词');
return;
}
const d = await this.api('/api/admin/sensitive-words', {
method: 'POST',
body: JSON.stringify(this.newWord)
});
if(d.success){
this.newWord = {word: '', replacement: '***'};
this.loadSensitiveWords();
}else{
alert(d.error || '添加失败');
}
},

async deleteWord(id){
const d = await this.api('/api/admin/sensitive-words/' + id, {method: 'DELETE'});
if(d.success) this.loadSensitiveWords();
},

async loadStats(){
const d = await this.api('/api/admin/statistics');
if(d.success) this.stats = d.data;
}
}
}
</script>
</body>
</html>
"##;

pub const MANIFEST_JSON: &str = r##"{
  "name": "ARCANUM",
  "short_name": "ARCANUM",
  "description": "加密聊天应用",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#000000",
  "theme_color": "#000000",
  "icons": [
    {"src": "/icon-192.png", "sizes": "192x192", "type": "image/png"},
    {"src": "/icon-512.png", "sizes": "512x512", "type": "image/png"}
  ]
}"##;

pub const SERVICE_WORKER_JS: &str = r##"const CACHE_NAME = 'arcanum-v1';
const ASSETS = ['/', '/manifest.json'];
self.addEventListener('install', e => e.waitUntil(caches.open(CACHE_NAME).then(cache => cache.addAll(ASSETS))));
self.addEventListener('activate', e => e.waitUntil(caches.keys().then(keys => Promise.all(keys.filter(k => k !== CACHE_NAME).map(k => caches.delete(k))))));
self.addEventListener('fetch', e => {
  if (e.request.method !== 'GET') return;
  e.respondWith(caches.match(e.request).then(cached => cached || fetch(e.request)));
});
"##;
