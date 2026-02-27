pub const INDEX_HTML: &str = r##"<!DOCTYPE html>
<html lang="zh">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1.0,maximum-scale=1.0,user-scalable=no">
<title>ARCANUM</title>
<style>
*{margin:0;padding:0;box-sizing:border-box}
:root{--bg:#000;--card:#0d0d0d;--text:#f0f0f0;--muted:#666;--accent:#0ff;--accent2:#f0f;--border:#1a1a1a;--error:#ff4466;--success:#00ff88;--warn:#ffaa00}
body{background:var(--bg);color:var(--text);font-family:-apple-system,sans-serif;min-height:100vh;min-height:100dvh;line-height:1.4}
@keyframes glitch{0%,100%{text-shadow:-2px 0 var(--accent2),2px 0 var(--accent)}25%{text-shadow:2px 0 var(--accent2),-2px 0 var(--accent)}50%{text-shadow:-1px 0 var(--accent2),1px 0 var(--accent)}75%{text-shadow:1px 0 var(--accent2),-1px 0 var(--accent)}}
.glitch{animation:glitch .3s infinite}
.scanlines::before{content:"";position:fixed;inset:0;background:repeating-linear-gradient(0deg,rgba(0,0,0,.06),rgba(0,0,0,.06) 1px,transparent 1px,transparent 2px);pointer-events:none;z-index:9999}
@keyframes blink{0%,50%{border-color:var(--accent)}51%,100%{border-color:transparent}}
.typewriter{border-right:2px solid var(--accent);animation:blink 1s infinite}
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
@keyframes spin{to{transform:rotate(360deg)}}
@media(min-width:540px){.container{padding:16px}}
</style>
</head>
<body class="scanlines">
<div class="status" id="status">离线</div>

<div id="loginPage" class="container">
<div class="logo glitch"><span id="logoText"></span></div>
<div class="card">
<input class="input" id="loginUid" placeholder="UID" autocapitalize="characters" style="margin-bottom:8px">
<input class="input" type="password" id="loginPwd" placeholder="密码" style="margin-bottom:8px">
<button class="btn full" onclick="login()">进入</button>
<div class="err" id="loginErr"></div>
</div>
</div>

<div id="mainPage" class="container hidden">
<div class="logo glitch"><span id="logoText2"></span></div>

<div id="channelView">
<div class="card">
<input class="input" id="cipherInput" placeholder="输入频道名进入..." autocapitalize="off" onkeydown="if(event.key==='Enter')enterChannel()">
<button class="btn full" onclick="enterChannel()" style="margin-top:8px">进入频道</button>
<div class="err" id="cipherErr"></div>
</div>
<div id="myChannels"></div>
<div class="card hidden" id="adminEntry"><button class="btn full" onclick="showAdmin()">管理面板</button></div>
</div>

<div id="chatView" class="hidden">
<div class="chat-wrap">
<div class="chat-header">
<button class="chat-back" onclick="leaveChat()">←</button>
<h2 id="chatTitle">聊天</h2>
<div style="width:30px"></div>
</div>
<div class="chat-msgs" id="msgs"></div>
<div class="chat-input">
<textarea id="msgInput" rows="1" placeholder="消息..." onkeydown="handleKey(event)"></textarea>
<div class="chat-actions">
<label class="chat-action-btn" title="上传图片" id="imageUploadBtn">
<input type="file" accept="image/*" id="imageInput" style="display:none" onchange="uploadImage(event)">
<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2"/><circle cx="8.5" cy="8.5" r="1.5"/><path d="M21 15l-5-5L5 21"/></svg>
</label>
<label class="chat-action-btn" title="上传文件" id="fileUploadBtn">
<input type="file" accept=".txt" id="fileInput" style="display:none" onchange="uploadFile(event)">
<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><path d="M14 2v6h6"/><path d="M12 18v-6"/><path d="M9 15l3-3 3 3"/></svg>
</label>
</div>
<button class="send-btn" onclick="send()"><svg viewBox="0 0 24 24"><path d="M2 21l21-9L2 3v7l15 2-15 2v7z"/></svg></button>
</div>
</div>
</div>

<div id="adminView" class="hidden">
<div class="admin-tabs">
<div class="admin-tab on" onclick="adminTab('users')">用户</div>
<div class="admin-tab" onclick="adminTab('groups')">频道</div>
<div class="admin-tab" onclick="adminTab('ips')">IP</div>
</div>

<div id="usersSection" class="admin-section on">
<div class="card" id="createUserCard">
<h3 style="font-size:13px;color:var(--accent);margin-bottom:10px">创建用户</h3>
<div class="admin-form">
<input class="input" id="newUid" placeholder="UID (留空自动生成)" autocapitalize="characters">
<input class="input" id="newNick" placeholder="昵称">
<input class="input" id="newPwd" placeholder="密码 (6位+)">
<button class="btn full" onclick="createUser()">创建</button>
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
<button class="btn full" onclick="createChannel()">创建</button>
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

<button class="btn full" onclick="showChannel()" style="margin-top:12px">返回频道</button>
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
<button class="user-menu-item" onclick="closeUserMenu()">关闭</button>
</div>

<div class="modal-overlay hidden" id="permModal" onclick="if(event.target===this)closePermModal()">
<div class="modal">
<div class="modal-header">
<h3>管理用户权限</h3>
<button class="modal-close" onclick="closePermModal()">×</button>
</div>
<p style="font-size:12px;color:var(--muted);margin-bottom:8px">用户: <span id="permUserName" style="color:var(--accent)"></span></p>
<p style="font-size:11px;color:var(--muted);margin-bottom:12px">提示: 勾选权限后点击保存即可授权，取消勾选可撤销权限</p>
<div class="permission-grid" id="permGrid"></div>
<button class="btn full success" style="margin-top:12px" onclick="savePermissions()">保存权限</button>
</div>
</div>

<div class="modal-overlay hidden" id="muteModal" onclick="if(event.target===this)closeMuteModal()">
<div class="modal">
<div class="modal-header">
<h3>禁言用户</h3>
<button class="modal-close" onclick="closeMuteModal()">×</button>
</div>
<p style="font-size:12px;color:var(--muted);margin-bottom:12px">用户: <span id="muteUserName" style="color:var(--accent)"></span></p>
<div class="mute-options">
<div class="mute-option" onclick="selectMuteDuration(5)">5分钟</div>
<div class="mute-option" onclick="selectMuteDuration(30)">30分钟</div>
<div class="mute-option" onclick="selectMuteDuration(60)">1小时</div>
<div class="mute-option" onclick="selectMuteDuration(360)">6小时</div>
<div class="mute-option" onclick="selectMuteDuration(1440)">1天</div>
</div>
<button class="btn full warn" style="margin-top:12px" onclick="confirmMute()">确认禁言</button>
</div>
</div>

<div class="upload-progress hidden" id="uploadProgress"><span class="loading-spinner"></span>上传中...</div>

<script>
let token="",user=null,ws=null,groupId=null,lastSend=0;
let menuTargetUser=null;
let selectedMuteDuration=30;
let allPermissions=[];
let userPermissions={};
let onlineUsers=new Set();
const API=location.origin;

// WebSocket 重连机制
let wsReconnectAttempts=0;
let wsMaxReconnectAttempts=10;
let wsReconnectDelay=1000;
let wsHeartbeatInterval=null;
let wsLastPong=0;

function debounce(fn,delay){let t;return function(...args){clearTimeout(t);t=setTimeout(()=>fn.apply(this,args),delay)}}
function throttle(fn,delay){let last=0;return function(...args){const now=Date.now();if(now-last>=delay){last=now;fn.apply(this,args)}}}

function typeWriter(el,text,i){if(i<text.length){el.textContent=text.substring(0,i+1);setTimeout(()=>typeWriter(el,text,i+1),100)}}
function startLogo(){typeWriter(document.getElementById("logoText"),"ARCANUM",0);typeWriter(document.getElementById("logoText2"),"ARCANUM",0)}

async function api(path,opts={}){const r=await fetch(API+path,{...opts,headers:{"Authorization":"Bearer "+token,"Content-Type":"application/json",...opts.headers}});const d=await r.json();if(d.error&&(d.error.includes("封禁")||d.error.includes("踢出")||d.error.includes("未登录"))){localStorage.clear();location.reload()}return d}

async function login(){
const uid=document.getElementById("loginUid").value.trim().toUpperCase();
const pwd=document.getElementById("loginPwd").value;
if(!uid||!pwd)return;
try{
const r=await fetch(API+"/api/auth/login",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({uid,password:pwd})});
const d=await r.json();
if(d.success){token=d.data.token;user=d.data.user;localStorage.setItem("t",token);localStorage.setItem("u",JSON.stringify(user));showMain()}
else document.getElementById("loginErr").textContent=d.error;
}catch(e){document.getElementById("loginErr").textContent="网络错误"}
}

