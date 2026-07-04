# Clip Bridge

Clip Bridge 是一个 Rust 写的 P2P 剪贴板和文件传递工具。HTML/CSS/JS 编译进可执行文件里，部署时发布一个二进制即可；TLS 交给 nginx/Caddy 反代处理。

## 路由

- `/server`：WebSocket signaling server，只转发 WebRTC 的 `join / offer / answer / ICE`，不传剪贴板或文件正文。
- `/server/config`：返回浏览器 WebRTC 使用的 `iceServers`。
- `/<password>`：前端页面。除 `/server`、`/server/config` 和 `/` 外，大部分路径都可以当 password 房间。

## 传递方式

剪贴板文本和文件正文都通过 WebRTC DataChannel 在浏览器之间传递。服务端只负责让同一个 password 房间里的浏览器互相发现和交换连接信息。

这意味着：

- 服务端不保存剪贴板历史，也不保存文件。
- 新进入房间的人不会看到之前的剪贴板内容，只会收到进入之后的新传递。
- 直连成功时，剪贴板和文件正文不经过 server。
- NAT 打洞失败时，需要 TURN 兜底；使用 TURN 时，中继流量会经过 TURN 服务器，但业务 WebSocket 仍然不传正文。

## 页面操作

- 点击主区域：读取本机剪贴板；如果和本页最新一条不同，就通过 P2P 发送给同房间设备。
- 收到剪贴板：下方本地列表会自动增加一条，不需要额外操作。
- 点击列表行：把那一条写入本机剪贴板。
- 发送文件：选择一个文件后进入待发送状态；其他同房间设备会看到一个下载卡片，点击下载后通过 P2P 分片传输。
- 清空：清空本地列表和当前文件卡片，并通过 P2P 通知当前已连接的同房间页面清空。

浏览器剪贴板 API 需要安全上下文：`https://` 或 `localhost/127.0.0.1`。公网部署时建议用 nginx 处理 HTTPS 和 WebSocket。

## 运行

```bash
cargo run --release -- --bind 0.0.0.0:7259
```

配置 STUN/TURN：

```bash
cargo run --release -- \
  --bind 0.0.0.0:7259 \
  --ice-server stun:stun.example.com:3478 \
  --ice-server turns:turn.example.com:5349,user,pass
```

`--ice-server` 可以重复。格式是：

```text
URL
URL,USERNAME,CREDENTIAL
```

打开：

```text
https://your-domain.example/my-password
```

## 测试

Playwright E2E 会启动 Rust 服务、打开两个 Chromium 页面、等待 WebRTC DataChannel 打通、验证剪贴板和文件都通过 P2P 到达另一边。

```bash
npm run test:e2e
npm run test:e2e:report
```

## nginx 示例

```nginx
server {
    listen 443 ssl http2;
    server_name clip.example.com;

    ssl_certificate /etc/letsencrypt/live/clip.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/clip.example.com/privkey.pem;

    location / {
        proxy_pass http://127.0.0.1:7259;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

## 名字备选

- Clip Bridge
- Peer Drop
- Room Relay
- Clip Ferry
- Pocket Relay
