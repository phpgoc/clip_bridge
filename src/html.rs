pub const ROOT_HTML: &str = r#"<!doctype html>
<html lang="zh-CN">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Clip Bridge</title>
  <style>
    :root { color-scheme: light dark; font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; }
    body { margin: 0; min-height: 100vh; display: grid; place-items: center; background: #f5f6f3; color: #1b1f24; }
    main { width: min(520px, calc(100vw - 32px)); }
    h1 { font-size: 28px; margin: 0 0 12px; }
    form { display: flex; gap: 8px; }
    input { flex: 1; min-width: 0; padding: 12px; border: 1px solid #bac1cc; border-radius: 6px; font: inherit; }
    button { padding: 0 16px; border: 0; border-radius: 6px; background: #1768e5; color: white; font: inherit; cursor: pointer; }
    p { color: #5d6673; line-height: 1.6; }
  </style>
</head>
<body>
  <main>
    <h1>Clip Bridge</h1>
    <p>在 URL 后面放一个 password，同一个地址就是同一个 P2P 剪贴板和文件房间。</p>
    <form id="form">
      <input id="room" autocomplete="off" placeholder="password">
      <button>打开</button>
    </form>
  </main>
  <script>
    const formEl = document.getElementById('form');
    const roomEl = document.getElementById('room');

    formEl.addEventListener('submit', (event) => {
      event.preventDefault();
      const value = roomEl.value.trim();
      if (value && value !== 'server') location.href = '/' + encodeURIComponent(value);
    });
  </script>
</body>
</html>"#;

pub const APP_HTML: &str = r#"<!doctype html>
<html lang="zh-CN">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Clip Bridge</title>
  <style>
    :root {
      color-scheme: light dark;
      --bg: #f5f6f3;
      --panel: #ffffff;
      --text: #1b1f24;
      --muted: #5d6673;
      --line: #d8dde5;
      --strong: #1768e5;
      --danger: #b42318;
      --ok: #117a50;
      font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    }

    * { box-sizing: border-box; }
    body { margin: 0; min-height: 100vh; background: var(--bg); color: var(--text); }
    button { font: inherit; }
    .shell { width: min(760px, calc(100vw - 24px)); margin: 0 auto; padding: 18px 0 28px; }
    .topbar { display: flex; justify-content: flex-end; gap: 8px; margin-bottom: 10px; }
    .button {
      min-height: 38px;
      padding: 0 14px;
      border-radius: 6px;
      border: 1px solid var(--line);
      background: var(--panel);
      color: var(--text);
      cursor: pointer;
    }
    .button.primary { color: #fff; border-color: var(--strong); background: var(--strong); }
    .button.danger { color: var(--danger); border-color: color-mix(in srgb, var(--danger) 35%, var(--line)); }
    .hint { min-height: 24px; margin: 10px 0 12px; color: var(--muted); font-size: 14px; }
    .hint.error { color: var(--danger); }
    .file-card {
      display: none;
      grid-template-columns: minmax(0, 1fr) auto;
      gap: 10px;
      align-items: center;
      padding: 12px;
      margin-bottom: 12px;
      border: 1px solid var(--line);
      border-radius: 8px;
      background: var(--panel);
    }
    .file-card.show { display: grid; }
    .file-name { overflow-wrap: anywhere; }
    .file-meta { margin-top: 4px; color: var(--muted); font-size: 12px; }
    .list {
      display: grid;
      gap: 8px;
      margin: 0;
      padding: 0;
      list-style: none;
    }
    .row {
      display: grid;
      grid-template-columns: 34px minmax(0, 1fr);
      gap: 10px;
      align-items: start;
      min-height: 56px;
      padding: 10px;
      border: 1px solid var(--line);
      border-radius: 8px;
      background: var(--panel);
      cursor: pointer;
    }
    .row.selected { border-color: var(--strong); box-shadow: inset 0 0 0 1px var(--strong); }
    .select-box {
      width: 24px;
      height: 24px;
      margin: 3px 0 0;
      accent-color: var(--strong);
      cursor: pointer;
    }
    .preview { min-width: 0; }
    .preview-text {
      margin: 0;
      max-height: 72px;
      overflow: hidden;
      white-space: pre-wrap;
      overflow-wrap: anywhere;
      line-height: 1.45;
    }
    .meta { margin-top: 5px; color: var(--muted); font-size: 12px; }
    .empty-list {
      padding: 14px;
      border: 1px dashed var(--line);
      border-radius: 8px;
      color: var(--muted);
      background: color-mix(in srgb, var(--panel) 80%, transparent);
    }
    .hidden { display: none; }

    @media (max-width: 560px) {
      .shell { width: min(100vw - 16px, 760px); padding-top: 10px; }
      .topbar { justify-content: stretch; }
      .button { flex: 1; }
      .file-card { grid-template-columns: 1fr; }
    }

    @media (prefers-color-scheme: dark) {
      :root { --bg: #111418; --panel: #191e24; --text: #eef1f5; --muted: #9ba5b4; --line: #303842; }
    }
  </style>
</head>
<body>
  <main class="shell">
    <div class="topbar">
      <input id="fileInput" data-testid="file-input" class="hidden" type="file">
      <button id="fileButton" data-testid="file-send" class="button primary">发送文件</button>
      <button id="clearButton" data-testid="clear" class="button danger">清空</button>
    </div>

    <div id="toast" data-testid="toast" class="hint"></div>

    <section id="fileCard" data-testid="file-card" class="file-card">
      <div>
        <div id="fileName" data-testid="file-name" class="file-name"></div>
        <div id="fileMeta" data-testid="file-meta" class="file-meta"></div>
      </div>
      <button id="downloadButton" data-testid="file-download" class="button primary">下载</button>
    </section>

    <ul id="history" data-testid="history-list" class="list"></ul>
  </main>

  <script>
    const room = decodeURIComponent(location.pathname.replace(/^\/+/, '')) || 'default';
    const peerId = crypto.randomUUID ? crypto.randomUUID() : `${Date.now()}-${Math.random()}`;
    const clearButtonEl = document.getElementById('clearButton');
    const toastEl = document.getElementById('toast');
    const historyEl = document.getElementById('history');
    const fileInputEl = document.getElementById('fileInput');
    const fileButtonEl = document.getElementById('fileButton');
    const fileCardEl = document.getElementById('fileCard');
    const fileNameEl = document.getElementById('fileName');
    const fileMetaEl = document.getElementById('fileMeta');
    const downloadButtonEl = document.getElementById('downloadButton');
    let rtcConfig = { iceServers: [{ urls: 'stun:stun.l.google.com:19302' }] };
    const peers = new Map();
    const seenMessages = new Set();
    const outboundFiles = new Map();
    const inboundFiles = new Map();
    let currentFileOffer = null;
    let selected = null;
    let historyItems = [];
    let sequence = 0;
    let signalSocket = null;

    window.__serverClipboard = {
      peerId: () => peerId,
      peerCount: () => openChannels().length,
      historyCount: () => historyItems.length,
      currentFileName: () => currentFileOffer?.name || '',
    };

    function setToast(message, isError = false) {
      toastEl.textContent = message;
      toastEl.classList.toggle('error', isError);
    }

    function formatTime(seconds) {
      if (!seconds) return '';
      return new Date(seconds * 1000).toLocaleString();
    }

    function formatBytes(size) {
      if (size < 1024) return size + ' B';
      if (size < 1024 * 1024) return (size / 1024).toFixed(1) + ' KB';
      return (size / 1024 / 1024).toFixed(1) + ' MB';
    }

    function render() {
      historyEl.innerHTML = '';

      if (!historyItems.length) {
        const empty = document.createElement('li');
        empty.className = 'empty-list';
        empty.dataset.testid = 'empty-history';
        empty.textContent = '暂无剪贴板记录';
        historyEl.appendChild(empty);
        return;
      }

      for (const item of historyItems) {
        const li = document.createElement('li');
        li.className = 'row';
        li.dataset.testid = 'history-row';
        if (selected?.id === item.id) li.classList.add('selected');

        const input = document.createElement('input');
        input.className = 'select-box';
        input.type = 'radio';
        input.name = 'selected_clipboard_item';
        input.checked = selected?.id === item.id;
        input.dataset.testid = 'history-select';

        const preview = document.createElement('div');
        preview.className = 'preview';
        const text = document.createElement('p');
        text.className = 'preview-text';
        text.textContent = item.text;
        const meta = document.createElement('div');
        meta.className = 'meta';
        meta.textContent = `#${item.version} · ${formatTime(item.createdAt)}`;
        preview.append(text, meta);

        li.append(input, preview);
        li.addEventListener('click', () => copyHistoryItem(item));
        historyEl.appendChild(li);
      }
    }

    function renderFileOffer() {
      fileCardEl.classList.toggle('show', Boolean(currentFileOffer));
      if (!currentFileOffer) return;
      fileNameEl.textContent = currentFileOffer.name;
      fileMetaEl.textContent = `${formatBytes(currentFileOffer.size)} · ${currentFileOffer.status || '待下载'}`;
      downloadButtonEl.disabled = currentFileOffer.status === '传输中';
      downloadButtonEl.textContent = currentFileOffer.status === '传输中' ? '传输中' : '下载';
    }

    function addItem(text, messageId = `${peerId}-${Date.now()}-${Math.random()}`) {
      if (!text || seenMessages.has(messageId)) return false;
      seenMessages.add(messageId);
      sequence += 1;
      const item = {
        id: messageId,
        text,
        version: sequence,
        createdAt: Math.floor(Date.now() / 1000),
      };
      historyItems.push(item);
      historyItems = historyItems.slice(-50);
      selected = item;
      render();
      return true;
    }

    async function copyHistoryItem(item) {
      selected = item;
      render();
      try {
        await navigator.clipboard.writeText(item.text);
        setToast('已复制到本机剪贴板');
      } catch (err) {
        setToast(err.message || '复制失败', true);
      }
    }

    function clearLocal() {
      selected = null;
      historyItems = [];
      seenMessages.clear();
      currentFileOffer = null;
      inboundFiles.clear();
      render();
      renderFileOffer();
    }

    function connectSignaling() {
      const protocol = location.protocol === 'https:' ? 'wss:' : 'ws:';
      signalSocket = new WebSocket(`${protocol}//${location.host}/server`);

      signalSocket.onopen = () => {
        sendSignal({ kind: 'join', room, peer_id: peerId });
      };
      signalSocket.onmessage = async (event) => {
        const message = JSON.parse(event.data);
        if (message.kind === 'peers') {
          for (const remoteId of message.peers) await createPeer(remoteId, true);
        }
        if (message.kind === 'peer_left') closePeer(message.peer_id);
        if (message.kind === 'signal') await handleSignal(message.from, message.data);
      };
      signalSocket.onclose = () => {
        for (const remoteId of peers.keys()) closePeer(remoteId);
        setTimeout(connectSignaling, 1200);
      };
    }

    async function loadServerConfig() {
      try {
        const res = await fetch('/server/config', { cache: 'no-store' });
        if (!res.ok) return;
        const config = await res.json();
        if (Array.isArray(config.iceServers) && config.iceServers.length) {
          rtcConfig = { iceServers: config.iceServers };
        }
      } catch (_) {
      }
    }

    function sendSignal(message) {
      if (signalSocket?.readyState === WebSocket.OPEN) {
        signalSocket.send(JSON.stringify(message));
      }
    }

    function openChannels() {
      return Array.from(peers.values())
        .map((peer) => peer.channel)
        .filter((channel) => channel?.readyState === 'open');
    }

    function channelFor(remoteId) {
      const channel = peers.get(remoteId)?.channel;
      return channel?.readyState === 'open' ? channel : null;
    }

    async function createPeer(remoteId, initiator) {
      if (peers.has(remoteId)) return peers.get(remoteId);

      const pc = new RTCPeerConnection(rtcConfig);
      const peer = { pc, channel: null, pendingCandidates: [] };
      peers.set(remoteId, peer);

      pc.onicecandidate = (event) => {
        if (event.candidate) {
          sendSignal({
            kind: 'signal',
            to: remoteId,
            data: { type: 'ice', candidate: event.candidate },
          });
        }
      };
      pc.onconnectionstatechange = () => {
        document.body.dataset.peerCount = String(openChannels().length);
        if (['closed', 'disconnected', 'failed'].includes(pc.connectionState)) closePeer(remoteId);
      };
      pc.ondatachannel = (event) => wireChannel(remoteId, event.channel);

      if (initiator) {
        wireChannel(remoteId, pc.createDataChannel('clipboard'));
        const offer = await pc.createOffer();
        await pc.setLocalDescription(offer);
        sendSignal({
          kind: 'signal',
          to: remoteId,
          data: { type: 'offer', description: pc.localDescription },
        });
      }

      return peer;
    }

    function wireChannel(remoteId, channel) {
      const peer = peers.get(remoteId);
      if (!peer) return;
      peer.channel = channel;
      channel.onopen = () => {
        document.body.dataset.peerCount = String(openChannels().length);
      };
      channel.onclose = () => {
        document.body.dataset.peerCount = String(openChannels().length);
      };
      channel.onmessage = (event) => {
        const message = JSON.parse(event.data);
        if (message.kind === 'clipboard_text') addItem(message.text, message.id);
        if (message.kind === 'clear') clearLocal();
        if (message.kind === 'file_offer') receiveFileOffer(remoteId, message);
        if (message.kind === 'file_request') sendFile(remoteId, message.id);
        if (message.kind === 'file_start') receiveFileStart(remoteId, message);
        if (message.kind === 'file_chunk') receiveFileChunk(message);
        if (message.kind === 'file_end') receiveFileEnd(message.id);
      };
    }

    async function handleSignal(remoteId, data) {
      const peer = await createPeer(remoteId, false);

      if (data.type === 'offer') {
        await peer.pc.setRemoteDescription(data.description);
        const answer = await peer.pc.createAnswer();
        await peer.pc.setLocalDescription(answer);
        sendSignal({
          kind: 'signal',
          to: remoteId,
          data: { type: 'answer', description: peer.pc.localDescription },
        });
        await flushCandidates(peer);
      }

      if (data.type === 'answer') {
        await peer.pc.setRemoteDescription(data.description);
        await flushCandidates(peer);
      }

      if (data.type === 'ice') {
        if (peer.pc.remoteDescription) {
          await peer.pc.addIceCandidate(data.candidate);
        } else {
          peer.pendingCandidates.push(data.candidate);
        }
      }
    }

    async function flushCandidates(peer) {
      while (peer.pendingCandidates.length) {
        await peer.pc.addIceCandidate(peer.pendingCandidates.shift());
      }
    }

    function closePeer(remoteId) {
      const peer = peers.get(remoteId);
      if (!peer) return;
      peer.channel?.close();
      peer.pc.close();
      peers.delete(remoteId);
      document.body.dataset.peerCount = String(openChannels().length);
    }

    function broadcast(message) {
      const body = JSON.stringify(message);
      for (const channel of openChannels()) channel.send(body);
    }

    async function clearRoom() {
      clearLocal();
      broadcast({ kind: 'clear' });
      setToast('已清空');
    }

    async function clickClipboard() {
      try {
        if (!navigator.clipboard) {
          setToast('剪贴板不可用，请使用 HTTPS 或 localhost', true);
          return;
        }

        const clip = await navigator.clipboard.readText();
        if (!clip) {
          setToast('本机剪贴板为空', true);
          return;
        }
        if (historyItems.at(-1)?.text === clip) {
          setToast('剪贴板没有变化');
          return;
        }

        const id = `${peerId}-${Date.now()}-${Math.random()}`;
        addItem(clip, id);
        broadcast({ kind: 'clipboard_text', id, text: clip });
        setToast(openChannels().length ? '已通过 P2P 发送' : '已保存到本页，等待同房间设备');
      } catch (err) {
        setToast(err.message || '剪贴板操作失败', true);
      }
    }

    function chooseFile() {
      fileInputEl.click();
    }

    function offerSelectedFile() {
      const file = fileInputEl.files?.[0];
      if (!file) return;
      const id = `${peerId}-file-${Date.now()}-${Math.random()}`;
      outboundFiles.set(id, file);
      currentFileOffer = {
        id,
        peerId,
        name: file.name,
        size: file.size,
        type: file.type || 'application/octet-stream',
        status: openChannels().length ? '待下载' : '等待同房间设备',
      };
      renderFileOffer();
      broadcast({
        kind: 'file_offer',
        id,
        name: file.name,
        size: file.size,
        type: file.type || 'application/octet-stream',
      });
      setToast(openChannels().length ? '文件已准备发送' : '文件已选择，等待同房间设备');
    }

    function receiveFileOffer(remoteId, message) {
      currentFileOffer = {
        id: message.id,
        peerId: remoteId,
        name: message.name,
        size: message.size,
        type: message.type || 'application/octet-stream',
        status: '待下载',
      };
      renderFileOffer();
      setToast('收到文件，可点击下载');
    }

    function requestCurrentFile() {
      if (!currentFileOffer) return;
      if (currentFileOffer.peerId === peerId) {
        setToast('这是本机待发送文件');
        return;
      }
      const channel = channelFor(currentFileOffer.peerId);
      if (!channel) {
        setToast('发送方已离线', true);
        return;
      }
      currentFileOffer.status = '传输中';
      renderFileOffer();
      channel.send(JSON.stringify({ kind: 'file_request', id: currentFileOffer.id }));
    }

    async function sendFile(remoteId, fileId) {
      const file = outboundFiles.get(fileId);
      const channel = channelFor(remoteId);
      if (!file || !channel) return;

      const chunkSize = 12 * 1024;
      const totalChunks = Math.ceil(file.size / chunkSize);
      channel.send(JSON.stringify({
        kind: 'file_start',
        id: fileId,
        name: file.name,
        size: file.size,
        type: file.type || 'application/octet-stream',
        totalChunks,
      }));

      for (let index = 0; index < totalChunks; index += 1) {
        const start = index * chunkSize;
        const chunk = await file.slice(start, start + chunkSize).arrayBuffer();
        channel.send(JSON.stringify({
          kind: 'file_chunk',
          id: fileId,
          index,
          data: arrayBufferToBase64(chunk),
        }));
        await waitForBufferedAmount(channel);
      }

      channel.send(JSON.stringify({ kind: 'file_end', id: fileId }));
      setToast('文件已发送');
    }

    function receiveFileStart(remoteId, message) {
      inboundFiles.set(message.id, {
        peerId: remoteId,
        name: message.name,
        size: message.size,
        type: message.type || 'application/octet-stream',
        totalChunks: message.totalChunks,
        chunks: new Array(message.totalChunks),
        received: 0,
      });
      currentFileOffer = {
        id: message.id,
        peerId: remoteId,
        name: message.name,
        size: message.size,
        type: message.type || 'application/octet-stream',
        status: '传输中',
      };
      renderFileOffer();
    }

    function receiveFileChunk(message) {
      const transfer = inboundFiles.get(message.id);
      if (!transfer || transfer.chunks[message.index]) return;
      const bytes = base64ToBytes(message.data);
      transfer.chunks[message.index] = bytes;
      transfer.received += bytes.byteLength;
      if (currentFileOffer?.id === message.id) {
        currentFileOffer.status = `${Math.round((transfer.received / transfer.size) * 100)}%`;
        renderFileOffer();
      }
    }

    function receiveFileEnd(fileId) {
      const transfer = inboundFiles.get(fileId);
      if (!transfer) return;
      const blob = new Blob(transfer.chunks, { type: transfer.type });
      const url = URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = transfer.name;
      document.body.appendChild(link);
      link.click();
      link.remove();
      setTimeout(() => URL.revokeObjectURL(url), 1000);
      inboundFiles.delete(fileId);
      if (currentFileOffer?.id === fileId) {
        currentFileOffer.status = '完成';
        renderFileOffer();
      }
      setToast('文件已下载');
    }

    function arrayBufferToBase64(buffer) {
      let binary = '';
      const bytes = new Uint8Array(buffer);
      for (let i = 0; i < bytes.length; i += 1) binary += String.fromCharCode(bytes[i]);
      return btoa(binary);
    }

    function base64ToBytes(value) {
      const binary = atob(value);
      const bytes = new Uint8Array(binary.length);
      for (let i = 0; i < binary.length; i += 1) bytes[i] = binary.charCodeAt(i);
      return bytes;
    }

    async function waitForBufferedAmount(channel) {
      while (channel.bufferedAmount > 512 * 1024) {
        await new Promise((resolve) => setTimeout(resolve, 20));
      }
    }

    function isBlankClickTarget(target) {
      return !target.closest('button, input, a, label, .row, .file-card');
    }

    window.addEventListener('click', (event) => {
      if (event.button === 0 && isBlankClickTarget(event.target)) clickClipboard();
    });
    clearButtonEl.addEventListener('click', clearRoom);
    fileButtonEl.addEventListener('click', chooseFile);
    fileInputEl.addEventListener('change', offerSelectedFile);
    downloadButtonEl.addEventListener('click', requestCurrentFile);

    async function start() {
      render();
      renderFileOffer();
      await loadServerConfig();
      connectSignaling();
    }

    start();
  </script>
</body>
</html>"#;
