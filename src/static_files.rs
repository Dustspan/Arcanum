pub const INDEX_HTML: &str = r##"<!DOCTYPE html>
<html lang="zh">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1.0,maximum-scale=1.0,user-scalable=no">
<meta name="theme-color" content="#000000">
<title>ARCANUM</title>
<link rel="manifest" href="/manifest.json">
<style>
/* 防止模板闪烁 */
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

.input{width:100%;padding:12px;background:var(--bg2);border:1px solid var(--border);color:var(--text);border-radius:4px;font-size:14px;outline:none}
.input:focus{border-color:var(--accent)}

.card{background:var(--card);border:1px solid var(--border);border-radius:8px;padding:16px;margin-bottom:12px}
.err{color:var(--error);font-size:13px;padding:8px;background:rgba(255,51,102,.1);border-radius:4px;margin-top:8px}

.login-logo{font-size:28px;font-weight:700;text-align:center;margin:60px 0 30px;color:var(--accent)}
.login-form{display:flex;flex-direction:column;gap:12px}

.header{display:flex;justify-content:space-between;align-items:center;padding:8px 0;margin-bottom:12px}
.header h1{font-size:16px;color:var(--accent)}
.header-info{font-size:11px;color:var(--muted)}
.header-actions{display:flex;gap:6px}

.channel-input{display:flex;gap:8px;margin-bottom:16px}
.channel-list{display:flex;flex-direction:column;gap:8px}
.channel-card{background:var(--card);border:1px solid var(--border);border-radius:8px;padding:14px;cursor:pointer}
.channel-card:hover{border-color:var(--accent)}
.channel-card h3{font-size:14px;margin-bottom:4px}
.channel-card p{font-size:12px;color:var(--muted)}

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
.msg-img{max-width:180px;border-radius:6px;cursor:pointer}
.msg-file{display:flex;align-items:center;gap:8px;padding:8px;background:var(--bg);border-radius:6px}
.msg-file-icon{font-size:20px}
.msg-file-info{font-size:12px}
.chat-input{display:flex;gap:8px;padding:12px;background:var(--bg);border-top:1px solid var(--border)}
.chat-input textarea{flex:1;padding:8px 12px;background:var(--card);border:1px solid var(--border);color:var(--text);border-radius:16px;font-size:13px;outline:none;resize:none;max-height:60px}
.chat-input input[type="file"]{display:none}
.chat-tools{display:flex;gap:4px}

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

/* 私聊界面 */
.dm-header{display:flex;align-items:center;gap:8px;padding:12px;background:var(--card);border-bottom:1px solid var(--border)}
.dm-back{background:none;border:none;color:var(--accent);font-size:20px;cursor:pointer}
.dm-title{font-size:14px}
.dm-status{font-size:10px;color:var(--success)}
.dm-status.offline{color:var(--muted)}

/* 文件上传按钮 */
.upload-btn{background:var(--bg2);border:1px solid var(--border);border-radius:50%;width:32px;height:32px;display:flex;align-items:center;justify-content:center;cursor:pointer;color:var(--muted);font-size:16px}
.upload-btn:hover{border-color:var(--accent);color:var(--accent)}

