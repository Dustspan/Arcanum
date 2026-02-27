pub const INDEX_HTML: &str = r##"<!DOCTYPE html>
<html lang="zh">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1.0,maximum-scale=1.0,user-scalable=no">
<title>ARCANUM</title>
<style>
:root{--bg:#0a0a0f;--bg2:#12121a;--card:#16161f;--text:#e0e0e8;--muted:#6a6a7a;--accent:#00f0ff;--border:#2a2a3a;--error:#ff3366;--success:#00ff88}
[data-theme="light"]{--bg:#f0f0f5;--bg2:#e8e8f0;--card:#fff;--text:#1a1a2e;--muted:#6a6a7a;--accent:#0088aa;--border:#d0d0da}
*{margin:0;padding:0;box-sizing:border-box}
body{background:var(--bg);color:var(--text);font-family:system-ui,sans-serif;min-height:100vh}
.container{max-width:540px;margin:0 auto;padding:12px;min-height:100vh}
.btn{padding:10px 20px;background:transparent;border:1px solid var(--accent);color:var(--accent);border-radius:4px;font-size:14px;cursor:pointer}
.btn:hover{background:var(--accent);color:#000}
.btn:disabled{opacity:.5;cursor:not-allowed}
.btn.full{width:100%}
.btn.sm{padding:6px 12px;font-size:12px}
.btn.danger{border-color:var(--error);color:var(--error)}
.input{width:100%;padding:12px;background:var(--bg2);border:1px solid var(--border);color:var(--text);border-radius:4px;font-size:14px;outline:none}
.input:focus{border-color:var(--accent)}
.card{background:var(--card);border:1px solid var(--border);border-radius:8px;padding:16px;margin-bottom:12px}
.err{color:var(--error);font-size:13px;padding:8px;background:rgba(255,51,102,.1);border-radius:4px;margin-top:8px}
.logo{font-size:28px;font-weight:700;text-align:center;margin:60px 0 30px;color:var(--accent)}
.header{display:flex;justify-content:space-between;align-items:center;padding:8px 0;margin-bottom:12px}
.header h1{font-size:16px;color:var(--accent)}
.header-info{font-size:11px;color:var(--muted)}
.header-actions{display:flex;gap:6px}
.tabs{display:flex;gap:4px;margin-bottom:12px;border-bottom:1px solid var(--border);padding-bottom:8px}
.tab{flex:1;text-align:center;padding:8px;color:var(--muted);cursor:pointer;font-size:13px;border-bottom:2px solid transparent}
.tab.active{color:var(--accent);border-bottom-color:var(--accent)}
.channel-input{display:flex;gap:8px;margin-bottom:16px}
.channel-card{background:var(--card);border:1px solid var(--border);border-radius:8px;padding:14px;cursor:pointer;margin-bottom:8px}
.channel-card:hover{border-color:var(--accent)}
.channel-card h3{font-size:14px;margin-bottom:4px}
.channel-card p{font-size:12px;color:var(--muted)}
.chat-wrap{display:flex;flex-direction:column;height:calc(100vh - 100px)}
.chat-header{display:flex;justify-content:space-between;align-items:center;padding:12px;background:var(--card);border-bottom:1px solid var(--border)}
.chat-header h3{font-size:14px}
.chat-msgs{flex:1;overflow-y:auto;padding:12px;display:flex;flex-direction:column;gap:10px;background:var(--bg2)}
.msg-row{display:flex;gap:8px}
.msg-row.me{flex-direction:row-reverse}
.msg-avatar{width:32px;height:32px;border-radius:6px;background:linear-gradient(135deg,var(--accent),#ff00aa);display:flex;align-items:center;justify-content:center;font-size:12px;font-weight:600;color:#000;flex-shrink:0;cursor:pointer;overflow:hidden}
.msg-avatar img{width:100%;height:100%;object-fit:cover}
.msg-bubble{max-width:70%;padding:8px 12px;border-radius:10px;font-size:13px}
.msg-bubble.in{background:var(--card);border:1px solid var(--border)}
.msg-bubble.out{background:var(--accent);color:#000}
.msg-nick{font-size:10px;color:var(--accent);margin-bottom:2px}
.msg-time{font-size:9px;color:var(--muted);margin-top:4px;text-align:right}
.msg-img{max-width:180px;border-radius:6px;cursor:pointer}
.chat-input{display:flex;gap:8px;padding:12px;background:var(--bg);border-top:1px solid var(--border)}
.chat-input textarea{flex:1;padding:8px 12px;background:var(--card);border:1px solid var(--border);color:var(--text);border-radius:16px;font-size:13px;outline:none;resize:none;max-height:60px}
.chat-input input[type="file"]{display:none}
.upload-btn{background:var(--bg2);border:1px solid var(--border);border-radius:50%;width:32px;height:32px;display:flex;align-items:center;justify-content:center;cursor:pointer;color:var(--muted);font-size:16px}
.upload-btn:hover{border-color:var(--accent);color:var(--accent)}
.admin-tabs{display:flex;gap:4px;margin-bottom:12px;flex-wrap:wrap}
.admin-tab{flex:1;min-width:50px;padding:8px;background:transparent;border:1px solid var(--border);color:var(--muted);border-radius:4px;font-size:11px;cursor:pointer}
.admin-tab.active{border-color:var(--accent);color:var(--accent)}
.admin-section{display:none}
.admin-section.active{display:block}
.item-card{background:var(--bg2);border:1px solid var(--border);border-radius:6px;padding:10px;margin-bottom:6px}
.item-header{display:flex;justify-content:space-between;align-items:center}
.item-title{font-size:13px}
.item-info{font-size:10px;color:var(--muted);margin-top:4px}
.badge{display:inline-block;padding:2px 6px;border-radius:8px;font-size:9px;margin-left:4px}
.badge.success{background:rgba(0,255,136,.2);color:var(--success)}
.badge.error{background:rgba(255,51,102,.2);color:var(--error)}
.perm-list{display:flex;flex-wrap:wrap;gap:4px;margin-top:6px}
.perm-tag{font-size:10px;padding:2px 6px;background:var(--bg);border:1px solid var(--border);border-radius:4px}
.modal-mask{position:fixed;inset:0;background:rgba(0,0,0,.8);display:flex;align-items:center;justify-content:center;z-index:1000;padding:12px}
.modal{background:var(--card);border:1px solid var(--border);border-radius:8px;max-width:400px;width:100%;max-height:90vh;overflow-y:auto}
.modal-header{display:flex;justify-content:space-between;align-items:center;padding:12px;border-bottom:1px solid var(--border)}
.modal-header h3{font-size:14px}
.modal-close{background:none;border:none;color:var(--muted);font-size:20px;cursor:pointer}
.modal-body{padding:12px}
.user-menu{position:fixed;background:var(--card);border:1px solid var(--border);border-radius:6px;padding:6px;z-index:1001;min-width:150px;box-shadow:0 4px 20px rgba(0,0,0,.5)}
.user-menu-item{display:block;width:100%;padding:6px 10px;background:none;border:none;color:var(--text);text-align:left;cursor:pointer;border-radius:4px;font-size:12px}
.user-menu-item:hover{background:var(--bg2)}
.user-menu-item.danger{color:var(--error)}
.friend-item{display:flex;align-items:center;gap:8px;padding:10px;background:var(--bg2);border-radius:6px;margin-bottom:6px;cursor:pointer}
.friend-item:hover{background:var(--card)}
.friend-avatar{width:36px;height:36px;border-radius:6px;background:linear-gradient(135deg,var(--accent),#ff00aa);display:flex;align-items:center;justify-content:center;font-size:14px;color:#000;overflow:hidden}
.friend-avatar img{width:100%;height:100%;object-fit:cover}
.friend-info{flex:1}
.friend-name{font-size:13px}
.friend-status{font-size:10px;color:var(--muted)}
.friend-status.online{color:var(--success)}
.dm-header{display:flex;align-items:center;gap:8px;padding:12px;background:var(--card);border-bottom:1px solid var(--border)}
.dm-back{background:none;border:none;color:var(--accent);font-size:20px;cursor:pointer}
.dm-title{font-size:14px}
.image-preview{position:fixed;inset:0;background:rgba(0,0,0,.95);display:flex;align-items:center;justify-content:center;z-index:2000;cursor:zoom-out}
.image-preview img{max-width:90%;max-height:90%}
</style>
</head>
<body>
<div id="app"></div>
<script src="https://unpkg.com/vue@3/dist/vue.global.prod.js"></script>
<script>
Vue.createApp({
  data() {
    return {
      loggedIn: false,
      user: {},
      token: '',
      theme: 'dark',
      isAdmin: false,
      userPerms: [],
      mainTab: 'channels',
      groups: [],
      currentGroup: null,
      channelInput: '',
      channelLoading: false,
      messages: [],
      msgInput: '',
      msgsBox: null,
      dmTarget: null,
      dmMessages: [],
      dmInput: '',
      friends: [],
      friendRequests: [],
      friendRequestCount: 0,
      showAdmin: false,
      adminTab: 'users',
      users: [],
      allGroups: [],
      words: [],
      stats: {},
      newUser: { uid: '', nickname: '', password: '' },
      createUserLoading: false,
      newGroup: { name: '' },
      createGroupLoading: false,
      newWord: { word: '', replacement: '***' },
      showPermModal: false,
      permTarget: null,
      permTargetPerms: [],
      allPermissions: [],
      userMenu: { show: false, x: 0, y: 0, uid: '', userId: '', nickname: '', online: false },
      loginForm: { uid: '', pwd: '' },
      loginError: '',
      loginLoading: false,
      previewImageUrl: null,
      ws: null
    };
  },
  computed: {
    canAccessAdmin() { return this.isAdmin || this.userPerms.length > 0; },
    canManageUser() { return this.hasPerm('user_ban') || this.hasPerm('user_mute') || this.isAdmin; }
  },
  methods: {
    hasPerm(name) {
      if (this.isAdmin) return true;
      return this.userPerms.includes(name);
    },
    hasUserPerm(name) {
      return this.permTargetPerms.includes(name);
    },
    async api(path, options = {}) {
      const headers = { 'Content-Type': 'application/json', ...options.headers };
      if (this.token) headers['Authorization'] = 'Bearer ' + this.token;
      try {
        const r = await fetch(location.origin + path, { ...options, headers });
        return await r.json();
      } catch (e) {
        return { success: false, error: e.message };
      }
    },
    async doLogin() {
      if (!this.loginForm.uid || !this.loginForm.pwd) {
        this.loginError = '请输入UID和密码';
        return;
      }
      this.loginLoading = true;
      this.loginError = '';
      try {
        const r = await fetch(location.origin + '/api/auth/login', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ uid: this.loginForm.uid.toUpperCase(), password: this.loginForm.pwd })
        });
        const d = await r.json();
        if (d.success) {
          this.token = d.data.token;
          this.user = d.data.user;
          this.isAdmin = d.data.user.role === 'admin';
          this.userPerms = d.data.user.permissions || [];
          localStorage.setItem('t', this.token);
          localStorage.setItem('u', JSON.stringify(this.user));
          this.loggedIn = true;
          this.connectWS();
          this.loadGroups();
          this.loadFriendRequests();
        } else {
          this.loginError = d.error || '登录失败';
        }
      } catch (e) {
        this.loginError = '网络错误';
      }
      this.loginLoading = false;
    },
    doLogout() {
      this.api('/api/auth/logout', { method: 'POST' });
      localStorage.clear();
      this.token = '';
      this.user = {};
      this.loggedIn = false;
      this.isAdmin = false;
      this.userPerms = [];
      if (this.ws) this.ws.close();
    },
    async loadGroups() {
      const d = await this.api('/api/groups');
      if (d.success) this.groups = d.data;
    },
    async doEnterChannel() {
      if (!this.channelInput.trim()) return;
      this.channelLoading = true;
      const d = await this.api('/api/groups/enter', {
        method: 'POST',
        body: JSON.stringify({ name: this.channelInput.trim() })
      });
      this.channelLoading = false;
      if (d.success) {
        this.channelInput = '';
        this.loadGroups();
      } else {
        alert(d.error || '进入失败');
      }
    },
    async doJoinGroup(id) {
      const d = await this.api('/api/groups/' + id);
      if (d.success) {
        this.currentGroup = d.data;
        this.loadMessages();
      } else {
        alert(d.error || '加入失败');
      }
    },
    doLeaveGroup() {
      this.currentGroup = null;
      this.messages = [];
    },
    async loadMessages() {
      if (!this.currentGroup) return;
      const d = await this.api('/api/messages/group/' + this.currentGroup.id);
      if (d.success) {
        this.messages = d.data;
        this.$nextTick(() => {
          const box = this.$refs.msgsBox;
          if (box) box.scrollTop = box.scrollHeight;
        });
      }
    },
    async doSendMsg() {
      if (!this.msgInput.trim() || !this.currentGroup) return;
      const content = this.msgInput;
      this.msgInput = '';
      const d = await this.api('/api/messages', {
        method: 'POST',
        body: JSON.stringify({ content, groupId: this.currentGroup.id })
      });
      if (!d.success) {
        this.msgInput = content;
        alert(d.error || '发送失败');
      }
    },
    async uploadFile(e) {
      const file = e.target.files[0];
      if (!file || !this.currentGroup) return;
      const formData = new FormData();
      formData.append('file', file);
      try {
        const r = await fetch(location.origin + '/api/messages/file/' + this.currentGroup.id, {
          method: 'POST',
          headers: { 'Authorization': 'Bearer ' + this.token },
          body: formData
        });
        const d = await r.json();
        if (!d.success) alert(d.error || '上传失败');
      } catch (e) {
        alert('上传失败');
      }
      e.target.value = '';
    },
    async recallMessage(id) {
      if (!confirm('确定撤回该消息?')) return;
      const d = await this.api('/api/messages/' + id + '/recall', { method: 'POST' });
      if (d.success) {
        const idx = this.messages.findIndex(m => m.id === id);
        if (idx >= 0) this.messages.splice(idx, 1);
      } else {
        alert(d.error || '撤回失败');
      }
    },
    renderMsg(m) {
      if (m.msgType === 'image') return '<img class="msg-img" src="' + m.content + '" @click="previewImage(\'' + m.content + '\')">';
      if (m.msgType === 'file') return '<div>📄 ' + (m.fileName || '文件') + '</div>';
      return m.content || '';
    },
    formatTime(t) {
      return new Date(t).toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
    },
    previewImage(url) {
      this.previewImageUrl = url;
    },
    connectWS() {
      const proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
      this.ws = new WebSocket(proto + '//' + location.host + '/ws?token=' + this.token);
      this.ws.onmessage = (e) => {
        const m = JSON.parse(e.data);
        if (m.event === 'message' && m.data.groupId === this.currentGroup?.id) {
          this.messages.push(m.data);
          this.$nextTick(() => {
            const box = this.$refs.msgsBox;
            if (box) box.scrollTop = box.scrollHeight;
          });
        }
        if (m.event === 'direct_message') {
          if (this.dmTarget && (m.data.senderId === this.dmTarget.id || m.data.receiverId === this.dmTarget.id)) {
            this.dmMessages.push(m.data);
          }
        }
        if (m.event === 'message_recall' && m.data.groupId === this.currentGroup?.id) {
          const idx = this.messages.findIndex(msg => msg.id === m.data.id);
          if (idx >= 0) this.messages.splice(idx, 1);
        }
        if (m.event === 'friend_request') {
          this.loadFriendRequests();
        }
      };
      this.ws.onclose = () => setTimeout(() => this.connectWS(), 3000);
    },
    toggleTheme() {
      this.theme = this.theme === 'dark' ? 'light' : 'dark';
      document.documentElement.setAttribute('data-theme', this.theme === 'light' ? 'light' : '');
      localStorage.setItem('theme', this.theme);
    },
    openUserMenu(e, userId, nickname) {
      e.stopPropagation();
      this.api('/api/users/' + userId).then(d => {
        if (d.success) {
          this.userMenu.online = d.data.online;
        }
      });
      this.userMenu.show = true;
      this.userMenu.x = Math.min(e.clientX, window.innerWidth - 160);
      this.userMenu.y = Math.min(e.clientY, window.innerHeight - 150);
      this.userMenu.userId = userId;
      this.userMenu.nickname = nickname;
      this.userMenu.uid = userId;
    },
    closeUserMenu() {
      this.userMenu.show = false;
    },
    startDMFromMenu() {
      this.dmTarget = { id: this.userMenu.userId, nickname: this.userMenu.nickname, online: this.userMenu.online };
      this.dmMessages = [];
      this.userMenu.show = false;
    },
    closeDM() {
      this.dmTarget = null;
      this.dmMessages = [];
    },
    async sendDM() {
      if (!this.dmInput.trim() || !this.dmTarget) return;
      const content = this.dmInput;
      this.dmInput = '';
      const d = await this.api('/api/direct/' + this.dmTarget.id, {
        method: 'POST',
        body: JSON.stringify({ content })
      });
      if (!d.success) {
        this.dmInput = content;
        alert(d.error || '发送失败');
      }
    },
    async loadFriends() {
      const d = await this.api('/api/friends');
      if (d.success) this.friends = d.data;
    },
    async loadFriendRequests() {
      const d = await this.api('/api/friends/requests');
      if (d.success) {
        this.friendRequests = d.data;
        this.friendRequestCount = d.data.length;
      }
    },
    async acceptFriend(userId) {
      const d = await this.api('/api/friends/' + userId + '/accept', { method: 'POST' });
      if (d.success) {
        this.loadFriendRequests();
        this.loadFriends();
      } else {
        alert(d.error || '失败');
      }
    },
    doAddFriend() {
      this.api('/api/friends/' + this.userMenu.userId, { method: 'POST' }).then(d => {
        alert(d.success ? '好友请求已发送' : (d.error || '失败'));
      });
      this.userMenu.show = false;
    },
    startDM(friend) {
      this.dmTarget = { id: friend.id, nickname: friend.nickname, online: friend.online };
      this.dmMessages = [];
    },
    openAdmin() {
      this.showAdmin = true;
      this.loadAllPermissions();
      if (this.adminTab === 'users') this.loadUsers();
    },
    async loadAllPermissions() {
      const d = await this.api('/api/admin/permissions');
      if (d.success) this.allPermissions = d.data;
    },
    async loadUsers() {
      const d = await this.api('/api/admin/users');
      if (d.success) this.users = d.data;
    },
    async doCreateUser() {
      if (!this.newUser.nickname || !this.newUser.password) {
        alert('请填写昵称和密码');
        return;
      }
      this.createUserLoading = true;
      const d = await this.api('/api/admin/users', { method: 'POST', body: JSON.stringify(this.newUser) });
      this.createUserLoading = false;
      if (d.success) {
        this.newUser = { uid: '', nickname: '', password: '' };
        this.loadUsers();
        alert('创建成功');
      } else {
        alert(d.error || '创建失败');
      }
    },
    async doBanUser(uid) {
      if (!confirm('确定封禁该用户?')) return;
      const d = await this.api('/api/admin/users/' + uid + '/ban', { method: 'PUT' });
      alert(d.success ? '已封禁' : (d.error || '失败'));
      this.loadUsers();
      this.userMenu.show = false;
    },
    async doUnbanUser(uid) {
      const d = await this.api('/api/admin/users/' + uid + '/unban', { method: 'PUT' });
      alert(d.success ? '已解封' : (d.error || '失败'));
      this.loadUsers();
    },
    async doMuteUser(uid) {
      const d = await this.api('/api/admin/users/' + uid + '/mute', {
        method: 'PUT',
        body: JSON.stringify({ duration_minutes: 30 })
      });
      alert(d.success ? '已禁言30分钟' : (d.error || '失败'));
      this.loadUsers();
      this.userMenu.show = false;
    },
    async doKickUser(uid) {
      if (!confirm('确定踢出该用户?')) return;
      const d = await this.api('/api/admin/users/' + uid + '/kick', { method: 'PUT' });
      alert(d.success ? '已踢出' : (d.error || '失败'));
      this.loadUsers();
      this.userMenu.show = false;
    },
    async loadAllGroups() {
      const d = await this.api('/api/admin/groups');
      if (d.success) this.allGroups = d.data;
    },
    async doCreateGroup() {
      if (!this.newGroup.name.trim()) {
        alert('请输入频道名称');
        return;
      }
      this.createGroupLoading = true;
      const d = await this.api('/api/groups', { method: 'POST', body: JSON.stringify({ name: this.newGroup.name.trim() }) });
      this.createGroupLoading = false;
      if (d.success) {
        alert('频道创建成功');
        this.newGroup.name = '';
        this.loadAllGroups();
        this.loadGroups();
      } else {
        alert(d.error || '创建失败');
      }
    },
    async doDeleteGroup(id) {
      if (!confirm('确定删除该频道?')) return;
      const d = await this.api('/api/admin/groups/' + id, { method: 'DELETE' });
      if (d.success) this.loadAllGroups();
    },
    async doClearGroupMessages(id) {
      if (!confirm('确定清空该频道所有消息?')) return;
      const d = await this.api('/api/messages/group/' + id, { method: 'DELETE' });
      alert(d.success ? '消息已清空' : (d.error || '失败'));
    },
    async loadWords() {
      const d = await this.api('/api/admin/sensitive-words');
      if (d.success) this.words = d.data;
    },
    async doAddWord() {
      if (!this.newWord.word) { alert('请输入敏感词'); return; }
      const d = await this.api('/api/admin/sensitive-words', { method: 'POST', body: JSON.stringify(this.newWord) });
      if (d.success) {
        this.newWord = { word: '', replacement: '***' };
        this.loadWords();
      } else {
        alert(d.error || '失败');
      }
    },
    async doDeleteWord(id) {
      const d = await this.api('/api/admin/sensitive-words/' + id, { method: 'DELETE' });
      if (d.success) this.loadWords();
    },
    async loadStats() {
      const d = await this.api('/api/admin/statistics');
      if (d.success) this.stats = d.data;
    },
    openPermModal(u) {
      this.permTarget = u;
      this.permTargetPerms = [...(u.permissions || [])];
      this.showPermModal = true;
    },
    togglePerm(name) {
      const idx = this.permTargetPerms.indexOf(name);
      if (idx >= 0) this.permTargetPerms.splice(idx, 1);
      else this.permTargetPerms.push(name);
    },
    async savePerms() {
      if (!this.permTarget) return;
      const currentPerms = this.permTarget.permissions || [];
      const toGrant = this.permTargetPerms.filter(p => !currentPerms.includes(p));
      const toRevoke = currentPerms.filter(p => !this.permTargetPerms.includes(p));
      for (const p of toGrant) {
        await this.api('/api/admin/users/' + this.permTarget.uid + '/permissions', {
          method: 'POST',
          body: JSON.stringify({ permission_name: p })
        });
      }
      for (const p of toRevoke) {
        await this.api('/api/admin/users/' + this.permTarget.uid + '/permissions', {
          method: 'DELETE',
          body: JSON.stringify({ permission_name: p })
        });
      }
      this.showPermModal = false;
      this.loadUsers();
      alert('权限已更新');
    }
  },
  mounted() {
    document.addEventListener('click', () => this.closeUserMenu());
    const savedTheme = localStorage.getItem('theme');
    if (savedTheme === 'light') {
      this.theme = 'light';
      document.documentElement.setAttribute('data-theme', 'light');
    }
    const t = localStorage.getItem('t');
    const u = localStorage.getItem('u');
    if (t && u) {
      this.token = t;
      try {
        this.user = JSON.parse(u);
        this.api('/api/auth/me').then(me => {
          if (me.success) {
            this.user = me.data;
            this.isAdmin = me.data.role === 'admin';
            this.userPerms = me.data.permissions || [];
            this.loggedIn = true;
            this.connectWS();
            this.loadGroups();
            this.loadFriendRequests();
          } else {
            localStorage.clear();
            this.token = '';
            this.user = {};
          }
        });
      } catch (e) {
        localStorage.clear();
      }
    }
  },
  template: `
<div class="container" @click="closeUserMenu">
  <!-- 登录页 -->
  <div v-if="!loggedIn">
    <div class="logo">ARCANUM</div>
    <div class="card">
      <form @submit.prevent="doLogin">
        <input class="input" v-model="loginForm.uid" placeholder="UID" style="margin-bottom:8px">
        <input class="input" v-model="loginForm.pwd" type="password" placeholder="密码" style="margin-bottom:8px">
        <div class="err" v-if="loginError">{{loginError}}</div>
        <button class="btn full" type="submit" :disabled="loginLoading">{{loginLoading ? '登录中...' : '进入'}}</button>
      </form>
    </div>
  </div>

  <!-- 主页 -->
  <div v-else>
    <div class="header">
      <div>
        <h1>{{user.nickname}}</h1>
        <div class="header-info">{{user.uid}} <span v-if="isAdmin" class="badge error">管理员</span></div>
      </div>
      <div class="header-actions">
        <button class="btn sm" @click="toggleTheme">{{theme === 'dark' ? '☀' : '🌙'}}</button>
        <button class="btn sm" v-if="canAccessAdmin" @click="openAdmin">⚙</button>
        <button class="btn sm danger" @click="doLogout">退出</button>
      </div>
    </div>

    <!-- 标签栏 -->
    <div class="tabs" v-if="!currentGroup && !dmTarget">
      <div class="tab" :class="{active: mainTab === 'channels'}" @click="mainTab = 'channels'">频道</div>
      <div class="tab" :class="{active: mainTab === 'friends'}" @click="mainTab = 'friends'; loadFriends()">好友<span v-if="friendRequestCount > 0" style="background:var(--error);color:#fff;font-size:10px;padding:2px 6px;border-radius:10px;margin-left:4px">{{friendRequestCount}}</span></div>
    </div>

    <!-- 私聊界面 -->
    <div v-if="dmTarget" class="card chat-wrap">
      <div class="dm-header">
        <button class="dm-back" @click="closeDM">←</button>
        <div class="msg-avatar">{{dmTarget.nickname ? dmTarget.nickname.charAt(0) : '?'}}</div>
        <div>
          <div class="dm-title">{{dmTarget.nickname}}</div>
          <div style="font-size:10px" :style="{color: dmTarget.online ? 'var(--success)' : 'var(--muted)'}">{{dmTarget.online ? '在线' : '离线'}}</div>
        </div>
      </div>
      <div class="chat-msgs" ref="dmMsgsBox">
        <div class="msg-row" v-for="m in dmMessages" :key="m.id" :class="{me: m.senderId === user.id}">
          <div class="msg-avatar">{{m.senderNickname ? m.senderNickname.charAt(0) : '?'}}</div>
          <div class="msg-bubble" :class="m.senderId === user.id ? 'out' : 'in'">
            <div class="msg-nick" v-if="m.senderId !== user.id">{{m.senderNickname}}</div>
            <div>{{m.content}}</div>
            <div class="msg-time">{{formatTime(m.createdAt)}}</div>
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
        <button class="btn" @click="doEnterChannel" :disabled="channelLoading">{{channelLoading ? '...' : '进入'}}</button>
      </div>
      <div class="channel-card" v-for="g in groups" :key="g.id" @click="doJoinGroup(g.id)">
        <h3>{{g.name}}</h3>
        <p>成员: {{g.memberCount}}</p>
      </div>
      <div class="card" v-if="groups.length === 0" style="text-align:center;color:var(--muted);font-size:13px">
        暂无频道<br><small>请联系管理员创建频道</small>
      </div>
    </div>

    <!-- 好友列表 -->
    <div v-else-if="!currentGroup && mainTab === 'friends'">
      <div class="card">
        <div style="font-size:13px;margin-bottom:8px">好友请求</div>
        <div v-for="r in friendRequests" :key="r.requestId" class="friend-item">
          <div class="friend-avatar">{{r.nickname ? r.nickname.charAt(0) : '?'}}</div>
          <div class="friend-info"><div class="friend-name">{{r.nickname}}</div></div>
          <button class="btn sm" @click="acceptFriend(r.userId)">接受</button>
        </div>
        <div v-if="friendRequests.length === 0" style="color:var(--muted);font-size:12px">暂无好友请求</div>
      </div>
      <div class="card">
        <div style="font-size:13px;margin-bottom:8px">好友列表</div>
        <div v-for="f in friends" :key="f.id" class="friend-item" @click="startDM(f)">
          <div class="friend-avatar"><img v-if="f.avatar" :src="f.avatar"><span v-else>{{f.nickname ? f.nickname.charAt(0) : '?'}}</span></div>
          <div class="friend-info">
            <div class="friend-name">{{f.nickname}}</div>
            <div class="friend-status" :class="{online: f.online}">{{f.online ? '在线' : '离线'}}</div>
          </div>
        </div>
        <div v-if="friends.length === 0" style="color:var(--muted);font-size:12px">暂无好友</div>
      </div>
    </div>

    <!-- 聊天界面 -->
    <div v-else-if="currentGroup" class="card chat-wrap">
      <div class="chat-header">
        <div>
          <h3>{{currentGroup.name}}</h3>
          <div style="font-size:11px;color:var(--muted)">成员: {{currentGroup.memberCount}}</div>
        </div>
        <button class="btn sm" @click="doLeaveGroup">← 返回</button>
      </div>
      <div class="chat-msgs" ref="msgsBox">
        <div class="msg-row" v-for="m in messages" :key="m.id" :class="{me: m.senderId === user.id}">
          <div class="msg-avatar" @click.stop="openUserMenu($event, m.senderId, m.senderNickname)">{{m.senderNickname ? m.senderNickname.charAt(0) : '?'}}</div>
          <div class="msg-bubble" :class="m.senderId === user.id ? 'out' : 'in'">
            <div class="msg-nick" v-if="m.senderId !== user.id">{{m.senderNickname}}</div>
            <div v-if="m.msgType === 'image'"><img class="msg-img" :src="m.content" @click="previewImage(m.content)"></div>
            <div v-else v-html="renderMsg(m)"></div>
            <div class="msg-time">{{formatTime(m.createdAt)}}</div>
          </div>
          <button v-if="m.senderId === user.id" style="background:none;border:none;color:var(--muted);font-size:10px;cursor:pointer" @click="recallMessage(m.id)">撤回</button>
        </div>
      </div>
      <div class="chat-input">
        <label class="upload-btn" v-if="hasPerm('file_upload')">📷<input type="file" accept="image/*,.txt" @change="uploadFile"></label>
        <textarea v-model="msgInput" placeholder="消息..." @keyup.enter="doSendMsg" rows="1"></textarea>
        <button class="btn" @click="doSendMsg">→</button>
      </div>
    </div>
  </div>

  <!-- 管理面板 -->
  <div class="modal-mask" v-if="showAdmin" @click.self="showAdmin = false">
    <div class="modal" style="max-width:500px">
      <div class="modal-header"><h3>管理面板</h3><button class="modal-close" @click="showAdmin = false">×</button></div>
      <div class="modal-body">
        <div class="admin-tabs">
          <button class="admin-tab" :class="{active: adminTab === 'users'}" @click="adminTab = 'users'; loadUsers()">用户</button>
          <button class="admin-tab" :class="{active: adminTab === 'groups'}" @click="adminTab = 'groups'; loadAllGroups()">频道</button>
          <button class="admin-tab" :class="{active: adminTab === 'words'}" @click="adminTab = 'words'; loadWords()">敏感词</button>
          <button class="admin-tab" :class="{active: adminTab === 'stats'}" @click="adminTab = 'stats'; loadStats()">统计</button>
        </div>
        <div class="admin-section" :class="{active: adminTab === 'users'}">
          <div class="card" v-if="hasPerm('user_create')">
            <input class="input" v-model="newUser.uid" placeholder="UID (留空自动生成)" style="margin-bottom:8px">
            <input class="input" v-model="newUser.nickname" placeholder="昵称" style="margin-bottom:8px">
            <input class="input" v-model="newUser.password" type="password" placeholder="密码" style="margin-bottom:8px">
            <button class="btn full" @click="doCreateUser" :disabled="createUserLoading">{{createUserLoading ? '创建中...' : '创建用户'}}</button>
          </div>
          <div class="item-card" v-for="u in users" :key="u.id">
            <div class="item-header"><span class="item-title">{{u.nickname}} <span class="badge" :class="u.status === 'banned' ? 'error' : (u.online ? 'success' : '')">{{u.status === 'banned' ? '已封禁' : (u.online ? '在线' : '离线')}}</span></span></div>
            <div class="item-info">{{u.uid}}</div>
            <div style="display:flex;gap:4px;margin-top:6px;flex-wrap:wrap">
              <button class="btn sm" v-if="hasPerm('user_ban') && u.status !== 'banned' && u.role !== 'admin'" @click="doBanUser(u.uid)">封禁</button>
              <button class="btn sm" v-if="hasPerm('user_ban') && u.status === 'banned'" @click="doUnbanUser(u.uid)">解封</button>
              <button class="btn sm" v-if="hasPerm('user_mute')" @click="doMuteUser(u.uid)">禁言</button>
              <button class="btn sm" v-if="hasPerm('permission_grant') && u.role !== 'admin'" @click="openPermModal(u)">权限</button>
            </div>
          </div>
        </div>
        <div class="admin-section" :class="{active: adminTab === 'groups'}">
          <div class="card" v-if="hasPerm('group_create')">
            <input class="input" v-model="newGroup.name" placeholder="频道名称" style="margin-bottom:8px">
            <button class="btn full" @click="doCreateGroup" :disabled="createGroupLoading">{{createGroupLoading ? '创建中...' : '创建频道'}}</button>
          </div>
          <div class="item-card" v-for="g in allGroups" :key="g.id">
            <div class="item-header"><span class="item-title">{{g.name}}</span><span class="badge success">{{g.memberCount}}人</span></div>
            <div class="item-info">{{g.id}}</div>
            <div style="margin-top:6px;display:flex;gap:4px">
              <button class="btn sm danger" v-if="hasPerm('group_delete')" @click="doDeleteGroup(g.id)">删除</button>
              <button class="btn sm" v-if="hasPerm('message_delete')" @click="doClearGroupMessages(g.id)">清空消息</button>
            </div>
          </div>
        </div>
        <div class="admin-section" :class="{active: adminTab === 'words'}">
          <div class="card" v-if="isAdmin">
            <input class="input" v-model="newWord.word" placeholder="敏感词" style="margin-bottom:8px">
            <input class="input" v-model="newWord.replacement" placeholder="替换为" style="margin-bottom:8px">
            <button class="btn full" @click="doAddWord">添加</button>
          </div>
          <div class="item-card" v-for="w in words" :key="w.id">
            <div class="item-header"><span class="item-title">{{w.word}}</span><button class="btn sm danger" v-if="isAdmin" @click="doDeleteWord(w.id)">删除</button></div>
            <div class="item-info">替换为: {{w.replacement}}</div>
          </div>
        </div>
        <div class="admin-section" :class="{active: adminTab === 'stats'}">
          <div style="display:grid;grid-template-columns:repeat(2,1fr);gap:10px">
            <div class="item-card" style="text-align:center"><div style="font-size:20px;color:var(--accent)">{{stats.users?.total || 0}}</div><div style="font-size:10px;color:var(--muted)">用户总数</div></div>
            <div class="item-card" style="text-align:center"><div style="font-size:20px;color:var(--accent)">{{stats.users?.online || 0}}</div><div style="font-size:10px;color:var(--muted)">在线用户</div></div>
            <div class="item-card" style="text-align:center"><div style="font-size:20px;color:var(--accent)">{{stats.groups?.total || 0}}</div><div style="font-size:10px;color:var(--muted)">频道总数</div></div>
            <div class="item-card" style="text-align:center"><div style="font-size:20px;color:var(--accent)">{{stats.messages?.total || 0}}</div><div style="font-size:10px;color:var(--muted)">消息总数</div></div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- 权限管理 -->
  <div class="modal-mask" v-if="showPermModal" @click.self="showPermModal = false">
    <div class="modal">
      <div class="modal-header"><h3>权限管理 - {{permTarget?.nickname}}</h3><button class="modal-close" @click="showPermModal = false">×</button></div>
      <div class="modal-body">
        <div v-for="p in allPermissions" :key="p.name" style="margin-bottom:6px">
          <label style="display:flex;align-items:center;gap:8px;cursor:pointer">
            <input type="checkbox" :checked="hasUserPerm(p.name)" @change="togglePerm(p.name)">
            <span style="font-size:12px">{{p.name}}</span>
          </label>
        </div>
        <button class="btn full" @click="savePerms" style="margin-top:12px">保存</button>
      </div>
    </div>
  </div>

  <!-- 用户菜单 -->
  <div class="user-menu" v-if="userMenu.show" :style="{left: userMenu.x + 'px', top: userMenu.y + 'px'}" @click.stop>
    <div style="padding:6px;border-bottom:1px solid var(--border);margin-bottom:6px;font-weight:500">{{userMenu.nickname}}</div>
    <button class="user-menu-item" @click="startDMFromMenu">私聊</button>
    <button class="user-menu-item" @click="doAddFriend">添加好友</button>
    <template v-if="canManageUser && userMenu.userId !== user.id">
      <button class="user-menu-item" @click="doMuteUser(userMenu.uid)">禁言</button>
      <button class="user-menu-item danger" @click="doBanUser(userMenu.uid)">封禁</button>
      <button class="user-menu-item danger" @click="doKickUser(userMenu.uid)">踢出</button>
    </template>
  </div>

  <!-- 图片预览 -->
  <div class="image-preview" v-if="previewImageUrl" @click="previewImageUrl = null">
    <img :src="previewImageUrl">
  </div>
</div>
`
}).mount('#app');
</script>
</body>
</html>
"##;

pub const MANIFEST_JSON: &str = r##"{"name":"ARCANUM","short_name":"ARCANUM","start_url":"/","display":"standalone","background_color":"#000000","theme_color":"#000000"}"##;

pub const SERVICE_WORKER_JS: &str = r##"const CACHE_NAME='arcanum-v1';self.addEventListener('install',e=>e.waitUntil(caches.open(CACHE_NAME).then(c=>c.addAll(['/']))));self.addEventListener('fetch',e=>e.respondWith(caches.match(e.request).then(r=>r||fetch(e.request))));"##;