function showMain(){
document.getElementById("loginPage").classList.add("hidden");
document.getElementById("mainPage").classList.remove("hidden");
if(user.role==="admin"||user.permissions&&user.permissions.length>0){
document.getElementById("adminEntry").classList.remove("hidden");
}
connectWebSocket();loadMyChannels();
}

async function enterChannel(){
const name=document.getElementById("cipherInput").value.trim();
if(!name){document.getElementById("cipherErr").textContent="请输入频道名";return}
try{
const d=await api("/api/groups/enter",{method:"POST",body:JSON.stringify({name})});
if(d.success){groupId=d.data.id;document.getElementById("cipherErr").textContent="";document.getElementById("cipherInput").value="";showChat()}
else{document.getElementById("cipherErr").textContent=d.error||"频道不存在"}
}catch(e){document.getElementById("cipherErr").textContent="网络错误"}
}

async function loadMyChannels(){
try{
const d=await api("/api/groups");
const el=document.getElementById("myChannels");
if(d.success){el.innerHTML=d.data.length?d.data.map(g=>"<div class=\"channel-card\" onclick=\"joinChannel('"+g.id+"')\"><h3>"+esc(g.name)+"</h3><p>点击进入</p></div>").join(""):"<div class=\"empty\">暂无频道，输入频道名进入</div>"}
}catch(e){}
}

