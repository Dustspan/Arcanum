pub const INDEX_HTML: &str = r##"<!DOCTYPE html>
<html lang="zh">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1.0,maximum-scale=1.0,user-scalable=no">
<title>ARCANUM</title>
<style>
*{margin:0;padding:0;box-sizing:border-box}
:root{--bg:#000;--card:#0d0d0d;--text:#f0f0f0;--muted:#666;--accent:#0ff;--accent2:#f0f;--border:#1a1a1a;--error:#ff4466;--success:#00ff88;--warn:#ffaa00}
[data-theme="light"]{--bg:#f5f5f5;--card:#fff;--text:#1a1a1a;--muted:#888;--accent:#088;--accent2:#a0a;--border:#ddd;--error:#c44;--success:#0a0;--warn:#a80}
body{background:var(--bg);color:var(--text);font-family:-apple-system,sans-serif;min-height:100vh;min-height:100dvh;line-height:1.4;transition:background .3s,color .3s}
@keyframes glitch{0%,100%{text-shadow:-2px 0 var(--accent2),2px 0 var(--accent)}25%{text-shadow:2px 0 var(--accent2),-2px 0 var(--accent)}50%{text-shadow:-1px 0 var(--accent2),1px 0 var(--accent)}75%{text-shadow:1px 0 var(--accent2),-1px 0 var(--accent)}}
.glitch{animation:glitch .3s infinite}
.scanlines::before{content:"";position:fixed;inset:0;background:repeating-linear-gradient(0deg,rgba(0,0,0,.06),rgba(0,0,0,.06) 1px,transparent 1px,transparent 2px);pointer-events:none;z-index:9999}
.container{width:100%;max-width:540px;margin:0 auto;padding:12px;min-height:100vh;min-height:100dvh}
.hidden{display:none!important}
.logo{font-size:clamp(18px,5vw,24px);font-weight:300;letter-spacing:clamp(4px,2vw,8px);text-align:center;padding:clamp(24px,8vw,40px) 0 clamp(16px,4vw,24px);color:var(--accent)}
.card{background:var(--card);border:1px solid var(--border);border-radius:12px;padding:clamp(12px,3vw,16px);margin:8px 0}
.input{width:100%;padding:10px 12px;background:transparent;border:1px solid var(--border);color:var(--text);border-radius:8px;font-size:14px;outline:none}
.input:focus{border-color:var(--accent)}
.btn{padding:8px 16px;background:transparent;border:1px solid var(--accent);color:var(--accent);border-radius:8px;font-size:13px;cursor:pointer;transition:all .2s}
.btn:hover{background:var(--accent);color:#000}
.btn:disabled{opacity:.5;cursor:not-allowed}
.btn.full{width:100%}
.btn.sm{padding:5px 10px;font-size:11px}
.btn.danger{border-color:var(--error);color:var(--error)}
.btn.danger:hover{background:var(--error);color:#fff}
.btn.warn{border-color:var(--warn);color:var(--warn)}
.btn.warn:hover{background:var(--warn);color:#000}
.btn.success{border-color:var(--success);color:var(--success)}
.btn.success:hover{background:var(--success);color:#000}
.err{color:var(--error);font-size:12px;margin:8px 0;text-align:center}
.success{color:var(--success);font-size:12px;margin:8px 0}
.status{position:fixed;top:8px;right:8px;padding:4px 10px;font-size:10px;border:1px solid var(--border);border-radius:12px;z-index:100}
.status.on{border-color:var(--accent);color:var(--accent)}
.status.reconnecting{border-color:var(--warn);color:var(--warn)}
.channel-card{background:linear-gradient(135deg,rgba(0,255,255,.03),rgba(255,0,255,.03));border:1px solid var(--border);border-radius:12px;padding:14px;margin:8px 0;cursor:pointer;transition:all .2s}
.channel-card:active{border-color:var(--accent);transform:scale(.99)}
.channel-card h3{font-size:15px;margin-bottom:4px}
.channel-card p{color:var(--muted);font-size:12px}
.chat-wrap{display:flex;flex-direction:column;height:calc(100vh - 24px);height:calc(100dvh - 24px)}
.chat-header{display:flex;align-items:center;padding:12px;border-bottom:1px solid var(--border);flex-shrink:0}
.chat-header h2{flex:1;text-align:center;font-size:15px;font-weight:500}
.chat-header-actions{display:flex;gap:4px}
.chat-back{background:none;border:none;color:var(--accent);font-size:14px;cursor:pointer;padding:4px 8px}
.chat-msgs{flex:1;overflow-y:auto;padding:12px;display:flex;flex-direction:column;gap:12px;background:#050505}
.msg-row{display:flex;align-items:flex-start;gap:8px}
.msg-row.me{flex-direction:row-reverse}
.msg-avatar{width:36px;height:36px;border-radius:8px;background:linear-gradient(135deg,var(--accent),var(--accent2));display:flex;align-items:center;justify-content:center;font-size:14px;font-weight:600;color:#000;flex-shrink:0;cursor:pointer;overflow:hidden;object-fit:cover;position:relative}
.msg-avatar img{width:100%;height:100%;object-fit:cover}
.msg-avatar .online-dot{position:absolute;bottom:0;right:0;width:10px;height:10px;border-radius:50%;border:2px solid var(--bg)}
.msg-avatar .online-dot.on{background:var(--success)}
.msg-avatar .online-dot.off{background:var(--muted)}
.msg-bubble{max-width:70%;padding:10px 12px;border-radius:12px;font-size:14px;line-height:1.5;word-break:break-word}
.msg-bubble.in{background:var(--card);border:1px solid var(--border);border-top-left-radius:4px}
.msg-bubble.out{background:var(--accent);color:#000;border-top-right-radius:4px}
.msg-nick{font-size:11px;color:var(--accent);margin-bottom:2px}
.msg-bubble.out .msg-nick{color:rgba(0,0,0,.5)}
.msg-time{font-size:10px;color:var(--muted);margin-top:4px;text-align:right}
.msg-bubble.out .msg-time{color:rgba(0,0,0,.4)}
.msg-read{margin-left:8px;color:var(--accent)}
.msg-reply{padding:4px 8px;margin-bottom:4px;background:rgba(0,0,0,.1);border-left:2px solid var(--accent);border-radius:4px;font-size:11px;cursor:pointer}
.msg-reply:hover{background:rgba(0,0,0,.2)}
.msg-reply-nick{color:var(--accent);font-weight:500;margin-right:8px}
.msg-reply-content{color:var(--muted)}
.msg-highlight{animation:highlight 2s}
@keyframes highlight{0%,100%{background:transparent}50%{background:rgba(0,255,255,.2)}}
.reply-btn{background:none;border:none;color:var(--muted);cursor:pointer;padding:2px 4px;font-size:10px;opacity:.5}
.reply-btn:hover{opacity:1;color:var(--accent)}
.reply-preview{padding:8px 12px;background:var(--card);border-bottom:1px solid var(--border);font-size:12px;display:flex;justify-content:space-between;align-items:center}
.reply-preview b{color:var(--accent)}
.member-item{display:flex;align-items:center;gap:12px;padding:8px;border-bottom:1px solid var(--border)}
.member-item:last-child{border-bottom:none}
.member-avatar{width:40px;height:40px;border-radius:50%;background:var(--accent);display:flex;align-items:center;justify-content:center;font-weight:500;color:#000;overflow:hidden}
.member-avatar img{width:100%;height:100%;object-fit:cover}
.member-info{flex:1}
.member-nick{font-size:14px;font-weight:500}
.member-badge{font-size:10px;padding:2px 6px;border-radius:4px;margin-left:8px}
.member-badge.admin{background:var(--accent);color:#000}
.member-status{font-size:11px;margin-top:2px}
.member-status.online{color:var(--success)}
.member-status.offline{color:var(--muted)}
.typing-indicator{padding:4px 12px;font-size:11px;color:var(--muted);font-style:italic}
.group-announcement{padding:8px 12px;background:rgba(0,255,255,.1);border-bottom:1px solid var(--border);font-size:12px;color:var(--accent);cursor:pointer}
.group-announcement:hover{background:rgba(0,255,255,.15)}
.search-result-item{padding:8px;border-bottom:1px solid var(--border);cursor:pointer}
.search-result-item:hover{background:rgba(255,255,255,.05)}
.search-result-nick{font-size:12px;color:var(--accent);margin-bottom:4px}
.search-result-content{font-size:13px;color:var(--text);word-break:break-all}
.msg-image{max-width:100%;max-height:300px;border-radius:8px;cursor:pointer;display:block}
.msg-file{display:flex;align-items:center;gap:8px;padding:8px;background:rgba(0,0,0,.2);border-radius:8px;margin-top:4px}
.msg-file-icon{width:32px;height:32px;background:var(--accent);border-radius:6px;display:flex;align-items:center;justify-content:center}
.msg-file-info{flex:1}
.msg-file-name{font-size:12px;font-weight:500}
.msg-file-size{font-size:10px;color:var(--muted)}
.chat-input{display:flex;gap:8px;padding:12px;border-top:1px solid var(--border);background:var(--bg);flex-shrink:0;align-items:flex-end}
.chat-input textarea{flex:1;padding:10px 12px;background:var(--card);border:1px solid var(--border);color:var(--text);border-radius:20px;font-size:14px;outline:none;resize:none;max-height:80px;font-family:inherit;line-height:1.4}
.chat-input textarea:focus{border-color:var(--accent)}
.chat-actions{display:flex;gap:4px}
.chat-action-btn{width:36px;height:36px;background:var(--card);border:1px solid var(--border);border-radius:50%;cursor:pointer;display:flex;align-items:center;justify-content:center;color:var(--muted);transition:all .2s}
.chat-action-btn:hover{border-color:var(--accent);color:var(--accent)}
.send-btn{width:40px;height:40px;background:var(--accent);border:none;border-radius:50%;cursor:pointer;display:flex;align-items:center;justify-content:center;flex-shrink:0}
.send-btn svg{width:18px;height:18px;fill:#000}
.admin-tabs{display:flex;gap:4px;margin-bottom:12px;flex-wrap:wrap}
.admin-tab{flex:1;min-width:60px;padding:10px;background:transparent;border:1px solid var(--border);color:var(--muted);border-radius:8px;font-size:12px;cursor:pointer;text-align:center}
.admin-tab.on{border-color:var(--accent);color:var(--accent)}
.admin-section{display:none}
.admin-section.on{display:block}
.admin-form{display:flex;flex-direction:column;gap:8px;margin-bottom:12px}
.admin-form input{margin:0}
.item-card{background:var(--card);border:1px solid var(--border);border-radius:10px;padding:12px;margin:8px 0}
.item-card .item-header{display:flex;justify-content:space-between;align-items:center;margin-bottom:6px}
.item-card .item-title{font-size:14px;font-weight:500}
.item-card .item-badge{font-size:10px;padding:2px 8px;border-radius:10px;background:var(--border);color:var(--muted)}
.item-card .item-badge.admin{background:rgba(255,170,0,.2);color:var(--warn)}
.item-card .item-badge.banned{background:rgba(255,68,102,.2);color:var(--error)}
.item-card .item-badge.active{background:rgba(0,255,136,.2);color:var(--success)}
.item-card .item-badge.online{background:rgba(0,255,255,.2);color:var(--accent)}
.item-card .item-badge.muted{background:rgba(255,170,0,.2);color:var(--warn)}
.item-card .item-info{font-size:11px;color:var(--muted);margin-bottom:8px}
.item-card .item-actions{display:flex;gap:4px;flex-wrap:wrap}
.item-card .item-actions button{flex:1;min-width:50px}
.empty{text-align:center;color:var(--muted);font-size:13px;padding:24px}
.user-menu{position:fixed;background:var(--card);border:1px solid var(--border);border-radius:12px;padding:8px;z-index:1000;min-width:180px;box-shadow:0 4px 20px rgba(0,0,0,.5)}
.user-menu-header{padding:8px;border-bottom:1px solid var(--border);margin-bottom:8px;display:flex;align-items:center;gap:10px}
.user-menu-avatar{width:40px;height:40px;border-radius:8px;background:linear-gradient(135deg,var(--accent),var(--accent2));display:flex;align-items:center;justify-content:center;font-size:16px;font-weight:600;color:#000;overflow:hidden;flex-shrink:0}
.user-menu-avatar img{width:100%;height:100%;object-fit:cover}
.user-menu-info h4{font-size:14px;font-weight:500}
.user-menu-info p{font-size:11px;color:var(--muted)}
.user-menu-status{font-size:11px;margin-top:2px}
.user-menu-status.online{color:var(--success)}
.user-menu-status.offline{color:var(--muted)}
.user-menu-status.muted{color:var(--warn)}
.user-menu-item{display:block;width:100%;padding:8px 12px;background:transparent;border:none;color:var(--text);font-size:13px;text-align:left;cursor:pointer;border-radius:6px}
.user-menu-item:hover{background:var(--border)}
.user-menu-item.danger{color:var(--error)}
.user-menu-item.warn{color:var(--warn)}
.permission-list{display:flex;flex-wrap:wrap;gap:4px;margin-top:8px}
.permission-tag{font-size:10px;padding:2px 6px;border-radius:4px;background:rgba(0,255,255,.1);color:var(--accent);border:1px solid rgba(0,255,255,.2)}
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,.8);display:flex;align-items:center;justify-content:center;z-index:2000}
.modal{background:var(--card);border:1px solid var(--border);border-radius:12px;padding:16px;max-width:90%;max-height:90%;overflow:auto}
.modal-header{display:flex;justify-content:space-between;align-items:center;margin-bottom:12px}
.modal-header h3{font-size:16px;font-weight:500}
.modal-close{background:none;border:none;color:var(--muted);font-size:20px;cursor:pointer}
.permission-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(140px,1fr));gap:8px;max-height:300px;overflow-y:auto}
.permission-item{padding:8px;background:var(--bg);border:1px solid var(--border);border-radius:8px}
.permission-item label{display:flex;align-items:center;gap:8px;font-size:12px;cursor:pointer}
.permission-item input{accent-color:var(--accent)}
.mute-options{display:flex;gap:8px;flex-wrap:wrap;margin-top:8px}
.mute-option{padding:6px 12px;background:var(--bg);border:1px solid var(--border);border-radius:6px;font-size:12px;cursor:pointer}
.mute-option:hover{border-color:var(--accent)}
.mute-option.on{border-color:var(--accent);color:var(--accent)}
.upload-progress{position:fixed;bottom:80px;left:50%;transform:translateX(-50%);background:var(--card);border:1px solid var(--accent);border-radius:8px;padding:8px 16px;font-size:12px;color:var(--accent);z-index:1000}
.loading-spinner{display:inline-block;width:14px;height:14px;border:2px solid var(--accent);border-radius:50%;border-top-color:transparent;animation:spin 1s linear infinite;margin-right:8px}
.theme-toggle{position:absolute;top:12px;right:12px;background:var(--card);border:1px solid var(--border);border-radius:50%;width:36px;height:36px;font-size:16px;cursor:pointer;transition:all .2s}
.theme-toggle:hover{border-color:var(--accent)}
@keyframes spin{to{transform:rotate(360deg)}}
@media(min-width:540px){.container{padding:16px}}
</style>
</head>
<body class="scanlines">
<div class="status" id="status">离线</div>

<div id="loginPage" class="container">
<div class="logo glitch"><span id="logoText"></span></div>
<button class="theme-toggle" id="themeToggle" title="切换主题">🌙</button>
<div class="card">
<input class="input" id="loginUid" placeholder="UID" autocapitalize="characters" style="margin-bottom:8px">
<input class="input" type="password" id="loginPwd" placeholder="密码" style="margin-bottom:8px">
<button class="btn full" id="loginBtn">进入</button>
<div class="err" id="loginErr"></div>
</div>
</div>

<div id="mainPage" class="container hidden">
<div class="logo glitch"><span id="logoText2"></span></div>

<div id="channelView">
<div class="card">
<input class="input" id="cipherInput" placeholder="输入频道名进入..." autocapitalize="off">
<button class="btn full" id="enterChannelBtn" style="margin-top:8px">进入频道</button>
<div class="err" id="cipherErr"></div>
</div>
<div id="myChannels"></div>
<div class="card hidden" id="adminEntry"><button class="btn full" id="showAdminBtn">管理面板</button></div>
</div>

<div id="chatView" class="hidden">
<div class="chat-wrap">
<div class="chat-header">
<button class="chat-back" id="leaveChatBtn">←</button>
<h2 id="chatTitle">聊天</h2>
<div class="chat-header-actions">
<button class="chat-action-btn" id="membersBtn" title="成员">👥</button>
<button class="chat-action-btn" id="searchBtn" title="搜索">🔍</button>
<button class="chat-action-btn" id="groupInfoBtn" title="频道信息">ℹ</button>
</div>
</div>
<div class="group-announcement hidden" id="groupAnnouncement"></div>
<div class="chat-msgs" id="msgs"></div>
<div class="typing-indicator hidden" id="typingIndicator"></div>
<div class="reply-preview hidden" id="replyPreview"><span></span><button onclick="cancelReply()">✕</button></div>
<div class="chat-input">
<textarea id="msgInput" rows="1" placeholder="消息..."></textarea>
<div class="chat-actions">
<label class="chat-action-btn" title="上传图片">
<input type="file" accept="image/*" id="imageInput" style="display:none">
<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2"/><circle cx="8.5" cy="8.5" r="1.5"/><path d="M21 15l-5-5L5 21"/></svg>
</label>
<label class="chat-action-btn" title="上传文件">
<input type="file" accept=".txt" id="fileInput" style="display:none">
<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><path d="M14 2v6h6"/><path d="M12 18v-6"/><path d="M9 15l3-3 3 3"/></svg>
</label>
</div>
<button class="send-btn" id="sendBtn"><svg viewBox="0 0 24 24"><path d="M2 21l21-9L2 3v7l15 2-15 2v7z"/></svg></button>
</div>
</div>
</div>

<div id="adminView" class="hidden">
<div class="admin-tabs">
<div class="admin-tab on" data-tab="users">用户</div>
<div class="admin-tab" data-tab="groups">频道</div>
<div class="admin-tab" data-tab="ips">IP</div>
</div>

<div id="usersSection" class="admin-section on">
<div class="card" id="createUserCard">
<h3 style="font-size:13px;color:var(--accent);margin-bottom:10px">创建用户</h3>
<div class="admin-form">
<input class="input" id="newUid" placeholder="UID (留空自动生成)" autocapitalize="characters">
<input class="input" id="newNick" placeholder="昵称">
<input class="input" id="newPwd" placeholder="密码 (6位+)">
<button class="btn full" id="createUserBtn">创建</button>
</div>
<div id="userRes"></div>
</div>
<div id="userList"></div>
</div>

<div id="groupsSection" class="admin-section">
<div class="card" id="createGroupCard">
<h3 style="font-size:13px;color:var(--accent);margin-bottom:10px">创建频道</h3>
<div class="admin-form">
<input class="input" id="newChan" placeholder="频道名 (即暗号)">
<button class="btn full" id="createChannelBtn">创建</button>
</div>
<div id="chanRes"></div>
</div>
<div id="groupList"></div>
</div>

<div id="ipsSection" class="admin-section">
<div class="card">
<h3 style="font-size:13px;color:var(--warn);margin-bottom:10px">IP封禁列表</h3>
<div id="ipList"></div>
</div>
</div>

<button class="btn full" id="showChannelBtn" style="margin-top:12px">返回频道</button>
</div>
</div>

<div class="user-menu hidden" id="userMenu">
<div class="user-menu-header">
<div class="user-menu-avatar" id="menuAvatar">U</div>
<div class="user-menu-info">
<h4 id="menuUserName">用户名</h4>
<p id="menuUserInfo">UID: xxx</p>
<p class="user-menu-status" id="menuUserStatus">在线</p>
</div>
</div>
<div id="menuActions"></div>
<button class="user-menu-item" id="closeUserMenuBtn">关闭</button>
</div>

<div class="modal-overlay hidden" id="permModal">
<div class="modal">
<div class="modal-header">
<h3>管理用户权限</h3>
<button class="modal-close" id="closePermModalBtn">×</button>
</div>
<p style="font-size:12px;color:var(--muted);margin-bottom:8px">用户: <span id="permUserName" style="color:var(--accent)"></span></p>
<p style="font-size:11px;color:var(--muted);margin-bottom:12px">提示: 勾选权限后点击保存即可授权，取消勾选可撤销权限</p>
<div class="permission-grid" id="permGrid"></div>
<button class="btn full success" style="margin-top:12px" id="savePermsBtn">保存权限</button>
</div>
</div>

<div class="modal-overlay hidden" id="muteModal">
<div class="modal">
<div class="modal-header">
<h3>禁言用户</h3>
<button class="modal-close" id="closeMuteModalBtn">×</button>
</div>
<p style="font-size:12px;color:var(--muted);margin-bottom:12px">用户: <span id="muteUserName" style="color:var(--accent)"></span></p>
<div class="mute-options" id="muteOptions"></div>
<button class="btn full warn" style="margin-top:12px" id="confirmMuteBtn">确认禁言</button>
</div>
</div>

<div class="modal-overlay hidden" id="groupInfoModal">
<div class="modal">
<div class="modal-header">
<h3>频道信息</h3>
<button class="modal-close" id="closeGroupInfoModalBtn">×</button>
</div>
<div id="groupInfoContent"></div>
</div>
</div>

<div class="modal-overlay hidden" id="searchModal">
<div class="modal">
<div class="modal-header">
<h3>搜索消息</h3>
<button class="modal-close" id="closeSearchModalBtn">×</button>
</div>
<div style="padding:8px">
<input class="input" id="searchInput" placeholder="输入关键词搜索...">
<div id="searchResults" style="margin-top:12px;max-height:300px;overflow-y:auto"></div>
</div>
</div>
</div>

<div class="modal-overlay hidden" id="membersModal">
<div class="modal">
<div class="modal-header">
<h3>频道成员</h3>
<button class="modal-close" id="closeMembersModalBtn">×</button>
</div>
<div id="membersList" style="padding:8px;max-height:400px;overflow-y:auto"></div>
</div>
</div>

<div class="upload-progress hidden" id="uploadProgress"><span class="loading-spinner"></span>上传中...</div>

<script>
(function(){
let token="",user=null,ws=null,groupId=null,lastSend=0;
let menuTargetUser=null;
let selectedMuteDuration=30;
let allPermissions=[];
let userPermissions={};
let onlineUsers=new Set();
let displayedMsgIds=new Set();
const API=location.origin;

let wsReconnectAttempts=0;
let wsMaxReconnectAttempts=10;
let wsReconnectDelay=1000;
let wsHeartbeatInterval=null;
let wsLastPong=0;

// 主题切换
function initTheme(){
const saved=localStorage.getItem("theme");
if(saved==="light"){
document.documentElement.setAttribute("data-theme","light");
$("themeToggle").textContent="☀";
}
}

function toggleTheme(){
const current=document.documentElement.getAttribute("data-theme");
if(current==="light"){
document.documentElement.removeAttribute("data-theme");
localStorage.setItem("theme","dark");
$("themeToggle").textContent="🌙";
}else{
document.documentElement.setAttribute("data-theme","light");
localStorage.setItem("theme","light");
$("themeToggle").textContent="☀";
}
}

function $(id){return document.getElementById(id)}

function esc(t){const d=document.createElement("div");d.textContent=t;return d.innerHTML}
function formatTime(t){return new Date(t).toLocaleTimeString("zh-CN",{hour:"2-digit",minute:"2-digit"})}
function formatFileSize(b){if(b<1024)return b+"B";if(b<1024*1024)return(b/1024).toFixed(1)+"KB";return(b/1024/1024).toFixed(1)+"MB"}

function typeWriter(el,text,i){if(i<text.length){el.textContent=text.substring(0,i+1);setTimeout(()=>typeWriter(el,text,i+1),100)}}

async function api(path,opts={}){const r=await fetch(API+path,{...opts,headers:{"Authorization":"Bearer "+token,"Content-Type":"application/json",...opts.headers}});const d=await r.json();if(d.error&&(d.error.includes("封禁")||d.error.includes("踢出")||d.error.includes("未登录"))){localStorage.clear();location.reload()}return d}

async function login(){
const uid=$("loginUid").value.trim().toUpperCase();
const pwd=$("loginPwd").value;
const errEl=$("loginErr");
const btn=$("loginBtn");
if(!uid){errEl.textContent="请输入UID";return}
if(!pwd){errEl.textContent="请输入密码";return}
btn.disabled=true;
btn.textContent="登录中...";
errEl.textContent="";
try{
const r=await fetch(API+"/api/auth/login",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({uid,password:pwd})});
const d=await r.json();
if(d.success){token=d.data.token;user=d.data.user;localStorage.setItem("t",token);localStorage.setItem("u",JSON.stringify(user));showMain()}
else errEl.textContent=d.error||"登录失败";
}catch(e){errEl.textContent="网络错误: "+e.message}
finally{btn.disabled=false;btn.textContent="进入"}
}

function showMain(){
$("loginPage").classList.add("hidden");
$("mainPage").classList.remove("hidden");
if(user.role==="admin"||(user.permissions&&user.permissions.length>0))$("adminEntry").classList.remove("hidden");
connectWebSocket();
loadMyChannels();
}

async function loadMyChannels(){
try{
const d=await api("/api/groups");
const el=$("myChannels");
if(d.success)el.innerHTML=d.data.length?d.data.map(g=>'<div class="channel-card" data-gid="'+g.id+'"><h3>'+esc(g.name)+'</h3><p>点击进入</p></div>').join(""):'<div class="empty">暂无频道，输入频道名进入</div>';
}catch(e){}
}

function showChat(){$("channelView").classList.add("hidden");$("chatView").classList.remove("hidden");$("adminView").classList.add("hidden");displayedMsgIds.clear();loadMsgs()}
function leaveChat(){groupId=null;$("channelView").classList.remove("hidden");$("chatView").classList.add("hidden");loadMyChannels()}

async function enterChannel(){
const name=$("cipherInput").value.trim();
if(!name){$("cipherErr").textContent="请输入频道名";return}
try{
const d=await api("/api/groups/enter",{method:"POST",body:JSON.stringify({name})});
if(d.success){
groupId=d.data.id;
$("cipherErr").textContent="";
$("cipherInput").value="";
$("chatTitle").textContent=name;
// 显示公告
if(d.data.announcement){
$("groupAnnouncement").textContent="📢 "+d.data.announcement;
$("groupAnnouncement").classList.remove("hidden");
}else{
$("groupAnnouncement").classList.add("hidden");
}
showChat();
}
else $("cipherErr").textContent=d.error||"频道不存在";
}catch(e){$("cipherErr").textContent="网络错误"}
}

let oldestCreatedAt=null;
let isLoadingMore=false;

async function loadMsgs(){
if(!groupId)return;
try{
const d=await api("/api/messages/group/"+groupId);
if(d.success){
const el=$("msgs");
el.innerHTML="";
displayedMsgIds.clear();
d.data.forEach(m=>addMessage(m,false));
el.scrollTop=el.scrollHeight;
// 保存最旧消息时间用于分页
if(d.pagination&&d.pagination.oldestCreatedAt){
oldestCreatedAt=d.pagination.oldestCreatedAt;
}
// 标记所有消息已读
api("/api/messages/group/"+groupId+"/read",{method:"POST"});
}
}catch(e){}
}

async function loadMoreMsgs(){
if(!groupId||isLoadingMore||!oldestCreatedAt)return;
isLoadingMore=true;
try{
const d=await api("/api/messages/group/"+groupId+"?before="+encodeURIComponent(oldestCreatedAt));
if(d.success&&d.data.length>0){
const el=$("msgs");
const oldScrollHeight=el.scrollHeight;
// 在顶部插入旧消息
d.data.reverse().forEach(m=>{
if(!displayedMsgIds.has(m.id)){
displayedMsgIds.add(m.id);
el.innerHTML=renderMessage(m)+el.innerHTML;
}
});
// 保持滚动位置
el.scrollTop=el.scrollHeight-oldScrollHeight;
// 更新最旧消息时间
if(d.pagination&&d.pagination.oldestCreatedAt){
oldestCreatedAt=d.pagination.oldestCreatedAt;
}
}
}catch(e){}
isLoadingMore=false;
}

function addMessage(m,scroll=true){
if(displayedMsgIds.has(m.id))return;
displayedMsgIds.add(m.id);
const el=$("msgs");
el.innerHTML+=renderMessage(m);
if(scroll)el.scrollTop=el.scrollHeight;
}

function renderMessage(m){
const isMe=m.senderId===user.id;
const isOnline=onlineUsers.has(m.senderId);
const avatarHtml=m.senderAvatar?'<img src="'+m.senderAvatar+'" alt="">':m.senderNickname.charAt(0).toUpperCase();
const onlineDot='<span class="online-dot '+(isOnline?"on":"off")+'"></span>';
let contentHtml="";
if(m.msgType==="image")contentHtml='<img class="msg-image" src="'+m.content+'" onclick="window.open(\''+m.content+'\',\'_blank\')" loading="lazy">';
else if(m.msgType==="file"){const size=formatFileSize(m.fileSize);contentHtml='<div class="msg-file"><div class="msg-file-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="#000"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/></svg></div><div class="msg-file-info"><div class="msg-file-name">'+esc(m.fileName||"文件")+'</div><div class="msg-file-size">'+size+'</div></div></div>';}
else contentHtml=esc(m.content);
// 引用消息
let replyHtml="";
if(m.replyTo&&m.replyInfo){
replyHtml='<div class="msg-reply" onclick="scrollToMsg(\''+m.replyTo+'\')"><span class="msg-reply-nick">'+esc(m.replyInfo.senderNickname)+'</span><span class="msg-reply-content">'+esc(m.replyInfo.content.substring(0,50))+(m.replyInfo.content.length>50?"...":"")+'</span></div>';
}
// 添加双击撤回功能（仅限自己发送的消息）
const recallAttr=isMe?' ondblclick="recallMessage(\''+m.id+'\')" title="双击撤回"':"";
return'<div class="msg-row'+(isMe?" me":"")+'" data-mid="'+m.id+'"><div class="msg-avatar" data-sid="'+m.senderId+'" data-nick="'+esc(m.senderNickname)+'">'+avatarHtml+onlineDot+'</div><div class="msg-bubble '+(isMe?"out":"in")+'"'+recallAttr+'><div class="msg-nick">'+esc(m.senderNickname)+'</div>'+replyHtml+contentHtml+'<div class="msg-time">'+formatTime(m.createdAt)+'<button class="reply-btn" onclick="setReply(\''+m.id+'\',\''+esc(m.senderNickname)+'\',\''+esc(m.content.substring(0,30))+'\')">↩</button></div></div></div>';
}

let replyTo=null;
let replyNick=null;

function setReply(msgId,nick,content){
replyTo=msgId;
replyNick=nick;
$("replyPreview").innerHTML='回复 <b>'+esc(nick)+'</b>: '+esc(content.substring(0,30))+(content.length>30?"...":"");
$("replyPreview").classList.remove("hidden");
$("msgInput").focus();
}

function cancelReply(){
replyTo=null;
replyNick=null;
$("replyPreview").classList.add("hidden");
}

function scrollToMsg(msgId){
const el=$("msgs");
const msgEl=el.querySelector('[data-mid="'+msgId+'"]');
if(msgEl){
msgEl.scrollIntoView({behavior:"smooth",block:"center"});
msgEl.classList.add("msg-highlight");
setTimeout(()=>msgEl.classList.remove("msg-highlight"),2000);
}
}

async function recallMessage(msgId){
if(!confirm("确定撤回这条消息？"))return;
try{
const d=await api("/api/messages/"+msgId+"/recall",{method:"POST"});
if(d.success){
const el=$("msgs");
const msgEl=el.querySelector('[data-mid="'+msgId+'"]');
if(msgEl)msgEl.remove();
displayedMsgIds.delete(msgId);
}else{alert(d.error||"撤回失败")}
}catch(e){alert("撤回失败")}
}

function send(){
const now=Date.now();if(now-lastSend<300)return;lastSend=now;
const input=$("msgInput");const content=input.value.trim();
if(!content||!ws)return;
if(content.length>5000){alert("消息太长");return}
ws.send(JSON.stringify({event:"message",data:{group_id:groupId,content,reply_to:replyTo}}));
input.value="";input.style.height="auto";
clearTyping();
cancelReply();
}

// 输入状态
let typingTimeout=null;
let isTyping=false;

function sendTyping(){
if(!ws||isTyping)return;
isTyping=true;
ws.send(JSON.stringify({event:"typing",data:{group_id:groupId}}));
typingTimeout=setTimeout(()=>{isTyping=false;},3000);
}

function clearTyping(){
if(typingTimeout){clearTimeout(typingTimeout);typingTimeout=null;}
isTyping=false;
}

// 显示输入状态
let typingUsers=new Map();
let typingHideTimeout=null;

function showTyping(nickname){
const el=$("typingIndicator");
if(!el)return;
typingUsers.set(nickname,Date.now());
updateTypingDisplay();
}

function updateTypingDisplay(){
const el=$("typingIndicator");
if(!el)return;
// 清理超过3秒的用户
const now=Date.now();
typingUsers=new Map([...typingUsers].filter(([_,t])=>now-t<3000));
if(typingUsers.size===0){
el.textContent="";
el.classList.add("hidden");
}else if(typingUsers.size===1){
el.textContent=[...typingUsers.keys()][0]+" 正在输入...";
el.classList.remove("hidden");
}else{
el.textContent=[...typingUsers.keys()].slice(0,3).join(", ")+" 正在输入...";
el.classList.remove("hidden");
}
}

function showUploadProgress(){$("uploadProgress").classList.remove("hidden")}
function hideUploadProgress(){$("uploadProgress").classList.add("hidden")}

async function uploadImage(e){
const file=e.target.files[0];if(!file)return;
if(file.size>5*1024*1024){alert("文件太大（最大5MB）");return}
showUploadProgress();
const formData=new FormData();
formData.append("file",file);
try{
const r=await fetch(API+"/api/messages/file/"+groupId,{method:"POST",headers:{"Authorization":"Bearer "+token},body:formData});
const d=await r.json();
if(d.success)addMessage(d.data);
else alert(d.error||"上传失败");
}catch(err){alert("上传失败")}
finally{hideUploadProgress()}
e.target.value="";
}

async function uploadFile(e){
const file=e.target.files[0];if(!file)return;
if(file.size>5*1024*1024){alert("文件太大（最大5MB）");return}
showUploadProgress();
const formData=new FormData();
formData.append("file",file);
try{
const r=await fetch(API+"/api/messages/file/"+groupId,{method:"POST",headers:{"Authorization":"Bearer "+token},body:formData});
const d=await r.json();
if(d.success)addMessage(d.data);
else alert(d.error||"上传失败");
}catch(err){alert("上传失败")}
finally{hideUploadProgress()}
e.target.value="";
}

function connectWebSocket(){
const p=location.protocol==="https:"?"wss:":"ws:";
const wsUrl=p+"//"+location.host+"/ws?token="+token;
try{
ws=new WebSocket(wsUrl);
ws.onopen=onWsOpen;
ws.onclose=onWsClose;
ws.onerror=onWsError;
ws.onmessage=onWsMessage;
}catch(e){console.error("WebSocket连接失败:",e);scheduleReconnect()}
}

function onWsOpen(){wsReconnectAttempts=0;wsReconnectDelay=1000;updateStatus("在线","on");startHeartbeat()}
function onWsClose(event){stopHeartbeat();updateStatus("离线","");if(event.code!==1000&&event.code!==1001)scheduleReconnect()}
function onWsError(error){stopHeartbeat()}
function onWsMessage(e){
const m=JSON.parse(e.data);
if(m.event==="pong"){wsLastPong=Date.now();return}
if(m.event==="message"){
if(m.data.senderId)onlineUsers.add(m.data.senderId);
if(m.data.groupId===groupId)addMessage(m.data);
}
if(m.event==="message_recall"){
// 处理消息撤回
if(m.data.groupId===groupId){
const el=$("msgs");
const msgEl=el.querySelector('[data-mid="'+m.data.id+'"]');
if(msgEl)msgEl.remove();
displayedMsgIds.delete(m.data.id);
}
}
if(m.event==="message_read"){
// 处理已读状态更新
if(m.data.groupId===groupId){
updateReadCount(m.data.id,m.data.readCount);
}
}
if(m.event==="typing"){
// 处理输入状态
if(m.data.groupId===groupId&&m.data.userId!==user.id){
showTyping(m.data.nickname);
}
}
}

function updateReadCount(msgId,readCount){
const el=$("msgs");
const msgEl=el.querySelector('[data-mid="'+msgId+'"]');
if(msgEl){
let readEl=msgEl.querySelector(".msg-read");
if(!readEl){
const timeEl=msgEl.querySelector(".msg-time");
if(timeEl)timeEl.innerHTML+='<span class="msg-read">'+readCount+'已读</span>';
}else{readEl.textContent=readCount+'已读';}
}
}

function scheduleReconnect(){
if(wsReconnectAttempts>=wsMaxReconnectAttempts){updateStatus("连接失败","");return}
wsReconnectAttempts++;
const delay=Math.min(wsReconnectDelay*Math.pow(2,wsReconnectAttempts-1),30000);
updateStatus("重连中("+wsReconnectAttempts+")","reconnecting");
setTimeout(()=>{if(!ws||ws.readyState===WebSocket.CLOSED)connectWebSocket()},delay);
}

function startHeartbeat(){
stopHeartbeat();
wsLastPong=Date.now();
wsHeartbeatInterval=setInterval(()=>{
if(ws&&ws.readyState===WebSocket.OPEN){
if(Date.now()-wsLastPong>60000){ws.close();return}
ws.send(JSON.stringify({event:"ping"}));
}
},30000);
}
function stopHeartbeat(){if(wsHeartbeatInterval){clearInterval(wsHeartbeatInterval);wsHeartbeatInterval=null}}
function updateStatus(text,cls){const status=$("status");status.textContent=text;status.className="status";if(cls)status.classList.add(cls)}

function showAdmin(){
$("channelView").classList.add("hidden");
$("chatView").classList.add("hidden");
$("adminView").classList.remove("hidden");
checkAdminPermissions();
loadUsers();loadGroups();loadIps();loadPermissions();
}
function showChannel(){$("channelView").classList.remove("hidden");$("chatView").classList.add("hidden");$("adminView").classList.add("hidden")}
function adminTab(name){
document.querySelectorAll(".admin-tab").forEach(t=>t.classList.remove("on"));
document.querySelectorAll(".admin-section").forEach(s=>s.classList.remove("on"));
event.target.classList.add("on");
$(name+"Section").classList.add("on");
}
function checkAdminPermissions(){
const perms=user.permissions||[];
const hasPerm=(p)=>user.role==="admin"||perms.includes(p);
$("createUserCard").classList.toggle("hidden",!hasPerm("user_create"));
$("createGroupCard").classList.toggle("hidden",!hasPerm("group_create"));
}

async function createUser(){
const uid=$("newUid").value.trim().toUpperCase();
const nick=$("newNick").value.trim();
const pwd=$("newPwd").value;
if(!nick||pwd.length<6){$("userRes").innerHTML='<div class="err">请填写昵称和密码(6位+)</div>';return}
try{
const body={nickname:nick,password:pwd};if(uid)body.uid=uid;
const d=await api("/api/admin/users",{method:"POST",body:JSON.stringify(body)});
if(d.success){
$("userRes").innerHTML='<div class="success">创建成功</div><div style="font-size:12px;margin-top:8px">UID: '+d.data.uid+'<br>昵称: '+d.data.nickname+'<br>密码: '+d.data.password+'</div>';
$("newUid").value="";$("newNick").value="";$("newPwd").value="";loadUsers();
}else $("userRes").innerHTML='<div class="err">'+d.error+'</div>';
}catch(e){$("userRes").innerHTML='<div class="err">网络错误</div>'}
}

async function loadUsers(){
try{
const d=await api("/api/admin/users");
const el=$("userList");
if(d.success){
const perms=user.permissions||[];
const hasPerm=(p)=>user.role==="admin"||perms.includes(p);
el.innerHTML=d.data.length?d.data.map(u=>{
userPermissions[u.uid]=u.permissions||[];
if(u.online)onlineUsers.add(u.id);
let badges='<span class="item-badge '+(u.online?"online":"")+'">'+(u.online?"在线":"离线")+'</span>';
if(u.role==="admin")badges='<span class="item-badge admin">管理员</span>'+badges;
if(u.status==="banned")badges='<span class="item-badge banned">已封禁</span>'+badges;
if(u.mutedUntil&&new Date(u.mutedUntil)>new Date())badges+='<span class="item-badge muted">禁言中</span>';
let actions="";
if(u.role!=="admin"){
if(hasPerm("user_ban"))actions+=(u.status==="banned"?'<button class="btn sm" data-act="unban" data-uid="'+u.uid+'">解封</button>':'<button class="btn sm warn" data-act="ban" data-uid="'+u.uid+'">封禁</button>');
if(hasPerm("user_mute"))actions+='<button class="btn sm warn" data-act="mute" data-uid="'+u.uid+'" data-nick="'+esc(u.nickname)+'">禁言</button>';
if(hasPerm("user_kick"))actions+='<button class="btn sm" data-act="kick" data-uid="'+u.uid+'">踢出</button>';
if(hasPerm("user_kick"))actions+='<button class="btn sm danger" data-act="delete" data-uid="'+u.uid+'">删除</button>';
if(hasPerm("permission_grant"))actions+='<button class="btn sm success" data-act="perm" data-uid="'+u.uid+'" data-nick="'+esc(u.nickname)+'">权限</button>';
}
const permTags=(u.permissions||[]).length?'<div class="permission-list">'+(u.permissions||[]).slice(0,5).map(p=>'<span class="permission-tag">'+p+'</span>').join("")+(u.permissions.length>5?'<span class="permission-tag">+'+(u.permissions.length-5)+'</span>':"")+"</div>":"";
return'<div class="item-card"><div class="item-header"><span class="item-title">'+esc(u.nickname)+'</span>'+badges+'</div><div class="item-info">UID: '+u.uid+(u.lastIp?" | IP: "+u.lastIp:"")+'</div>'+permTags+(actions?'<div class="item-actions">'+actions+'</div>':"")+"</div>";
}).join(""):'<div class="empty">暂无用户</div>';
}
}catch(e){}
}

async function loadGroups(){
try{
const d=await api("/api/admin/groups");
const el=$("groupList");
if(d.success){
const perms=user.permissions||[];
const hasPerm=(p)=>user.role==="admin"||perms.includes(p);
el.innerHTML=d.data.length?d.data.map(g=>{
let actions="";
if(hasPerm("message_delete"))actions+='<button class="btn sm" data-act="clearGroup" data-gid="'+g.id+'">清空</button>';
if(hasPerm("group_delete"))actions+='<button class="btn sm danger" data-act="deleteGroup" data-gid="'+g.id+'">删除</button>';
return'<div class="item-card"><div class="item-header"><span class="item-title">'+esc(g.name)+'</span><span class="item-badge">'+g.memberCount+'人</span></div><div class="item-info">暗号: '+esc(g.name)+'</div>'+(actions?'<div class="item-actions">'+actions+'</div>':"")+"</div>";
}).join(""):'<div class="empty">暂无频道</div>';
}
}catch(e){}
}

async function loadIps(){
try{
const d=await api("/api/admin/ips");
const el=$("ipList");
if(d.success)el.innerHTML=d.data.length?d.data.map(ip=>'<div class="item-card"><div class="item-header"><span class="item-title">'+ip.ip+'</span><button class="btn sm" data-act="unbanIp" data-ip="'+ip.ip+'">解封</button></div><div class="item-info">'+(ip.reason||"")+" | "+ip.createdAt+'</div></div>').join(""):'<div class="empty">暂无封禁IP</div>';
}catch(e){}
}

async function loadPermissions(){
try{
const d=await api("/api/admin/permissions");
if(d.success)allPermissions=d.data;
}catch(e){}
}

async function showGroupInfo(){
if(!groupId)return;
try{
const d=await api("/api/groups/"+groupId);
if(d.success){
const info=d.data;
const isOwner=info.ownerId===user.id;
const isAdmin=user.role==="admin";
let html='<div style="padding:8px">';
html+='<p style="font-size:14px;font-weight:500;margin-bottom:8px">'+esc(info.name)+'</p>';
html+='<p style="font-size:12px;color:var(--muted);margin-bottom:8px">成员: '+info.memberCount+'人</p>';
if(info.description){
html+='<p style="font-size:12px;margin-bottom:8px"><strong>描述:</strong> '+esc(info.description)+'</p>';
}
if(info.announcement){
html+='<p style="font-size:12px;margin-bottom:8px;color:var(--accent)"><strong>公告:</strong> '+esc(info.announcement)+'</p>';
}
if(isOwner||isAdmin){
html+='<hr style="border-color:var(--border);margin:12px 0">';
html+='<input class="input" id="groupDescInput" placeholder="频道描述（最多200字）" value="'+esc(info.description||"")+'">';
html+='<textarea class="input" id="groupAnnInput" placeholder="频道公告（最多500字）" style="margin-top:8px;height:60px">'+esc(info.announcement||"")+'</textarea>';
html+='<button class="btn full" style="margin-top:8px" id="saveGroupInfoBtn">保存</button>';
}
html+='</div>';
$("groupInfoContent").innerHTML=html;
$("groupInfoModal").classList.remove("hidden");
if(isOwner||isAdmin){
$("saveGroupInfoBtn").onclick=saveGroupInfo;
}
}
}catch(e){}
}

async function saveGroupInfo(){
const desc=$("groupDescInput").value.trim();
const ann=$("groupAnnInput").value.trim();
try{
const d=await api("/api/groups/"+groupId,{method:"PUT",body:JSON.stringify({description:desc,announcement:ann})});
if(d.success){
$("groupInfoModal").classList.add("hidden");
// 更新公告显示
if(ann){
$("groupAnnouncement").textContent="📢 "+ann;
$("groupAnnouncement").classList.remove("hidden");
}else{
$("groupAnnouncement").classList.add("hidden");
}
}else{alert(d.error||"保存失败")}
}catch(e){alert("保存失败")}
}

// 搜索功能
let searchTimeout=null;

async function doSearch(){
if(!groupId)return;
const q=$("searchInput").value.trim();
if(q.length<2){
$("searchResults").innerHTML='<div class="empty">请输入至少2个字符</div>';
return;
}
try{
const d=await api("/api/messages/group/"+groupId+"/search?q="+encodeURIComponent(q));
if(d.success){
if(d.data.length===0){
$("searchResults").innerHTML='<div class="empty">未找到相关消息</div>';
}else{
$("searchResults").innerHTML=d.data.map(m=>{
const isMe=m.senderId===user.id;
return'<div class="search-result-item" data-mid="'+m.id+'">'+
'<div class="search-result-nick">'+esc(m.senderNickname)+' <span class="msg-time">'+formatTime(m.createdAt)+'</span></div>'+
'<div class="search-result-content">'+esc(m.content)+'</div>'+
'</div>';
}).join("");
}
}
}catch(e){$("searchResults").innerHTML='<div class="empty">搜索失败</div>'}
}

function showSearch(){
$("searchModal").classList.remove("hidden");
$("searchInput").value="";
$("searchResults").innerHTML="";
$("searchInput").focus();
}

// 成员列表
async function showMembers(){
if(!groupId)return;
try{
const d=await api("/api/groups/"+groupId+"/members");
if(d.success){
const el=$("membersList");
el.innerHTML=d.data.map(m=>{
const avatarHtml=m.avatar?'<img src="'+m.avatar+'" alt="">':m.nickname.charAt(0).toUpperCase();
const roleBadge=m.role==="admin"?'<span class="member-badge admin">管理员</span>':'';
return'<div class="member-item">'+
'<div class="member-avatar">'+avatarHtml+'</div>'+
'<div class="member-info">'+
'<div class="member-nick">'+esc(m.nickname)+roleBadge+'</div>'+
'<div class="member-status '+(m.isOnline?"online":"offline")+'">'+(m.isOnline?"在线":"离线")+'</div>'+
'</div>'+
'</div>';
}).join("");
$("membersModal").classList.remove("hidden");
}
}catch(e){}
}

async function showUserMenu(e,sid,nick){
e.stopPropagation();
const menu=$("userMenu");
const isSelf=sid===user.id;
const perms=user.permissions||[];
const hasPerm=(p)=>user.role==="admin"||perms.includes(p);
let userInfo=null;
try{const d=await api("/api/users/"+sid);if(d.success)userInfo=d.data}catch(err){}
const avatarEl=$("menuAvatar");
if(userInfo&&userInfo.avatar)avatarEl.innerHTML='<img src="'+userInfo.avatar+'" alt="">';
else avatarEl.textContent=nick.charAt(0).toUpperCase();
$("menuUserName").textContent=nick;
$("menuUserInfo").textContent="UID: "+(userInfo?userInfo.uid:sid);
const statusEl=$("menuUserStatus");
if(userInfo){
if(userInfo.status==="banned"){statusEl.textContent="已封禁";statusEl.className="user-menu-status offline"}
else if(userInfo.muted){statusEl.textContent="禁言中";statusEl.className="user-menu-status muted"}
else if(userInfo.online){statusEl.textContent="在线";statusEl.className="user-menu-status online"}
else{statusEl.textContent="离线";statusEl.className="user-menu-status offline"}
}else statusEl.textContent="";
let actionsHtml="";
if(!isSelf){
if(hasPerm("user_mute"))actionsHtml+='<button class="user-menu-item warn" data-act="menuMute">禁言</button>';
if(hasPerm("user_mute"))actionsHtml+='<button class="user-menu-item" data-act="menuUnmute">解除禁言</button>';
if(hasPerm("user_kick"))actionsHtml+='<button class="user-menu-item warn" data-act="menuKick">踢出</button>';
if(hasPerm("user_ban"))actionsHtml+='<button class="user-menu-item danger" data-act="menuBan">封禁</button>';
if(hasPerm("permission_grant"))actionsHtml+='<button class="user-menu-item" data-act="menuGrant">管理权限</button>';
}
$("menuActions").innerHTML=actionsHtml;
menuTargetUser={uid:userInfo?userInfo.uid:sid,userId:sid,nick};
menu.style.left=Math.min(e.clientX,window.innerWidth-200)+"px";
menu.style.top=Math.min(e.clientY,window.innerHeight-300)+"px";
menu.classList.remove("hidden");
}

function closeUserMenu(){$("userMenu").classList.add("hidden")}
function openPermModal(uid,nick){menuTargetUser={uid,nick};$("permUserName").textContent=nick;renderPermGrid(uid);$("permModal").classList.remove("hidden")}
function renderPermGrid(uid){
const grid=$("permGrid");
const currentPerms=userPermissions[uid]||[];
grid.innerHTML=allPermissions.map(p=>'<div class="permission-item"><label><input type="checkbox" '+(currentPerms.includes(p.name)?"checked":"")+' data-perm="'+p.name+'"> '+p.description+'</label></div>').join("");
}

async function savePermissions(){
if(!menuTargetUser)return;
const checkboxes=document.querySelectorAll("#permGrid input[type='checkbox']");
const permToGrant=[],permToRevoke=[];
checkboxes.forEach(cb=>{
const permName=cb.dataset.perm;
const currentPerms=userPermissions[menuTargetUser.uid]||[];
if(cb.checked&&!currentPerms.includes(permName))permToGrant.push(permName);
else if(!cb.checked&¤tPerms.includes(permName))permToRevoke.push(permName);
});
for(const permName of permToGrant)await api("/api/admin/users/"+menuTargetUser.uid+"/permissions",{method:"POST",body:JSON.stringify({permission_name:permName})});
for(const permName of permToRevoke)await api("/api/admin/users/"+menuTargetUser.uid+"/permissions",{method:"DELETE",body:JSON.stringify({permission_name:permName})});
$("permModal").classList.add("hidden");
loadUsers();
}

function openMuteModal(uid,nick){
menuTargetUser={uid,nick};
$("muteUserName").textContent=nick;
const durations=[5,30,60,360,1440];
$("muteOptions").innerHTML=durations.map(d=>'<div class="mute-option" data-duration="'+d+'">'+(d<60?d+"分钟":d===60?"1小时":d===360?"6小时":"1天")+'</div>').join("");
selectedMuteDuration=30;
$("muteModal").classList.remove("hidden");
}

document.addEventListener("click",async function(e){
const t=e.target;
const act=t.dataset.act;
if(act==="ban"&&confirm("确定封禁该用户?")){await api("/api/admin/users/"+t.dataset.uid+"/ban",{method:"PUT"});loadUsers()}
if(act==="unban"){await api("/api/admin/users/"+t.dataset.uid+"/unban",{method:"PUT"});loadUsers()}
if(act==="kick"){await api("/api/admin/users/"+t.dataset.uid+"/kick",{method:"PUT"});alert("已踢出")}
if(act==="delete"&&confirm("确定删除该用户?")){await api("/api/admin/users/"+t.dataset.uid,{method:"DELETE"});loadUsers()}
if(act==="mute")openMuteModal(t.dataset.uid,t.dataset.nick);
if(act==="perm")openPermModal(t.dataset.uid,t.dataset.nick);
if(act==="clearGroup"&&confirm("确定清空该频道所有消息?")){await api("/api/messages/group/"+t.dataset.gid,{method:"DELETE"});alert("已清空")}
if(act==="deleteGroup"&&confirm("确定删除该频道?")){await api("/api/admin/groups/"+t.dataset.gid,{method:"DELETE"});loadGroups()}
if(act==="unbanIp"){await api("/api/admin/ips/"+t.dataset.ip,{method:"DELETE"});loadIps()}
if(act==="menuMute"){closeUserMenu();if(menuTargetUser)openMuteModal(menuTargetUser.uid,menuTargetUser.nick)}
if(act==="menuUnmute"){closeUserMenu();if(menuTargetUser){await api("/api/admin/users/"+menuTargetUser.uid+"/unmute",{method:"PUT"});alert("已解除禁言")}}
if(act==="menuKick"){closeUserMenu();if(menuTargetUser){await api("/api/admin/users/"+menuTargetUser.uid+"/kick",{method:"PUT"});alert("已踢出")}}
if(act==="menuBan"){closeUserMenu();if(menuTargetUser&&confirm("确定封禁该用户?")){await api("/api/admin/users/"+menuTargetUser.uid+"/ban",{method:"PUT"});alert("已封禁")}}
if(act==="menuGrant"){closeUserMenu();if(menuTargetUser)openPermModal(menuTargetUser.uid,menuTargetUser.nick)}
if(t.closest(".channel-card")){const gid=t.closest(".channel-card").dataset.gid;if(gid){groupId=gid;showChat()}}
if(t.classList.contains("mute-option")){selectedMuteDuration=parseInt(t.dataset.duration);document.querySelectorAll(".mute-option").forEach(el=>el.classList.remove("on"));t.classList.add("on")}
if(t.classList.contains("admin-tab"))adminTab(t.dataset.tab);
if(t.closest(".msg-avatar")){const av=t.closest(".msg-avatar");showUserMenu(e,av.dataset.sid,av.dataset.nick)}
if(!$("userMenu").contains(t)&&!t.closest(".msg-avatar"))closeUserMenu();
});

window.onload=function(){
initTheme();
typeWriter($("logoText"),"ARCANUM",0);
typeWriter($("logoText2"),"ARCANUM",0);
const t=localStorage.getItem("t"),u=localStorage.getItem("u");
if(t&&u){try{token=t;user=JSON.parse(u);showMain()}catch(e){localStorage.clear()}}
$("loginBtn").onclick=login;
$("themeToggle").onclick=toggleTheme;
$("loginPwd").onkeydown=function(e){if(e.key==="Enter")login()};
$("enterChannelBtn").onclick=enterChannel;
$("cipherInput").onkeydown=function(e){if(e.key==="Enter")enterChannel()};
$("leaveChatBtn").onclick=leaveChat;
$("sendBtn").onclick=send;
$("msgInput").onkeydown=function(e){if(e.key==="Enter"&&!e.shiftKey){e.preventDefault();send()}};
$("msgInput").oninput=function(){this.style.height="auto";this.style.height=Math.min(this.scrollHeight,80)+"px";sendTyping()};
$("imageInput").onchange=uploadImage;
$("fileInput").onchange=uploadFile;
$("showAdminBtn").onclick=showAdmin;
$("showChannelBtn").onclick=showChannel;
$("createUserBtn").onclick=createUser;
$("createChannelBtn").onclick=function(){
const name=$("newChan").value.trim();if(!name)return;
api("/api/groups",{method:"POST",body:JSON.stringify({name})}).then(d=>{
if(d.success){$("chanRes").innerHTML='<div class="success">创建成功: '+name+'</div>';$("newChan").value="";loadGroups()}
else $("chanRes").innerHTML='<div class="err">'+d.error+'</div>';
});
};
$("closeUserMenuBtn").onclick=closeUserMenu;
$("closePermModalBtn").onclick=function(){$("permModal").classList.add("hidden")};
$("closeMuteModalBtn").onclick=function(){$("muteModal").classList.add("hidden")};
$("savePermsBtn").onclick=savePermissions;
$("confirmMuteBtn").onclick=async function(){if(!menuTargetUser)return;await api("/api/admin/users/"+menuTargetUser.uid+"/mute",{method:"PUT",body:JSON.stringify({duration_minutes:selectedMuteDuration})});$("muteModal").classList.add("hidden");loadUsers()};
$("permModal").onclick=function(e){if(e.target===this)$("permModal").classList.add("hidden")};
$("muteModal").onclick=function(e){if(e.target===this)$("muteModal").classList.add("hidden")};
$("closeGroupInfoModalBtn").onclick=function(){$("groupInfoModal").classList.add("hidden")};
$("groupInfoModal").onclick=function(e){if(e.target===this)$("groupInfoModal").classList.add("hidden")};
$("groupInfoBtn").onclick=showGroupInfo;
$("closeSearchModalBtn").onclick=function(){$("searchModal").classList.add("hidden")};
$("searchModal").onclick=function(e){if(e.target===this)$("searchModal").classList.add("hidden")};
$("searchBtn").onclick=showSearch;
$("closeMembersModalBtn").onclick=function(){$("membersModal").classList.add("hidden")};
$("membersModal").onclick=function(e){if(e.target===this)$("membersModal").classList.add("hidden")};
$("membersBtn").onclick=showMembers;
$("searchInput").oninput=function(){
clearTimeout(searchTimeout);
searchTimeout=setTimeout(doSearch,300);
};
// 滚动加载更多消息
$("msgs").addEventListener("scroll",function(){
if(this.scrollTop<50&&!isLoadingMore){
loadMoreMsgs();
}
});
};
})();
</script>
</body>
</html>
"##;
