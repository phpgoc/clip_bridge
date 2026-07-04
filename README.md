# Clip Bridge

[English](README.md) | [中文](README.zh-CN.md)

Clip Bridge is a small Rust server for sharing clipboard text and files between devices in the same URL room.

- The room password is the URL path, for example `/my-room`.
- Clipboard text and file bytes are transferred over WebRTC DataChannels.
- WebSocket is used only for WebRTC signaling at `/server`.
- The server does not store clipboard text or files.
- Files are offered by name and size first; bytes are sent only when another peer clicks Download.
- A built-in UDP TURN relay is started by the same process for fallback when direct P2P fails.

## Run

```bash
clip_bridge -i 203.0.113.10 -u user -c pass -p 7259
```

With an explicit TURN UDP port:

```bash
clip_bridge -i 203.0.113.10 -u user -c pass -p 7259 -t 3478
```

Environment variables are also supported:

```bash
export CLIP_BRIDGE_TURN_PUBLIC_IP=203.0.113.10
export CLIP_BRIDGE_TURN_USERNAME=user
export CLIP_BRIDGE_TURN_CREDENTIAL=pass
clip_bridge -p 7259
```

Open:

```text
https://your-domain.example/room-password
```

## Options

Clip Bridge always listens on `0.0.0.0`. You only choose ports and TURN credentials:

```text
-p, --port <PORT>              HTTP/WebSocket port, default 7259
-t, --turn-port <TURN_PORT>    UDP TURN port, default 3478
-i, --ip <PUBLIC_IP>           public relay IP
-u, --username <USERNAME>      TURN username
-c, --credential <PASSWORD>    TURN password
```

If required TURN settings are missing, startup prints the help text and exits.

## Built-In TURN

Clip Bridge starts its own UDP TURN server. The browser receives `turn:<current-host>:<turn-port>` from `/server/config`, so the TURN hostname follows the page hostname.

Open the TURN UDP port on your server firewall, for example UDP `3478`. TLS for the web UI can still be handled by nginx.

## Test

```bash
npm install
npm run test:e2e
```