function joinChannel(id){groupId=id;showChat()}
function showChat(){document.getElementById("channelView").classList.add("hidden");document.getElementById("chatView").classList.remove("hidden");document.getElementById("adminView").classList.add("hidden");loadMsgs()}
function leaveChat(){groupId=null;document.getElementById("channelView").classList.remove("hidden");document.getElementById("chatView").classList.add("hidden");loadMyChannels()}

async function loadMsgs(){
if(!groupId)return;
try{
const d=await api("/api/messages/group/"+groupId);
if(d.success){
const el=document.getElementById("msgs");
el.innerHTML=d.data.map(m=>renderMessage(m)).join("");
el.scrollTop=el.scrollHeight;
}
}catch(e){}
}

function renderMessage(m){
const isMe=m.senderId===user.id;
const isOnline=onlineUsers.has(m.senderId);
const avatarHtml=m.senderAvatar?"<img src=\""+m.senderAvatar+"\" alt=\"\">":m.senderNickname.charAt(0).toUpperCase();
const onlineDot="<span class=\"online-dot "+(isOnline?"on":"off")+"\"></span>";
let contentHtml="";
if(m.msgType==="image"){
contentHtml="<img class=\"msg-image\" src=\""+m.content+"\" onclick=\"viewImage('"+m.content+"')\" loading=\"lazy\">";
}
else if(m.msgType==="file"){
const size=formatFileSize(m.fileSize);
contentHtml="<div class=\"msg-file\"><div class=\"msg-file-icon\"><svg width=\"16\" height=\"16\" viewBox=\"0 0 24 24\" fill=\"#000\"><path d=\"M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z\"/></svg></div><div class=\"msg-file-info\"><div class=\"msg-file-name\">"+esc(m.fileName||"文件")+"</div><div class=\"msg-file-size\">"+size+"</div></div></div>";
}
else{
contentHtml=esc(m.content);
}
return"<div class=\"msg-row"+(isMe?" me":"")+"\"><div class=\"msg-avatar\" onclick=\"showUserMenu(event,'"+m.senderId+"','"+esc(m.senderNickname)+"')\">"+avatarHtml+onlineDot+"</div><div class=\"msg-bubble "+(isMe?"out":"in")+"\"><div class=\"msg-nick\">"+esc(m.senderNickname)+"</div>"+contentHtml+"<div class=\"msg-time\">"+formatTime(m.createdAt)+"</div></div></div>";
}

