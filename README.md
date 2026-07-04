# P2P Clip Bridge Server

[English](README.md) | [中文](README.zh-CN.md)

P2P Clip Bridge Server is a small Rust server for sharing clipboard text and files between devices in the same URL room.

- The room password is the URL path, for example `/my-room`.
- Clipboard text and file bytes are transferred over WebRTC DataChannels.
- WebSocket is used only for WebRTC signaling at `/clip_bridge_server`.
- The server does not store clipboard text or files.
- Files are offered by name and size first; bytes are sent only when another peer clicks Download.
- A built-in UDP TURN relay is started by the same process for fallback when direct P2P fails.

## Run

```bash
p2p_clip_bridge_server -i 203.0.113.10 -u user -c pass -p 7259
```

With an explicit TURN UDP port:

```bash
p2p_clip_bridge_server -i 203.0.113.10 -u user -c pass -p 7259 -t 3478
```

Environment variables are also supported:

```bash
export P2P_CLIP_BRIDGE_SERVER_TURN_PUBLIC_IP=127.0.0.1
export P2P_CLIP_BRIDGE_SERVER_TURN_USERNAME=user
export P2P_CLIP_BRIDGE_SERVER_TURN_CREDENTIAL=pass
p2p_clip_bridge_server -p 7259
```

Open:

```text
https://your-domain.example/room-password
```

## Options

P2P Clip Bridge Server always listens on `0.0.0.0`. You only choose ports and TURN credentials:

```text
-p, --port <PORT>              HTTP/WebSocket port, default 7259
-t, --turn-port <TURN_PORT>    UDP TURN port, default 3478
-i, --ip <PUBLIC_IP>           public relay IP
-u, --username <USERNAME>      TURN username
-c, --credential <PASSWORD>    TURN password
-d, --debug                    print signaling and TURN auth events
```

If required TURN settings are missing, startup prints the help text and exits.

## Built-In TURN

P2P Clip Bridge Server starts its own UDP TURN server. The browser receives `turn:<current-host>:<turn-port>` from `/clip_bridge_server/config`, so the TURN hostname follows the page hostname.

Open the TURN UDP port on your server firewall, for example UDP `3478`. TLS for the web UI can still be handled by nginx.

Debug output shows WebSocket signaling events and TURN auth events only. Clipboard text and file bytes are transferred over WebRTC DataChannels and are not printed by the server.

The default ICE list tries direct connections first with:

```text
stun:<current-host>:<turn-port>
stun:stun.miwifi.com:3478
stun:stun.chat.bilibili.com:3478
stun:stun.l.google.com:19302
turn:<current-host>:<turn-port>
```

Public STUN servers are listed in `src/ice.rs` so source builders can tune them for their region.

## Test

```bash
npm install
npm run test:e2e
```
