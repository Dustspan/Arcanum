pub const INDEX_HTML: &str = r#"<!DOCTYPE html>
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
.scanlines::before{content:'';position:fixed;inset:0;background:repeating-linear-gradient(0deg,rgba(0,0,0,.06),rgba(0,0,0,.06) 1px,transparent 1px,transparent 2px);pointer-events:none;z-index:9999}
@keyframes blink{0%,50%{border-color:var(--accent)}51%,100%{border-color:transparent}}
.typewriter{border-right:2px solid var(--accent);animation:blink 1s infinite}
.container{width:100%;max-width:540px;margin:0 auto;padding:12px;min-height:100vh;min-height:100dvh}
.hidden{display:none!important}
.logo{font-size:clamp(18px,5vw,24px);font-weight:300;letter-spacing:clamp(4px,2vw,8px);text-align:center;padding:clamp(24px,8vw,40px) 0 clamp(16px,4vw,24px);color:var(--accent)}
.card{background:var(--card);border:1px solid var(--border);border-radius:12px;padding:clamp(12px,3vw,16px);margin:8px 0}
.input{width:100%;padding:10px 12px;background:transparent;border:1px solid var(--border);color:var(--text);border-radius:8px;font-size:14px;outline:none}
.input:focus{border-color:var(--accent)}
.btn{padding:8px 16px;background:transparent;border:1px solid var(--accent);color:var(--accent);border-radius:8px;font-size:13px;cursor:pointer}
.btn:hover{background:var(--accent);color:#000}
.btn.full{width:100%}
.btn.sm{padding:5px 10px;font-size:11px}
.btn.danger{border-color:var(--error);color:var(--error)}
.btn.danger:hover{background:var(--error);color:#fff}
.btn.warn{border-color:var(--warn);color:var(--warn)}
.btn.warn:hover{background:var(--warn);color:#000}
.err{color:var(--error);font-size:12px;margin:8px 0;text-align:center}
.success{color:var(--success);font-size:12px;margin:8px 0}
.status{position:fixed;top:8px;right:8px;padding:4px 10px;font-size:10px;border:1px solid var(--border);border-radius:12px;z-index:100}
.status.on{border-color:var(--accent);color:var(--accent)}
.channel-card{background:linear-gradient(135deg,rgba(0,255,255,.03),rgba(255,0,255,.03));border:1px solid var(--border);border-radius:12px;padding:14px;margin:8px 0;cursor:pointer}
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
.msg-avatar{width:36px;height:36px;border-radius:8px;background:linear-gradient(135deg,var(--accent),var(--accent2));display:flex;align-items:center;justify-content:center;font-size:14px;font-weight:600;color:#000;flex-shrink:0}
.msg-bubble{max-width:70%;padding:10px 12px;border-radius:12px;font-size:14px;line-height:1.5;word-break:break-word}
.msg-bubble.in{background:var(--card);border:1px solid var(--border);border-top-left-radius:4px}
.msg-bubble.out{background:var(--accent);color:#000;border-top-right-radius:4px}
.msg-nick{font-size:11px;color:var(--accent);margin-bottom:2px}
.msg-bubble.out .msg-nick{color:rgba(0,0,0,.5)}
.msg-time{font-size:10px;color:var(--muted);margin-top:4px;text-align:right}
.msg-bubble.out .msg-time{color:rgba(0,0,0,.4)}
.chat-input{display:flex;gap:8px;padding:12px;border-top:1px solid var(--border);background:var(--bg);flex-shrink:0}
.chat-input textarea{flex:1;padding:10px 12px;background:var(--card);border:1px solid var(--border);color:var(--text);border-radius:20px;font-size:14px;outline:none;resize:none;max-height:80px;font-family:inherit;line-height:1.4}
.chat-input textarea:focus{border-color:var(--accent)}
.send-btn{width:40px;height:40px;background:var(--accent);border:none;border-radius:50%;cursor:pointer;display:flex;align-items:center;justify-content:center;flex-shrink:0}
.send-btn svg{width:18px;height:18px;fill:#000}
.admin-tabs{display:flex;gap:4px;margin-bottom:12px}
.admin-tab{flex:1;padding:10px;background:transparent;border:1px solid var(--border);color:var(--muted);border-radius:8px;font-size:13px;cursor:pointer;text-align:center}
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
.item-card .item-info{font-size:11px;color:var(--muted);margin-bottom:8px}
.item-card .item-actions{display:flex;gap:4px;flex-wrap:wrap}
.item-card .item-actions button{flex:1;min-width:50px}
.empty{text-align:center;color:var(--muted);font-size:13px;padding:24px}
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
<h2>聊天</h2>
<div style="width:30px"></div>
</div>
<div class="chat-msgs" id="msgs"></div>
<div class="chat-input">
<textarea id="msgInput" rows="1" placeholder="消息..." onkeydown="handleKey(event)"></textarea>
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
<div class="card">
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
<div class="card">
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

<script>
let token='',user=null,ws=null,groupId=null,lastSend=0;
const API=location.origin;

function typeWriter(el,text,i){if(i<text.length){el.textContent=text.substring(0,i+1);setTimeout(()=>typeWriter(el,text,i+1),100)}}
function startLogo(){typeWriter(document.getElementById('logoText'),'ARCANUM',0);typeWriter(document.getElementById('logoText2'),'ARCANUM',0)}

async function api(path,opts={}){const r=await fetch(API+path,{...opts,headers:{'Authorization':'Bearer '+token,'Content-Type':'application/json',...opts.headers}});const d=await r.json();if(d.error&&(d.error.includes('封禁')||d.error.includes('踢出')||d.error.includes('未登录'))){localStorage.clear();location.reload()}return d}

async function login(){
const uid=document.getElementById('loginUid').value.trim().toUpperCase();
const pwd=document.getElementById('loginPwd').value;
if(!uid||!pwd)return;
try{
const r=await fetch(API+'/api/auth/login',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({uid,password:pwd})});
const d=await r.json();
if(d.success){token=d.data.token;user=d.data.user;localStorage.setItem('t',token);localStorage.setItem('u',JSON.stringify(user));showMain()}
else document.getElementById('loginErr').textContent=d.error;
}catch(e){document.getElementById('loginErr').textContent='网络错误'}
}

function showMain(){
document.getElementById('loginPage').classList.add('hidden');
document.getElementById('mainPage').classList.remove('hidden');
if(user.role==='admin')document.getElementById('adminEntry').classList.remove('hidden');
connect();loadMyChannels();
}

async function enterChannel(){
const name=document.getElementById('cipherInput').value.trim();
if(!name){document.getElementById('cipherErr').textContent='请输入频道名';return}
try{
const d=await api('/api/groups/enter',{method:'POST',body:JSON.stringify({name})});
if(d.success){groupId=d.data.id;document.getElementById('cipherErr').textContent='';document.getElementById('cipherInput').value='';showChat()}
else{document.getElementById('cipherErr').textContent=d.error||'频道不存在'}
}catch(e){document.getElementById('cipherErr').textContent='网络错误'}
}

async function loadMyChannels(){
try{
const d=await api('/api/groups');
const el=document.getElementById('myChannels');
if(d.success){el.innerHTML=d.data.length?d.data.map(g=>'<div class="channel-card" onclick="joinChannel(\''+g.id+'\')"><h3>'+esc(g.name)+'</h3><p>点击进入</p></div>').join(''):'<div class="empty">暂无频道，输入频道名进入</div>'}
}catch(e){}
}

function joinChannel(id){groupId=id;showChat()}
function showChat(){document.getElementById('channelView').classList.add('hidden');document.getElementById('chatView').classList.remove('hidden');document.getElementById('adminView').classList.add('hidden');loadMsgs()}
function leaveChat(){groupId=null;document.getElementById('channelView').classList.remove('hidden');document.getElementById('chatView').classList.add('hidden');loadMyChannels()}

async function loadMsgs(){
if(!groupId)return;
try{
const d=await api('/api/messages/group/'+groupId);
if(d.success){
const el=document.getElementById('msgs');
el.innerHTML=d.data.map(m=>{
const isMe=m.senderId===user.id;
return'<div class="msg-row'+(isMe?' me':'')+'"><div class="msg-avatar">'+m.senderNickname.charAt(0).toUpperCase()+'</div><div class="msg-bubble '+(isMe?'out':'in')+'"><div class="msg-nick">'+esc(m.senderNickname)+'</div>'+esc(m.content)+'<div class="msg-time">'+new Date(m.createdAt).toLocaleTimeString('zh-CN',{hour:'2-digit',minute:'2-digit'})+'</div></div></div>';
}).join('');
el.scrollTop=el.scrollHeight;
}
}catch(e){}
}

function send(){
const now=Date.now();if(now-lastSend<500)return;lastSend=now;
const input=document.getElementById('msgInput');const content=input.value.trim();
if(!content||!ws)return;if(content.length>5000){alert('消息太长');return}
ws.send(JSON.stringify({event:'message',data:{group_id:groupId,content}}));
input.value='';input.style.height='auto';
}

function handleKey(e){if(e.key==='Enter'&&!e.shiftKey){e.preventDefault();send()}}

function connect(){
const p=location.protocol==='https:'?'wss:':'ws:';
ws=new WebSocket(p+'//'+location.host+'/ws?token='+token);
ws.onopen=()=>{document.getElementById('status').textContent='在线';document.getElementById('status').classList.add('on')};
ws.onclose=()=>{document.getElementById('status').textContent='离线';document.getElementById('status').classList.remove('on')};
ws.onmessage=e=>{
const m=JSON.parse(e.data);
if(m.event==='message'&&m.data.groupId===groupId){
const isMe=m.data.senderId===user.id;
const el=document.getElementById('msgs');
el.innerHTML+='<div class="msg-row'+(isMe?' me':'')+'"><div class="msg-avatar">'+m.data.senderNickname.charAt(0).toUpperCase()+'</div><div class="msg-bubble '+(isMe?'out':'in')+'"><div class="msg-nick">'+esc(m.data.senderNickname)+'</div>'+esc(m.data.content)+'<div class="msg-time">'+new Date(m.data.createdAt).toLocaleTimeString('zh-CN',{hour:'2-digit',minute:'2-digit'})+'</div></div></div>';
el.scrollTop=el.scrollHeight;
}
};
}

function showAdmin(){document.getElementById('channelView').classList.add('hidden');document.getElementById('chatView').classList.add('hidden');document.getElementById('adminView').classList.remove('hidden');loadUsers();loadGroups();loadIps()}
function showChannel(){document.getElementById('channelView').classList.remove('hidden');document.getElementById('chatView').classList.add('hidden');document.getElementById('adminView').classList.add('hidden')}
function adminTab(name){document.querySelectorAll('.admin-tab').forEach(t=>t.classList.remove('on'));document.querySelectorAll('.admin-section').forEach(s=>s.classList.remove('on'));event.target.classList.add('on');document.getElementById(name+'Section').classList.add('on')}

async function createUser(){
const uid=document.getElementById('newUid').value.trim().toUpperCase();
const nick=document.getElementById('newNick').value.trim();
const pwd=document.getElementById('newPwd').value;
if(!nick||pwd.length<6){document.getElementById('userRes').innerHTML='<div class="err">请填写昵称和密码(6位+)</div>';return}
try{
const body={nickname:nick,password:pwd};if(uid)body.uid=uid;
const d=await api('/api/admin/users',{method:'POST',body:JSON.stringify(body)});
if(d.success){
document.getElementById('userRes').innerHTML='<div class="success">创建成功</div><div style="font-size:12px;margin-top:8px">UID: '+d.data.uid+'<br>昵称: '+d.data.nickname+'<br>密码: '+d.data.password+'</div>';
document.getElementById('newUid').value='';document.getElementById('newNick').value='';document.getElementById('newPwd').value='';loadUsers();
}else{document.getElementById('userRes').innerHTML='<div class="err">'+d.error+'</div>'}
}catch(e){document.getElementById('userRes').innerHTML='<div class="err">网络错误</div>'}
}

async function loadUsers(){
try{
const d=await api('/api/admin/users');
const el=document.getElementById('userList');
if(d.success){el.innerHTML=d.data.length?d.data.map(u=>'<div class="item-card"><div class="item-header"><span class="item-title">'+esc(u.nickname)+'</span><span class="item-badge '+(u.online?'online':'')+'">'+(u.online?'在线':'离线')+'</span></div><div class="item-info">UID: '+u.uid+(u.lastIp?' | IP: '+u.lastIp:'')+'</div>'+(u.role!=='admin'?'<div class="item-actions">'+(u.status==='banned'?'<button class="btn sm" onclick="unbanUser(\''+u.uid+'\')">解封</button>':'<button class="btn sm warn" onclick="banUser(\''+u.uid+'\')">封禁</button>')+'<button class="btn sm" onclick="kickUser(\''+u.uid+'\')">踢出</button><button class="btn sm danger" onclick="deleteUser(\''+u.uid+'\')">删除</button></div>':'')+'</div>').join(''):'<div class="empty">暂无用户</div>'}
}catch(e){}
}

async function banUser(uid){if(!confirm('确定封禁该用户?'))return;try{await api('/api/admin/users/'+uid+'/ban',{method:'PUT'});loadUsers()}catch(e){}}
async function unbanUser(uid){try{await api('/api/admin/users/'+uid+'/unban',{method:'PUT'});loadUsers()}catch(e){}}
async function kickUser(uid){try{await api('/api/admin/kick/'+uid,{method:'PUT'});alert('已踢出')}catch(e){}}
async function deleteUser(uid){if(!confirm('确定删除该用户?'))return;try{await api('/api/admin/users/'+uid,{method:'DELETE'});loadUsers()}catch(e){}}

async function createChannel(){
const name=document.getElementById('newChan').value.trim();if(!name)return;
try{
const d=await api('/api/groups',{method:'POST',body:JSON.stringify({name})});
if(d.success){document.getElementById('chanRes').innerHTML='<div class="success">创建成功: '+name+'</div>';document.getElementById('newChan').value='';loadGroups()}
else{document.getElementById('chanRes').innerHTML='<div class="err">'+d.error+'</div>'}
}catch(e){}
}

async function loadGroups(){
try{
const d=await api('/api/admin/groups');
const el=document.getElementById('groupList');
if(d.success){el.innerHTML=d.data.length?d.data.map(g=>'<div class="item-card"><div class="item-header"><span class="item-title">'+esc(g.name)+'</span><span class="item-badge">'+g.memberCount+'人</span></div><div class="item-info">暗号: '+esc(g.name)+'</div><div class="item-actions"><button class="btn sm" onclick="clearGroup(\''+g.id+'\')">清空</button><button class="btn sm danger" onclick="deleteGroup(\''+g.id+'\')">删除</button></div></div>').join(''):'<div class="empty">暂无频道</div>'}
}catch(e){}
}

async function clearGroup(id){if(!confirm('确定清空该频道所有消息?'))return;try{await api('/api/messages/group/'+id,{method:'DELETE'});alert('已清空')}catch(e){}}
async function deleteGroup(id){if(!confirm('确定删除该频道?'))return;try{await api('/api/admin/groups/'+id,{method:'DELETE'});loadGroups()}catch(e){}}

async function loadIps(){
try{
const d=await api('/api/admin/ips');
const el=document.getElementById('ipList');
if(d.success){el.innerHTML=d.data.length?d.data.map(ip=>'<div class="item-card"><div class="item-header"><span class="item-title">'+ip.ip+'</span><button class="btn sm" onclick="unbanIp(\''+ip.ip+'\')">解封</button></div><div class="item-info">'+(ip.reason||'')+' | '+ip.createdAt+'</div></div>').join(''):'<div class="empty">暂无封禁IP</div>'}
}catch(e){}
}

async function unbanIp(ip){try{await api('/api/admin/ips/'+ip,{method:'DELETE'});loadIps()}catch(e){}}

function esc(t){const d=document.createElement('div');d.textContent=t;return d.innerHTML}

window.onload=()=>{
startLogo();
const t=localStorage.getItem('t'),u=localStorage.getItem('u');
if(t&&u){token=t;user=JSON.parse(u);showMain()}
document.getElementById('loginPwd').onkeydown=e=>{if(e.key==='Enter')login()};
};

document.getElementById('msgInput').addEventListener('input',function(){this.style.height='auto';this.style.height=Math.min(this.scrollHeight,80)+'px'});
</script>
</body>
</html>
"#;