const send=throttle(function(){
const now=Date.now();if(now-lastSend<300)return;lastSend=now;
const input=document.getElementById("msgInput");const content=input.value.trim();
if(!content||!ws)return;if(content.length>5000){alert("消息太长");return}
ws.send(JSON.stringify({event:"message",data:{group_id:groupId,content}}));
input.value="";input.style.height="auto";
},300);

function handleKey(e){if(e.key==="Enter"&&!e.shiftKey){e.preventDefault();send()}}

function showUploadProgress(){document.getElementById("uploadProgress").classList.remove("hidden")}
function hideUploadProgress(){document.getElementById("uploadProgress").classList.add("hidden")}

async function uploadImage(e){
const file=e.target.files[0];if(!file)return;
if(file.size>5*1024*1024){alert("文件太大（最大5MB）");return}
showUploadProgress();
const formData=new FormData();
formData.append("file",file);
try{
const r=await fetch(API+"/api/messages/file/"+groupId,{method:"POST",headers:{"Authorization":"Bearer "+token},body:formData});
const d=await r.json();
if(d.success){
ws.send(JSON.stringify({event:"message",data:{group_id:groupId,content:d.data.content,msg_type:"image",file_name:d.data.fileName,file_size:d.data.fileSize}}));
}else{alert(d.error||"上传失败")}
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
if(d.success){
ws.send(JSON.stringify({event:"message",data:{group_id:groupId,content:d.data.content,msg_type:"file",file_name:d.data.fileName,file_size:d.data.fileSize}}));
}else{alert(d.error||"上传失败")}
}catch(err){alert("上传失败")}
finally{hideUploadProgress()}
e.target.value="";
}

// WebSocket 连接与重连
function connectWebSocket(){
const p=location.protocol==="https:"?"wss:":"ws:";
const wsUrl=p+"//"+location.host+"/ws?token="+token;
try{
ws=new WebSocket(wsUrl);
ws.onopen=onWsOpen;
ws.onclose=onWsClose;
ws.onerror=onWsError;
ws.onmessage=onWsMessage;
}catch(e){
console.error("WebSocket连接失败:",e);
scheduleReconnect();
}
}

function onWsOpen(){
console.log("WebSocket已连接");
wsReconnectAttempts=0;
wsReconnectDelay=1000;
updateStatus("在线","on");
startHeartbeat();
}

function onWsClose(event){
console.log("WebSocket已断开:",event.code,event.reason);
stopHeartbeat();
updateStatus("离线","");
if(event.code!==1000&&event.code!==1001){
scheduleReconnect();
}
}

function onWsError(error){
console.error("WebSocket错误:",error);
stopHeartbeat();
}

function onWsMessage(e){
const m=JSON.parse(e.data);
if(m.event==="pong"){
wsLastPong=Date.now();
return;
}
if(m.event==="message"){
if(m.data.senderId){
onlineUsers.add(m.data.senderId);
}
if(m.data.groupId===groupId){
const el=document.getElementById("msgs");
el.innerHTML+=renderMessage(m.data);
el.scrollTop=el.scrollHeight;
}
}
}

function scheduleReconnect(){
if(wsReconnectAttempts>=wsMaxReconnectAttempts){
console.log("达到最大重连次数，停止重连");
updateStatus("连接失败","");
return;
}
wsReconnectAttempts++;
const delay=Math.min(wsReconnectDelay*Math.pow(2,wsReconnectAttempts-1),30000);
console.log("将在"+delay+"ms后重连，第"+wsReconnectAttempts+"次");
updateStatus("重连中("+wsReconnectAttempts+")","reconnecting");
setTimeout(()=>{
if(!ws||ws.readyState===WebSocket.CLOSED){
connectWebSocket();
}
},delay);
}

