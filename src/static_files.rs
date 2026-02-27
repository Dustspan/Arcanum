pub const INDEX_HTML: &str = r##"<!DOCTYPE html>
<html lang="zh">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1.0,maximum-scale=1.0,user-scalable=no">
<meta name="theme-color" content="#000000">
<title>ARCANUM</title>
<link rel="manifest" href="/manifest.json">
<style>
/* === 基础样式 === */
:root{--bg:#0a0a0f;--bg2:#12121a;--card:#16161f;--text:#e0e0e8;--muted:#6a6a7a;--accent:#00f0ff;--accent2:#ff00aa;--border:#2a2a3a;--error:#ff3366;--success:#00ff88;--warn:#ffaa00}
[data-theme="light"]{--bg:#f0f0f5;--bg2:#e8e8f0;--card:#fff;--text:#1a1a2e;--muted:#6a6a7a;--accent:#0088aa;--border:#d0d0da}
*{margin:0;padding:0;box-sizing:border-box}
body{background:var(--bg);color:var(--text);font-family:system-ui,sans-serif;min-height:100vh}
.container{max-width:540px;margin:0 auto;padding:12px;min-height:100vh}
[v-cloak]{display:none}

/* === 组件 === */
.btn{padding:10px 20px;background:transparent;border:1px solid var(--accent);color:var(--accent);border-radius:4px;font-size:14px;cursor:pointer;transition:all .2s}
.btn:hover{background:var(--accent);color:#000}
.btn:disabled{opacity:.5;cursor:not-allowed}
.btn.full{width:100%}
.btn.sm{padding:6px 12px;font-size:12px}
.btn.danger{border-color:var(--error);color:var(--error)}
.btn.danger:hover{background:var(--error);color:#fff}

.input{width:100%;padding:12px;background:var(--bg2);border:1px solid var(--border);color:var(--text);border-radius:4px;font-size:14px;outline:none}
.input:focus{border-color:var(--accent)}

.card{background:var(--card);border:1px solid var(--border);border-radius:8px;padding:16px;margin-bottom:12px}
.err{color:var(--error);font-size:13px;padding:8px;background:rgba(255,51,102,.1);border-radius:4px;margin-top:8px}

/* === 登录 === */
.login-logo{font-size:28px;font-weight:700;text-align:center;margin:60px 0 30px;color:var(--accent)}
.login-form{display:flex;flex-direction:column;gap:12px}

/* === 头部 === */
.header{display:flex;justify-content:space-between;align-items:center;padding:8px 0;margin-bottom:12px}
.header h1{font-size:16px;color:var(--accent)}
.header-info{font-size:11px;color:var(--muted)}
.header-actions{display:flex;gap:6px}

/* === 频道 === */
.channel-input{display:flex;gap:8px;margin-bottom:16px}
.channel-list{display:flex;flex-direction:column;gap:8px}
.channel-card{background:var(--card);border:1px solid var(--border);border-radius:8px;padding:14px;cursor:pointer}
.channel-card:hover{border-color:var(--accent)}
.channel-card h3{font-size:14px;margin-bottom:4px}
.channel-card p{font-size:12px;color:var(--muted)}

/* === 聊天 === */
.chat-wrap{display:flex;flex-direction:column;height:calc(100vh - 100px)}
.chat-header{display:flex;justify-content:space-between;align-items:center;padding:12px;background:var(--card);border-bottom:1px solid var(--border)}
.chat-header h3{font-size:14px}
.chat-msgs{flex:1;overflow-y:auto;padding:12px;display:flex;flex-direction:column;gap:10px;background:var(--bg2)}
.msg-row{display:flex;gap:8px}
.msg-row.me{flex-direction:row-reverse}
.msg-avatar{width:32px;height:32px;border-radius:6px;background:linear-gradient(135deg,var(--accent),var(--accent2));display:flex;align-items:center;justify-content:center;font-size:12px;font-weight:600;color:#000;flex-shrink:0;cursor:pointer;overflow:hidden}
.msg-avatar img{width:100%;height:100%;object-fit:cover}
.msg-bubble{max-width:70%;padding:8px 12px;border-radius:10px;font-size:13px}
.msg-bubble.in{background:var(--card);border:1px solid var(--border)}
.msg-bubble.out{background:var(--accent);color:#000}
.msg-nick{font-size:10px;color:var(--accent);margin-bottom:2px}
.msg-time{font-size:9px;color:var(--muted);margin-top:4px;text-align:right}
.chat-input{display:flex;gap:8px;padding:12px;background:var(--bg);border-top:1px solid var(--border)}
.chat-input textarea{flex:1;padding:8px 12px;background:var(--card);border:1px solid var(--border);color:var(--text);border-radius:16px;font-size:13px;outline:none;resize:none;max-height:60px}

/* === 管理 === */
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

/* === 模态框 === */
.modal-mask{position:fixed;inset:0;background:rgba(0,0,0,.8);display:flex;align-items:center;justify-content:center;z-index:1000;padding:12px}
.modal{background:var(--card);border:1px solid var(--border);border-radius:8px;max-width:400px;width:100%;max-height:90vh;overflow-y:auto}
.modal-header{display:flex;justify-content:space-between;align-items:center;padding:12px;border-bottom:1px solid var(--border)}
.modal-header h3{font-size:14px}
.modal-close{background:none;border:none;color:var(--muted);font-size:20px;cursor:pointer}
.modal-body{padding:12px}

/* === 用户菜单 === */
user-menu{position:fixed;background:var(--card);border:1px solid var(--border);border-radius:6px;padding:6px;z-index:1001;min-width:150px;box-shadow:0 4px 20px rgba(0,0,0,.5)}
.user-menu-header{padding:6px;border-bottom:1px solid var(--border);margin-bottom:6px;display:flex;align-items:center;gap:8px}
.user-menu-item{display:block;width:100%;padding:6px 10px;background:none;border:none;color:var(--text);text-align:left;cursor:pointer;border-radius:4px;font-size:12px}
.user-menu-item:hover{background:var(--bg2)}

.badge{display:inline-block;padding:2px 6px;border-radius:8px;font-size:9px}
.badge.success{background:rgba(0,255,136,.2);color:var(--success)}
.badge.error{background:rgba(255,51,102,.2);color:var(--error)}
</style>
</head>
<body>
<div id="app">
<div class="container">

<!-- 登录页 -->
<div v-if="!loggedIn" v-cloak>
<div class="login-logo">ARCANUM</div>
<div class="card">
<form class="login-form" @submit.prevent="doLogin">
<input class="input" v-model="loginForm.uid" placeholder="UID" autocomplete="username">
<input class="input" v-model="loginForm.pwd" type="password" placeholder="密码" autocomplete="current-password">
<div class="err" v-if="loginError">{{ loginError }}</div>
<button class="btn full" type="submit" :disabled="loginLoading">
{{ loginLoading ? '登录中...' : '进入' }}
</button>
</form>
</div>
</div>

<!-- 主页 -->
<div v-else v-cloak>
<!-- 头部 -->
<div class="header">
<div>
<h1>{{ user.nickname }}</h1>
<div class="header-info">{{ user.uid }}</div>
</div>
<div class="header-actions">
<button class="btn sm" @click="toggleTheme">{{ theme === 'dark' ? '☀' : '🌙' }}</button>
<button class="btn sm" v-if="isAdmin" @click="showAdmin = true">⚙</button>
<button class="btn sm danger" @click="doLogout">退出</button>
</div>
</div>

<!-- 频道列表 -->
<div v-if="!currentGroup">
<div class="channel-input">
<input class="input" v-model="channelInput" placeholder="输入频道名" @keyup.enter="doEnterChannel">
<button class="btn" @click="doEnterChannel">进入</button>
</div>
<div class="channel-list">
<div class="channel-card" v-for="g in groups" :key="g.id" @click="doJoinGroup(g.id)">
<h3>{{ g.name }}</h3>
<p>成员: {{ g.memberCount }}</p>
</div>
<div class="card" v-if="groups.length === 0" style="text-align:center;color:var(--muted)">
暂无频道，输入频道名创建或进入
</div>
</div>
</div>

<!-- 聊天界面 -->
<div v-else class="card chat-wrap">
<div class="chat-header">
<div>
<h3>{{ currentGroup.name }}</h3>
<div class="header-info">成员: {{ currentGroup.memberCount }}</div>
</div>
<div>
<button class="btn sm" @click="showGroupInfo = true">ℹ</button>
<button class="btn sm" @click="doLeaveGroup">←</button>
</div>
</div>
<div class="chat-msgs" ref="msgsBox">
<div class="msg-row" v-for="m in messages" :key="m.id" :class="{me: m.senderId === user.id}">
<div class="msg-avatar" @click="openUserMenu($event, m.senderId, m.senderNickname)">
<img v-if="m.senderAvatar" :src="m.senderAvatar">
<span v-else>{{ m.senderNickname?.charAt(0) }}</span>
</div>
<div class="msg-bubble" :class="m.senderId === user.id ? 'out' : 'in'">
<div class="msg-nick">{{ m.senderNickname }}</div>
<div v-html="renderMsg(m)"></div>
<div class="msg-time">{{ formatTime(m.createdAt) }}</div>
</div>
</div>
</div>
<div class="chat-input">
<textarea v-model="msgInput" placeholder="消息..." @keyup.enter="doSendMsg" rows="1"></textarea>
<button class="btn" @click="doSendMsg">→</button>
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

<!-- 用户 -->
<div class="admin-section" :class="{active: adminTab === 'users'}">
<div class="card">
<input class="input" v-model="newUser.uid" placeholder="UID (留空自动生成)" style="margin-bottom:8px">
<input class="input" v-model="newUser.nickname" placeholder="昵称" style="margin-bottom:8px">
<input class="input" v-model="newUser.password" type="password" placeholder="密码" style="margin-bottom:8px">
<button class="btn full" @click="doCreateUser">创建用户</button>
</div>
<div class="item-card" v-for="u in users" :key="u.id">
<div class="item-header">
<span class="item-title">{{ u.nickname }}</span>
<span class="badge" :class="u.online ? 'success' : ''">{{ u.online ? '在线' : '离线' }}</span>
</div>
<div class="item-info">{{ u.uid }}</div>
<div style="display:flex;gap:4px;margin-top:6px">
<button class="btn sm" v-if="u.status !== 'banned'" @click="doBanUser(u.uid)">封禁</button>
<button class="btn sm" v-else @click="doUnbanUser(u.uid)">解封</button>
<button class="btn sm" @click="doMuteUser(u.uid)">禁言</button>
</div>
</div>
</div>

<!-- 频道 -->
<div class="admin-section" :class="{active: adminTab === 'groups'}">
<div class="item-card" v-for="g in allGroups" :key="g.id">
<div class="item-header">
<span class="item-title">{{ g.name }}</span>
<span class="badge success">{{ g.memberCount }}人</span>
</div>
<div class="item-info">{{ g.id }}</div>
<div style="margin-top:6px">
<button class="btn sm danger" @click="doDeleteGroup(g.id)">删除</button>
</div>
</div>
</div>

<!-- 敏感词 -->
<div class="admin-section" :class="{active: adminTab === 'words'}">
<div class="card">
<input class="input" v-model="newWord.word" placeholder="敏感词" style="margin-bottom:8px">
<input class="input" v-model="newWord.replacement" placeholder="替换为 (默认***)" style="margin-bottom:8px">
<button class="btn full" @click="doAddWord">添加</button>
</div>
<div class="item-card" v-for="w in words" :key="w.id">
<div class="item-header">
<span class="item-title">{{ w.word }}</span>
<button class="btn sm danger" @click="doDeleteWord(w.id)">删除</button>
</div>
<div class="item-info">替换为: {{ w.replacement }}</div>
</div>
</div>

<!-- 统计 -->
<div class="admin-section" :class="{active: adminTab === 'stats'}">
<div class="stats-grid">
<div class="stat-card">
<div class="stat-value">{{ stats.users?.total || 0 }}</div>
<div class="stat-label">用户总数</div>
</div>
<div class="stat-card">
<div class="stat-value">{{ stats.users?.online || 0 }}</div>
<div class="stat-label">在线用户</div>
</div>
<div class="stat-card">
<div class="stat-value">{{ stats.groups?.total || 0 }}</div>
<div class="stat-label">频道总数</div>
</div>
<div class="stat-card">
<div class="stat-value">{{ stats.messages?.total || 0 }}</div>
<div class="stat-label">消息总数</div>
</div>
</div>
</div>
</div>
</div>
</div>

<!-- 用户菜单 -->
<user-menu v-if="userMenu.show" :style="{left: userMenu.x + 'px', top: userMenu.y + 'px'}" @click.away="userMenu.show = false">
<div class="user-menu-header">
<div class="msg-avatar">{{ userMenu.nickname?.charAt(0) }}</div>
<div>
<div style="font-weight:500">{{ userMenu.nickname }}</div>
<div class="item-info">{{ userMenu.uid }}</div>
</div>
</div>
<button class="user-menu-item" @click="doAddFriend">添加好友</button>
<button class="user-menu-item" @click="doDirectChat">私聊</button>
<template v-if="isAdmin">
<button class="user-menu-item" @click="doMuteUser(userMenu.uid)">禁言</button>
<button class="user-menu-item" style="color:var(--error)" @click="doBanUser(userMenu.uid)">封禁</button>
</template>
</user-menu>

</div>
</div>

<!-- Vue 3 -->
<script src="https://unpkg.com/vue@3/dist/vue.global.prod.js"></script>
<script>
const { createApp, ref, reactive, computed, onMounted, nextTick } = Vue;

createApp({
setup() {
// === 状态 ===
const loggedIn = ref(false);
const user = ref({});
const token = ref('');
const theme = ref('dark');
const groups = ref([]);
const currentGroup = ref(null);
const messages = ref([]);
const msgInput = ref('');
const channelInput = ref('');
const isAdmin = ref(false);
const showAdmin = ref(false);
const adminTab = ref('users');
const users = ref([]);
const allGroups = ref([]);
const words = ref([]);
const stats = ref({});
const loginForm = reactive({ uid: '', pwd: '' });
const loginError = ref('');
const loginLoading = ref(false);
const newUser = reactive({ uid: '', nickname: '', password: '' });
const newWord = reactive({ word: '', replacement: '***' });
const userMenu = reactive({ show: false, x: 0, y: 0, uid: '', userId: '', nickname: '' });
const msgsBox = ref(null);
let ws = null;

// === API ===
const API = location.origin;

async function api(path, options = {}) {
  const headers = { 'Content-Type': 'application/json', ...options.headers };
  if (token.value) headers['Authorization'] = 'Bearer ' + token.value;
  try {
    const r = await fetch(API + path, { ...options, headers });
    return await r.json();
  } catch (e) {
    console.error('API Error:', e);
    return { error: e.message };
  }
}

// === 登录 ===
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
      localStorage.setItem('t', token.value);
      localStorage.setItem('u', JSON.stringify(user.value));
      loggedIn.value = true;
      connectWS();
      loadGroups();
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
  if (ws) ws.close();
}

// === 频道 ===
async function loadGroups() {
  const d = await api('/api/groups');
  if (d.success) groups.value = d.data;
}

async function doEnterChannel() {
  if (!channelInput.value) return;
  const d = await api('/api/groups/enter', {
    method: 'POST',
    body: JSON.stringify({ name: channelInput.value })
  });
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
  }
}

function doLeaveGroup() {
  currentGroup.value = null;
  messages.value = [];
}

// === 消息 ===
async function loadMessages() {
  if (!currentGroup.value) return;
  const d = await api('/api/messages/group/' + currentGroup.value.id);
  if (d.success) {
    messages.value = d.data;
    nextTick(scrollToBottom);
  }
}

async function doSendMsg() {
  if (!msgInput.value.trim() || !currentGroup.value) return;
  const d = await api('/api/messages', {
    method: 'POST',
    body: JSON.stringify({ content: msgInput.value, groupId: currentGroup.value.id })
  });
  if (d.success) msgInput.value = '';
}

function renderMsg(m) {
  if (m.msgType === 'image') return '<img src="' + m.content + '" style="max-width:180px;border-radius:6px">';
  const el = document.createElement('div');
  el.textContent = m.content;
  return el.innerHTML;
}

function formatTime(t) {
  return new Date(t).toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
}

function scrollToBottom() {
  if (msgsBox.value) msgsBox.value.scrollTop = msgsBox.value.scrollHeight;
}

// === WebSocket ===
function connectWS() {
  const proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
  ws = new WebSocket(proto + '//' + location.host + '/ws?token=' + token.value);
  
  ws.onmessage = (e) => {
    const m = JSON.parse(e.data);
    if (m.event === 'message' && m.data.groupId === currentGroup.value?.id) {
      messages.value.push(m.data);
      nextTick(scrollToBottom);
    }
  };
  
  ws.onclose = () => setTimeout(connectWS, 3000);
}

// === 主题 ===
function toggleTheme() {
  theme.value = theme.value === 'dark' ? 'light' : 'dark';
  if (theme.value === 'light') {
    document.documentElement.setAttribute('data-theme', 'light');
  } else {
    document.documentElement.removeAttribute('data-theme');
  }
  localStorage.setItem('theme', theme.value);
}

// === 用户菜单 ===
function openUserMenu(e, userId, nickname) {
  e.stopPropagation();
  userMenu.show = true;
  userMenu.x = Math.min(e.clientX, window.innerWidth - 160);
  userMenu.y = Math.min(e.clientY, window.innerHeight - 150);
  userMenu.userId = userId;
  userMenu.nickname = nickname;
  userMenu.uid = userId;
}

function doAddFriend() {
  api('/api/friends/' + userMenu.userId, { method: 'POST' }).then(d => {
    alert(d.success ? '好友请求已发送' : (d.error || '失败'));
  });
  userMenu.show = false;
}

function doDirectChat() {
  alert('私聊: ' + userMenu.nickname);
  userMenu.show = false;
}

// === 管理 ===
async function loadUsers() {
  const d = await api('/api/admin/users');
  if (d.success) users.value = d.data;
}

async function doCreateUser() {
  if (!newUser.nickname || !newUser.password) {
    alert('请填写昵称和密码');
    return;
  }
  const d = await api('/api/admin/users', { method: 'POST', body: JSON.stringify(newUser) });
  if (d.success) {
    newUser.uid = '';
    newUser.nickname = '';
    newUser.password = '';
    loadUsers();
  } else {
    alert(d.error || '创建失败');
  }
}

async function doBanUser(uid) {
  if (!confirm('确定封禁?')) return;
  const d = await api('/api/admin/users/' + uid + '/ban', { method: 'PUT' });
  alert(d.success ? '已封禁' : (d.error || '失败'));
  loadUsers();
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
}

async function loadAllGroups() {
  const d = await api('/api/admin/groups');
  if (d.success) allGroups.value = d.data;
}

async function doDeleteGroup(id) {
  if (!confirm('确定删除?')) return;
  const d = await api('/api/admin/groups/' + id, { method: 'DELETE' });
  if (d.success) loadAllGroups();
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

// === 初始化 ===
onMounted(() => {
  console.log('ARCANUM mounted');
  
  // 加载主题
  const savedTheme = localStorage.getItem('theme');
  if (savedTheme === 'light') {
    theme.value = 'light';
    document.documentElement.setAttribute('data-theme', 'light');
  }
  
  // 检查登录状态
  const t = localStorage.getItem('t');
  const u = localStorage.getItem('u');
  if (t && u) {
    token.value = t;
    try {
      user.value = JSON.parse(u);
      api('/api/auth/me').then(me => {
        if (me.success) {
          user.value = me.data;
          isAdmin.value = me.data.role === 'admin';
          loggedIn.value = true;
          connectWS();
          loadGroups();
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
});

return {
  loggedIn, user, token, theme, groups, currentGroup, messages, msgInput, channelInput,
  isAdmin, showAdmin, adminTab, users, allGroups, words, stats,
  loginForm, loginError, loginLoading, newUser, newWord, userMenu, msgsBox,
  doLogin, doLogout, loadGroups, doEnterChannel, doJoinGroup, doLeaveGroup,
  loadMessages, doSendMsg, renderMsg, formatTime, toggleTheme,
  openUserMenu, doAddFriend, doDirectChat,
  loadUsers, doCreateUser, doBanUser, doUnbanUser, doMuteUser,
  loadAllGroups, doDeleteGroup, loadWords, doAddWord, doDeleteWord, loadStats
};
}
}).mount('#app');
</script>
</body>
</html>
"##;

pub const MANIFEST_JSON: &str = r##"{
  "name": "ARCANUM",
  "short_name": "ARCANUM",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#000000",
  "theme_color": "#000000",
  "icons": [
    {"src": "/icon-192.png", "sizes": "192x192", "type": "image/png"}
  ]
}"##;

pub const SERVICE_WORKER_JS: &str = r##"const CACHE_NAME = 'arcanum-v1';
self.addEventListener('install', e => e.waitUntil(caches.open(CACHE_NAME).then(c => c.addAll(['/', '/manifest.json']))));
self.addEventListener('fetch', e => e.respondWith(caches.match(e.request).then(r => r || fetch(e.request))));
"##;
