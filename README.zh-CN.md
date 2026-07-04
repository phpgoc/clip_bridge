# Clip Bridge

[English](README.md) | [中文](README.zh-CN.md)

Clip Bridge 是一个小型 Rust 服务，用同一个 URL 房间在多台设备之间传递剪贴板文字和文件。

- 房间密码就是 URL 路径，例如 `/my-room`。
- 剪贴板文字和文件内容通过 WebRTC DataChannel 传输。
- WebSocket 只用于 `/clip_bridge_server` 上的 WebRTC 信令交换。
- 服务端不保存剪贴板文字，也不保存文件内容。
- 文件会先广播文件名和大小；只有其他人点击 Download 后才开始传输文件内容。
- 同一个进程会启动内置 UDP TURN relay，直连失败时用于中继兜底。

## 运行

```bash
clip_bridge -i 203.0.113.10 -u user -c pass -p 7259
```

指定 TURN UDP 端口：

```bash
clip_bridge -i 203.0.113.10 -u user -c pass -p 7259 -t 3478
```

也可以使用环境变量：

```bash
export CLIP_BRIDGE_TURN_PUBLIC_IP=127.0.0.1
export CLIP_BRIDGE_TURN_USERNAME=user
export CLIP_BRIDGE_TURN_CREDENTIAL=pass
clip_bridge -p 7259
```

打开：

```text
https://your-domain.example/room-password
```

## 选项

Clip Bridge 永远监听 `0.0.0.0`，启动时只需要选择端口和 TURN 凭据：

```text
-p, --port <PORT>              HTTP/WebSocket 端口，默认 7259
-t, --turn-port <TURN_PORT>    UDP TURN 端口，默认 3478
-i, --ip <PUBLIC_IP>           对外 relay IP
-u, --username <USERNAME>      TURN 用户名
-c, --credential <PASSWORD>    TURN 密码
-d, --debug                    打印信令和 TURN 认证事件
```

缺少必需 TURN 配置时，启动会打印 help 并退出。

## 内置 TURN

Clip Bridge 会启动自己的 UDP TURN 服务。浏览器从 `/clip_bridge_server/config` 拿到的是 `turn:<current-host>:<turn-port>`，所以 TURN hostname 会跟随当前网页 hostname。

服务器防火墙需要开放 TURN UDP 端口，例如 UDP `3478`。网页 TLS 仍然可以交给 nginx。

Debug 输出只显示 WebSocket 信令事件和 TURN 认证事件。剪贴板文本和文件内容通过 WebRTC DataChannel 传输，服务端不会打印正文。

默认 ICE 列表会先尽量直连：

```text
stun:<current-host>:<turn-port>
stun:stun.miwifi.com:3478
stun:stun.chat.bilibili.com:3478
stun:stun.l.google.com:19302
turn:<current-host>:<turn-port>
```

公共 STUN 列表放在 `src/ice.rs`，从源码编译的人可以按自己的地区调整。

## 测试

```bash
npm install
npm run test:e2e
```