function startHeartbeat(){
stopHeartbeat();
wsLastPong=Date.now();
wsHeartbeatInterval=setInterval(()=>{
if(ws&&ws.readyState===WebSocket.OPEN){
if(Date.now()-wsLastPong>60000){
console.log("心跳超时，重连");
ws.close();
return;
}
ws.send(JSON.stringify({event:"ping"}));
}
},30000);
}

function stopHeartbeat(){
if(wsHeartbeatInterval){
clearInterval(wsHeartbeatInterval);
wsHeartbeatInterval=null;
}
}

function updateStatus(text,cls){
const status=document.getElementById("status");
status.textContent=text;
status.className="status";
if(cls)status.classList.add(cls);
}

function showAdmin(){
document.getElementById("channelView").classList.add("hidden");
document.getElementById("chatView").classList.add("hidden");
document.getElementById("adminView").classList.remove("hidden");
checkAdminPermissions();
loadUsers();loadGroups();loadIps();loadPermissions();
}
function showChannel(){document.getElementById("channelView").classList.remove("hidden");document.getElementById("chatView").classList.add("hidden");document.getElementById("adminView").classList.add("hidden")}
function adminTab(name){
document.querySelectorAll(".admin-tab").forEach(t=>t.classList.remove("on"));
document.querySelectorAll(".admin-section").forEach(s=>s.classList.remove("on"));
event.target.classList.add("on");
document.getElementById(name+"Section").classList.add("on");
}

function checkAdminPermissions(){
const perms=user.permissions||[];
const hasPerm=(p)=>user.role==="admin"||perms.includes(p);
document.getElementById("createUserCard").classList.toggle("hidden",!hasPerm("user_create"));
document.getElementById("createGroupCard").classList.toggle("hidden",!hasPerm("group_create"));
}

async function createUser(){
const uid=document.getElementById("newUid").value.trim().toUpperCase();
const nick=document.getElementById("newNick").value.trim();
const pwd=document.getElementById("newPwd").value;
if(!nick||pwd.length<6){document.getElementById("userRes").innerHTML="<div class=\"err\">请填写昵称和密码(6位+)</div>";return}
try{
const body={nickname:nick,password:pwd};if(uid)body.uid=uid;
const d=await api("/api/admin/users",{method:"POST",body:JSON.stringify(body)});
if(d.success){
document.getElementById("userRes").innerHTML="<div class=\"success\">创建成功</div><div style=\"font-size:12px;margin-top:8px\">UID: "+d.data.uid+"<br>昵称: "+d.data.nickname+"<br>密码: "+d.data.password+"</div>";
document.getElementById("newUid").value="";document.getElementById("newNick").value="";document.getElementById("newPwd").value="";loadUsers();
}else{document.getElementById("userRes").innerHTML="<div class=\"err\">"+d.error+"</div>"}
}catch(e){document.getElementById("userRes").innerHTML="<div class=\"err\">网络错误</div>"}
}