/* 回复引用 */
.reply-preview{background:var(--bg);padding:6px 10px;border-radius:4px;margin-bottom:4px;font-size:11px;border-left:2px solid var(--accent)}
.reply-preview .reply-nick{color:var(--accent)}
.reply-preview .reply-content{color:var(--muted);overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
</style>
</head>
<body>
<!-- 加载屏幕 -->
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
<button class="btn full" type="submit" :disabled="loginLoading">
{{ loginLoading ? '登录中...' : '进入' }}
</button>
</form>
</div>
</div>

<!-- 主页 -->
<div v-else>
<!-- 头部 -->
<div class="header">
<div>
<h1>{{ user.nickname }}</h1>
<div class="header-info">{{ user.uid }} <span v-if="isAdmin" class="badge error">管理员</span></div>
</div>
<div class="header-actions">
<button class="btn sm" @click="toggleTheme">{{ theme === 'dark' ? '☀' : '🌙' }}</button>
<button class="btn sm" v-if="canAccessAdmin" @click="openAdmin">⚙</button>
<button class="btn sm danger" @click="doLogout">退出</button>
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
<div class="msg-bubble" :class="m.senderId === user.id ? 'out' : 'in'">
<div class="msg-nick">{{ m.senderNickname }}</div>
<div v-if="m.msgType === 'image'">
<img class="msg-img" :src="m.content" @click="previewImage(m.content)">
</div>
<div v-else>{{ m.content }}</div>
<div class="msg-time">{{ formatTime(m.createdAt) }}</div>
</div>
</div>
</div>
<div class="chat-input">
<textarea v-model="dmInput" placeholder="私聊消息..." @keyup.enter="sendDM" rows="1"></textarea>
<button class="btn" @click="sendDM">→</button>
</div>
</div>

<!-- 频道列表 -->
<div v-else-if="!currentGroup">
<div class="channel-input">
<input class="input" v-model="channelInput" placeholder="输入频道名进入" @keyup.enter="doEnterChannel">
<button class="btn" @click="doEnterChannel" :disabled="channelLoading">{{ channelLoading ? '...' : '进入' }}</button>
</div>
<div class="channel-list">
<div class="channel-card" v-for="g in groups" :key="g.id" @click="doJoinGroup(g.id)">
<h3>{{ g.name }}</h3>
<p>成员: {{ g.memberCount }}</p>
</div>
<div class="card" v-if="groups.length === 0" style="text-align:center;color:var(--muted);font-size:13px">
暂无频道<br>
<small>请联系管理员创建频道</small>
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
<button class="btn sm" @click="doLeaveGroup">← 返回</button>
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
<div v-if="m.replyInfo" class="reply-preview">
<span class="reply-nick">{{ m.replyInfo.senderNickname }}</span>: <span class="reply-content">{{ m.replyInfo.content }}</span>
</div>
<div v-html="renderMsg(m)"></div>
<div class="msg-time">{{ formatTime(m.createdAt) }}</div>
</div>
</div>
</div>
<div class="chat-input">
<div class="chat-tools">
<label class="upload-btn" v-if="hasPerm('file_upload')">
📷
<input type="file" accept="image/*,.txt" @change="uploadFile">
</label>
</div>
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
<div class="perm-list">
<span class="perm-tag" v-for="p in (u.permissions || []).slice(0,5)" :key="p">{{ p }}</span>
</div>
<div style="display:flex;gap:4px;margin-top:6px;flex-wrap:wrap">
<button class="btn sm" v-if="hasPerm('user_ban') && u.status !== 'banned' && u.role !== 'admin'" @click="doBanUser(u.uid)">封禁</button>
<button class="btn sm" v-if="hasPerm('user_ban') && u.status === 'banned'" @click="doUnbanUser(u.uid)">解封</button>
<button class="btn sm" v-if="hasPerm('user_mute')" @click="doMuteUser(u.uid)">禁言</button>
<button class="btn sm" v-if="hasPerm('permission_grant')" @click="openPermModal(u)">权限</button>
</div>
</div>
</div>

<!-- 频道管理 -->
<div class="admin-section" :class="{active: adminTab === 'groups'}">
<div class="card" v-if="hasPerm('group_create')">
<input class="input" v-model="newGroup.name" placeholder="频道名称" style="margin-bottom:8px">
<button class="btn full" @click="doCreateGroup" :disabled="createGroupLoading">
{{ createGroupLoading ? '创建中...' : '创建频道' }}
</button>
</div>
<div class="item-card" v-for="g in allGroups" :key="g.id">
<div class="item-header">
<span class="item-title">{{ g.name }}</span>
<span class="badge success">{{ g.memberCount }}人</span>
</div>
<div class="item-info">{{ g.id }}</div>
<div style="margin-top:6px;display:flex;gap:4px">
<button class="btn sm danger" v-if="hasPerm('group_delete')" @click="doDeleteGroup(g.id)">删除</button>
<button class="btn sm" v-if="hasPerm('message_delete')" @click="doClearGroupMessages(g.id)">清空消息</button>
</div>
</div>
<div class="card" v-if="allGroups.length === 0" style="text-align:center;color:var(--muted);font-size:13px">
暂无频道<br>
<small>在上方创建新频道</small>
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

<!-- 权限管理模态框 -->
<div class="modal-mask" v-if="showPermModal" @click.self="showPermModal = false">
<div class="modal">
<div class="modal-header">
<h3>权限管理 - {{ permTarget?.nickname }}</h3>
<button class="modal-close" @click="showPermModal = false">×</button>
</div>
<div class="modal-body">
<div style="margin-bottom:12px">
<div v-for="p in allPermissions" :key="p.name" style="margin-bottom:6px">
<label style="display:flex;align-items:center;gap:8px;cursor:pointer">
<input type="checkbox" :checked="hasUserPerm(p.name)" @change="togglePerm(p.name)">
<span style="font-size:12px">{{ p.name }}</span>
</label>
</div>
</div>
<button class="btn full" @click="savePerms">保存</button>
</div>
</div>
</div>

<!-- 用户菜单 -->
<div class="user-menu" v-if="userMenu.show" :style="{left: userMenu.x + 'px', top: userMenu.y + 'px'}" @click.stop>
<div class="user-menu-header">
<div class="msg-avatar">{{ userMenu.nickname?.charAt(0) }}</div>
<div>
<div style="font-weight:500">{{ userMenu.nickname }}</div>
<div class="item-info" :class="userMenu.online ? 'badge success' : ''">{{ userMenu.online ? '在线' : '离线' }}</div>
</div>
</div>
<button class="user-menu-item" @click="doDirectChat">私聊</button>
<button class="user-menu-item" @click="doAddFriend">添加好友</button>
<template v-if="canManageUser && userMenu.userId !== user.id">
<button class="user-menu-item" @click="doMuteUser(userMenu.uid)">禁言</button>
<button class="user-menu-item danger" @click="doBanUser(userMenu.uid)">封禁</button>
<button class="user-menu-item danger" @click="doKickUser(userMenu.uid)">踢出</button>
</template>
</div>

</div>
</div>

<!-- Vue 3 -->
<script src="https://unpkg.com/vue@3/dist/vue.global.prod.js"></script>
<script>
const { createApp, ref, reactive, computed, onMounted, nextTick, watch } = Vue;

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
const channelLoading = ref(false);
const isAdmin = ref(false);
const userPerms = ref([]);
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
const createUserLoading = ref(false);
const newGroup = reactive({ name: '' });
const createGroupLoading = ref(false);
const newWord = reactive({ word: '', replacement: '***' });
const userMenu = reactive({ show: false, x: 0, y: 0, uid: '', userId: '', nickname: '', online: false });
const msgsBox = ref(null);
const showPermModal = ref(false);
const permTarget = ref(null);
const permTargetPerms = ref([]);
const allPermissions = ref([]);
// 私聊
const dmTarget = ref(null);
const dmMessages = ref([]);
const dmInput = ref('');
const dmMsgsBox = ref(null);
let ws = null;

// === 计算属性 ===
const canAccessAdmin = computed(() => {
  return isAdmin.value || userPerms.value.length > 0;
});

const canManageUser = computed(() => {
  return hasPerm('user_ban') || hasPerm('user_mute') || isAdmin.value;
});

// === 权限检查 ===
function hasPerm(name) {
  if (isAdmin.value) return true;
  return userPerms.value.includes(name);
}

function hasUserPerm(name) {
  return permTargetPerms.value.includes(name);
}

// === API ===
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
      userPerms.value = d.data.user.permissions || [];
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
  userPerms.value = [];
  if (ws) ws.close();
}