async function loadUsers(){
try{
const d=await api("/api/admin/users");
const el=document.getElementById("userList");
if(d.success){
const perms=user.permissions||[];
const hasPerm=(p)=>user.role==="admin"||perms.includes(p);
el.innerHTML=d.data.length?d.data.map(u=>{
userPermissions[u.uid]=u.permissions||[];
if(u.online)onlineUsers.add(u.id);
let badges="<span class=\"item-badge "+(u.online?"online":"")+"\">"+(u.online?"在线":"离线")+"</span>";
if(u.role==="admin")badges="<span class=\"item-badge admin\">管理员</span>"+badges;
if(u.status==="banned")badges="<span class=\"item-badge banned\">已封禁</span>"+badges;
if(u.mutedUntil&&new Date(u.mutedUntil)>new Date())badges+="<span class=\"item-badge muted\">禁言中</span>";
let actions="";
if(u.role!=="admin"){
if(hasPerm("user_ban"))actions+=(u.status==="banned"?"<button class=\"btn sm\" onclick=\"unbanUser('"+u.uid+"')\">解封</button>":"<button class=\"btn sm warn\" onclick=\"banUser('"+u.uid+"')\">封禁</button>");
if(hasPerm("user_mute"))actions+="<button class=\"btn sm warn\" onclick=\"muteUser('"+u.uid+"','"+esc(u.nickname)+"')\">禁言</button>";
if(hasPerm("user_kick"))actions+="<button class=\"btn sm\" onclick=\"kickUser('"+u.uid+"')\">踢出</button>";
if(hasPerm("user_kick"))actions+="<button class=\"btn sm danger\" onclick=\"deleteUser('"+u.uid+"')\">删除</button>";
if(hasPerm("permission_grant"))actions+="<button class=\"btn sm success\" onclick=\"openPermModal('"+u.uid+"','"+esc(u.nickname)+"')\">权限</button>";
}
const permTags=(u.permissions||[]).length?"<div class=\"permission-list\">"+(u.permissions||[]).slice(0,5).map(p=>"<span class=\"permission-tag\">"+p+"</span>").join("")+(u.permissions.length>5?"<span class=\"permission-tag\">+"+(u.permissions.length-5)+"</span>":"")+"</div>":"";
return"<div class=\"item-card\"><div class=\"item-header\"><span class=\"item-title\">"+esc(u.nickname)+"</span>"+badges+"</div><div class=\"item-info\">UID: "+u.uid+(u.lastIp?" | IP: "+u.lastIp:"")+"</div>"+permTags+(actions?"<div class=\"item-actions\">"+actions+"</div>":"")+"</div>";
}).join(""):"<div class=\"empty\">暂无用户</div>"
}
}catch(e){}
}

async function banUser(uid){if(!confirm("确定封禁该用户?"))return;try{await api("/api/admin/users/"+uid+"/ban",{method:"PUT"});loadUsers()}catch(e){}}
async function unbanUser(uid){try{await api("/api/admin/users/"+uid+"/unban",{method:"PUT"});loadUsers()}catch(e){}}
async function kickUser(uid){try{await api("/api/admin/users/"+uid+"/kick",{method:"PUT"});alert("已踢出")}catch(e){}}
async function deleteUser(uid){if(!confirm("确定删除该用户?"))return;try{await api("/api/admin/users/"+uid,{method:"DELETE"});loadUsers()}catch(e){}}

function muteUser(uid,nick){
menuTargetUser={uid,nick};
document.getElementById("muteUserName").textContent=nick;
document.getElementById("muteModal").classList.remove("hidden");
}
function selectMuteDuration(mins){
selectedMuteDuration=mins;
document.querySelectorAll(".mute-option").forEach(el=>el.classList.remove("on"));
event.target.classList.add("on");
}
async function confirmMute(){
if(!menuTargetUser)return;
try{
await api("/api/admin/users/"+menuTargetUser.uid+"/mute",{method:"PUT",body:JSON.stringify({duration_minutes:selectedMuteDuration})});
closeMuteModal();loadUsers();
}catch(e){}
}
function closeMuteModal(){document.getElementById("muteModal").classList.add("hidden");menuTargetUser=null}

async function createChannel(){
const name=document.getElementById("newChan").value.trim();if(!name)return;
try{
const d=await api("/api/groups",{method:"POST",body:JSON.stringify({name})});
if(d.success){document.getElementById("chanRes").innerHTML="<div class=\"success\">创建成功: "+name+"</div>";document.getElementById("newChan").value="";loadGroups()}
else{document.getElementById("chanRes").innerHTML="<div class=\"err\">"+d.error+"</div>"}
}catch(e){}
}

async function loadGroups(){
try{
const d=await api("/api/admin/groups");
const el=document.getElementById("groupList");
if(d.success){
const perms=user.permissions||[];
const hasPerm=(p)=>user.role==="admin"||perms.includes(p);
el.innerHTML=d.data.length?d.data.map(g=>{
let actions="";
if(hasPerm("message_delete"))actions+="<button class=\"btn sm\" onclick=\"clearGroup('"+g.id+"')\">清空</button>";
if(hasPerm("group_delete"))actions+="<button class=\"btn sm danger\" onclick=\"deleteGroup('"+g.id+"')\">删除</button>";
return"<div class=\"item-card\"><div class=\"item-header\"><span class=\"item-title\">"+esc(g.name)+"</span><span class=\"item-badge\">"+g.memberCount+"人</span></div><div class=\"item-info\">暗号: "+esc(g.name)+"</div>"+(actions?"<div class=\"item-actions\">"+actions+"</div>":"")+"</div>";
}).join(""):"<div class=\"empty\">暂无频道</div>"
}
}catch(e){}
}

async function clearGroup(id){if(!confirm("确定清空该频道所有消息?"))return;try{await api("/api/messages/group/"+id,{method:"DELETE"});alert("已清空")}catch(e){}}
async function deleteGroup(id){if(!confirm("确定删除该频道?"))return;try{await api("/api/admin/groups/"+id,{method:"DELETE"});loadGroups()}catch(e){}}

async function loadIps(){
try{
const d=await api("/api/admin/ips");
const el=document.getElementById("ipList");
if(d.success){el.innerHTML=d.data.length?d.data.map(ip=>"<div class=\"item-card\"><div class=\"item-header\"><span class=\"item-title\">"+ip.ip+"</span><button class=\"btn sm\" onclick=\"unbanIp('"+ip.ip+"')\">解封</button></div><div class=\"item-info\">"+(ip.reason||"")+" | "+ip.createdAt+"</div></div>").join(""):"<div class=\"empty\">暂无封禁IP</div>"}
}catch(e){}
}
async function unbanIp(ip){try{await api("/api/admin/ips/"+ip,{method:"DELETE"});loadIps()}catch(e){}}

async function loadPermissions(){
try{
const d=await api("/api/admin/permissions");
if(d.success){allPermissions=d.data}
}catch(e){}
}

// 用户菜单 - 显示在线状态
async function showUserMenu(e,userId,nick){
e.stopPropagation();
const menu=document.getElementById("userMenu");
const isSelf=userId===user.id;
const perms=user.permissions||[];
const hasPerm=(p)=>user.role==="admin"||perms.includes(p);

// 获取用户信息
let userInfo=null;
try{
const d=await api("/api/users/"+userId);
if(d.success)userInfo=d.data;
}catch(err){}

// 更新头像
const avatarEl=document.getElementById("menuAvatar");
if(userInfo&&userInfo.avatar){
avatarEl.innerHTML="<img src=\""+userInfo.avatar+"\" alt=\"\">";
}else{
avatarEl.textContent=nick.charAt(0).toUpperCase();
}

document.getElementById("menuUserName").textContent=nick;
document.getElementById("menuUserInfo").textContent="UID: "+(userInfo?userInfo.uid:userId);

// 更新在线状态
const statusEl=document.getElementById("menuUserStatus");
if(userInfo){
if(userInfo.status==="banned"){
statusEl.textContent="已封禁";
statusEl.className="user-menu-status offline";
}else if(userInfo.muted){
statusEl.textContent="禁言中";
statusEl.className="user-menu-status muted";
}else if(userInfo.online){
statusEl.textContent="在线";
statusEl.className="user-menu-status online";
}else{
statusEl.textContent="离线";
statusEl.className="user-menu-status offline";
}
}else{
statusEl.textContent="";
}

// 构建操作按钮
let actionsHtml="";
if(!isSelf){
if(hasPerm("user_mute"))actionsHtml+="<button class=\"user-menu-item warn\" onclick=\"menuMute()\">禁言</button>";
if(hasPerm("user_mute"))actionsHtml+="<button class=\"user-menu-item\" onclick=\"menuUnmute()\">解除禁言</button>";
if(hasPerm("user_kick"))actionsHtml+="<button class=\"user-menu-item warn\" onclick=\"menuKick()\">踢出</button>";
if(hasPerm("user_ban"))actionsHtml+="<button class=\"user-menu-item danger\" onclick=\"menuBan()\">封禁</button>";
if(hasPerm("permission_grant"))actionsHtml+="<button class=\"user-menu-item\" onclick=\"menuGrant()\">管理权限</button>";
}
document.getElementById("menuActions").innerHTML=actionsHtml;

menuTargetUser={uid:userInfo?userInfo.uid:userId,userId,nick};
menu.style.left=Math.min(e.clientX,window.innerWidth-200)+"px";
menu.style.top=Math.min(e.clientY,window.innerHeight-300)+"px";
menu.classList.remove("hidden");
}