// === 频道 ===
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
  } else {
    alert(d.error || '加入失败');
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
  const content = msgInput.value;
  msgInput.value = '';
  const d = await api('/api/messages', {
    method: 'POST',
    body: JSON.stringify({ content, groupId: currentGroup.value.id })
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
    if (!d.success) {
      alert(d.error || '上传失败');
    }
  } catch (e) {
    alert('上传失败: ' + e.message);
  }
  e.target.value = '';
}

function renderMsg(m) {
  if (m.msgType === 'image') return '<img class="msg-img" src="' + m.content + '" onclick="previewImage(\'' + m.content + '\')">';
  if (m.msgType === 'file') return '<div class="msg-file"><span class="msg-file-icon">📄</span><div class="msg-file-info">' + (m.fileName || '文件') + '</div></div>';
  const el = document.createElement('div');
  el.textContent = m.content;
  return el.innerHTML;
}

function formatTime(t) {
  return new Date(t).toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
}

function scrollToBottom() {
  if (msgsBox.value) msgsBox.value.scrollTop = msgsBox.value.scrollHeight;
  if (dmMsgsBox.value) dmMsgsBox.value.scrollTop = dmMsgsBox.value.scrollHeight;
}

function previewImage(url) {
  window.open(url, '_blank');
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
    if (m.event === 'direct_message') {
      // 如果正在与该用户私聊，添加消息
      if (dmTarget.value && (m.data.senderId === dmTarget.value.id || m.data.receiverId === dmTarget.value.id)) {
        dmMessages.value.push(m.data);
        nextTick(scrollToBottom);
      } else if (m.data.senderId !== user.value.id) {
        // 否则提示
        alert('收到 ' + m.data.senderNickname + ' 的私聊消息');
      }
    }
    if (m.event === 'message_recall' && m.data.groupId === currentGroup.value?.id) {
      const idx = messages.value.findIndex(msg => msg.id === m.data.id);
      if (idx >= 0) messages.value.splice(idx, 1);
    }
    if (m.event === 'friend_request') {
      alert(m.data.from + ' 请求添加你为好友');
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
  // 获取用户在线状态
  api('/api/users/' + userId).then(d => {
    if (d.success) {
      userMenu.online = d.data.online;
    }
  });
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
  dmTarget.value = { id: userMenu.userId, nickname: userMenu.nickname, online: userMenu.online };
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

// === 管理面板 ===
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
  try {
    const d = await api('/api/admin/users');
    if (d.success) users.value = d.data;
  } catch (e) {}
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
  try {
    const d = await api('/api/admin/groups');
    if (d.success) allGroups.value = d.data;
  } catch (e) {}
}

async function doCreateGroup() {
  if (!newGroup.name || !newGroup.name.trim()) {
    alert('请输入频道名称');
    return;
  }
  createGroupLoading.value = true;
  const d = await api('/api/groups', { method: 'POST', body: JSON.stringify({ name: newGroup.name.trim() }) });
  createGroupLoading.value = false;
  if (d.success) {
    alert('频道创建成功: ' + newGroup.name);
    newGroup.name = '';
    loadAllGroups();
    loadGroups();
  } else {
    alert('创建失败: ' + (d.error || '未知错误'));
  }
}

async function doDeleteGroup(id) {
  if (!confirm('确定删除该频道?')) return;
  const d = await api('/api/admin/groups/' + id, { method: 'DELETE' });
  if (d.success) loadAllGroups();
}

async function doClearGroupMessages(id) {
  if (!confirm('确定清空该频道所有消息? 此操作不可恢复!')) return;
  const d = await api('/api/messages/group/' + id, { method: 'DELETE' });
  alert(d.success ? '消息已清空' : (d.error || '失败'));
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

// === 权限管理 ===
function openPermModal(u) {
  permTarget.value = u;
  permTargetPerms.value = [...(u.permissions || [])];
  showPermModal.value = true;
}

function togglePerm(name) {
  const idx = permTargetPerms.value.indexOf(name);
  if (idx >= 0) {
    permTargetPerms.value.splice(idx, 1);
  } else {
    permTargetPerms.value.push(name);
  }
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

// === 初始化 ===
onMounted(() => {
  // 点击其他地方关闭菜单
  document.addEventListener('click', () => {
    userMenu.show = false;
  });
  
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
          userPerms.value = me.data.permissions || [];
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
  
  // 隐藏加载屏幕
  setTimeout(() => {
    document.getElementById('loading').style.display = 'none';
    document.getElementById('app').classList.add('loaded');
  }, 100);
});

return {
  loggedIn, user, token, theme, groups, currentGroup, messages, msgInput, channelInput, channelLoading,
  isAdmin, userPerms, showAdmin, adminTab, users, allGroups, words, stats,
  loginForm, loginError, loginLoading, newUser, createUserLoading, newGroup, createGroupLoading, newWord, userMenu, msgsBox,
  showPermModal, permTarget, permTargetPerms, allPermissions,
  dmTarget, dmMessages, dmInput, dmMsgsBox,
  canAccessAdmin, canManageUser, hasPerm, hasUserPerm,
  doLogin, doLogout, loadGroups, doEnterChannel, doJoinGroup, doLeaveGroup,
  loadMessages, doSendMsg, renderMsg, formatTime, uploadFile, previewImage,
  openUserMenu, doAddFriend, doDirectChat, closeDM, sendDM,
  toggleTheme, openAdmin,
  loadUsers, doCreateUser, doBanUser, doUnbanUser, doMuteUser, doKickUser,
  loadAllGroups, doCreateGroup, doDeleteGroup, doClearGroupMessages, loadWords, doAddWord, doDeleteWord, loadStats,
  openPermModal, togglePerm, savePerms
};
}
}).mount('#app');
</script>
</body>
</html>
"##;

pub const MANIFEST_JSON: &str = r##"{"name":"ARCANUM","short_name":"ARCANUM","start_url":"/","display":"standalone","background_color":"#000000","theme_color":"#000000","icons":[{"src":"/icon-192.png","sizes":"192x192","type":"image/png"}]}"##;

pub const SERVICE_WORKER_JS: &str = r##"const CACHE_NAME='arcanum-v1';self.addEventListener('install',e=>e.waitUntil(caches.open(CACHE_NAME).then(c=>c.addAll(['/', '/manifest.json']))));self.addEventListener('fetch',e=>e.respondWith(caches.match(e.request).then(r=>r||fetch(e.request))));"##;