function closeUserMenu(){document.getElementById("userMenu").classList.add("hidden")}
document.addEventListener("click",closeUserMenu);

async function menuMute(){
closeUserMenu();
if(menuTargetUser){
document.getElementById("muteUserName").textContent=menuTargetUser.nick;
document.getElementById("muteModal").classList.remove("hidden");
}
}
async function menuUnmute(){
closeUserMenu();
if(menuTargetUser){
try{await api("/api/admin/users/"+menuTargetUser.uid+"/unmute",{method:"PUT"});alert("已解除禁言")}catch(e){}
}
}
async function menuKick(){
closeUserMenu();
if(menuTargetUser){
try{await api("/api/admin/users/"+menuTargetUser.uid+"/kick",{method:"PUT"});alert("已踢出")}catch(e){}
}
}
async function menuBan(){
closeUserMenu();
if(menuTargetUser&&confirm("确定封禁该用户?")){
try{await api("/api/admin/users/"+menuTargetUser.uid+"/ban",{method:"PUT"});alert("已封禁")}catch(e){}
}
}
function menuGrant(){
closeUserMenu();
if(menuTargetUser){
openPermModal(menuTargetUser.uid,menuTargetUser.nick);
}
}

function openPermModal(uid,nick){
menuTargetUser={uid,nick};
document.getElementById("permUserName").textContent=nick;
renderPermGrid(uid);
document.getElementById("permModal").classList.remove("hidden");
}

function renderPermGrid(uid){
const grid=document.getElementById("permGrid");
const currentPerms=userPermissions[uid]||[];
grid.innerHTML=allPermissions.map(p=>{
const checked=currentPerms.includes(p.name)?"checked":"";
return"<div class=\"permission-item\"><label><input type=\"checkbox\" "+checked+" data-perm=\""+p.name+"\"> "+p.description+"</label></div>";
}).join("");
}

async function savePermissions(){
if(!menuTargetUser)return;
const checkboxes=document.querySelectorAll("#permGrid input[type=\"checkbox\"]");
const permToGrant=[];
const permToRevoke=[];
checkboxes.forEach(cb=>{
const permName=cb.dataset.perm;
const currentPerms=userPermissions[menuTargetUser.uid]||[];
if(cb.checked&&!currentPerms.includes(permName)){
permToGrant.push(permName);
}else if(!cb.checked&¤tPerms.includes(permName)){
permToRevoke.push(permName);
}
});
for(const permName of permToGrant){
await api("/api/admin/users/"+menuTargetUser.uid+"/permissions",{method:"POST",body:JSON.stringify({permission_name:permName})});
}
for(const permName of permToRevoke){
await api("/api/admin/users/"+menuTargetUser.uid+"/permissions",{method:"DELETE",body:JSON.stringify({permission_name:permName})});
}
closePermModal();
loadUsers();
}
function closePermModal(){document.getElementById("permModal").classList.add("hidden")}

function esc(t){const d=document.createElement("div");d.textContent=t;return d.innerHTML}
function formatTime(t){return new Date(t).toLocaleTimeString("zh-CN",{hour:"2-digit",minute:"2-digit"})}
function formatFileSize(b){if(b<1024)return b+"B";if(b<1024*1024)return(b/1024).toFixed(1)+"KB";return(b/1024/1024).toFixed(1)+"MB"}
function viewImage(src){window.open(src,"_blank")}

window.onload=()=>{
startLogo();
const t=localStorage.getItem("t"),u=localStorage.getItem("u");
if(t&&u){token=t;user=JSON.parse(u);showMain()}
document.getElementById("loginPwd").onkeydown=e=>{if(e.key==="Enter")login()};
};
document.getElementById("msgInput").addEventListener("input",function(){this.style.height="auto";this.style.height=Math.min(this.scrollHeight,80)+"px"});
</script>
</body>
</html>
"##;
