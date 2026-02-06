# Coolify Expo Notification Relay

This service receives Coolify webhook events and relays them to Expo Push
Notifications so your Expo app can show alerts.

## What it does

- Accepts Coolify webhook POSTs on a configurable path
- Optionally checks a shared webhook secret
- Sends one or more Expo push notifications
- Exposes a simple health endpoint

## Requirements

- Node.js 18+

## Quick start

```bash
npm install
cp .env.example .env
npm start
```

## Environment variables

| Variable | Required | Default | Description |
| --- | --- | --- | --- |
| `PORT` | No | `3000` | Port to listen on |
| `WEBHOOK_PATH` | No | `/webhook` | Path for incoming webhooks |
| `WEBHOOK_SECRET` | No | (empty) | Shared secret for webhook auth |
| `EXPO_PUSH_TOKENS` | Yes | (empty) | Comma-separated Expo push tokens |
| `EXPO_PUSH_TOKEN` | No | (empty) | Single token alternative |
| `EXPO_ACCESS_TOKEN` | No | (empty) | Optional Expo access token |
| `EXPO_TITLE_PREFIX` | No | `Coolify` | Prefix for notification title |
| `EXPO_BODY_FALLBACK` | No | `Coolify event received` | Fallback body text |
| `EXPO_PUSH_URL` | No | `https://exp.host/--/api/v2/push/send` | Expo push API URL |
| `LOG_LEVEL` | No | `info` | `info` or `debug` |

## Coolify webhook configuration

1. Set the webhook URL to your deployed service, for example:
   `https://your-relay.example.com/webhook`
2. If you set `WEBHOOK_SECRET`, add a header:
   - `x-webhook-secret: <YOUR_SECRET>`
   - or `Authorization: Bearer <YOUR_SECRET>`

## Notification mapping

- Title: `EXPO_TITLE_PREFIX: <event>` when `event` is present
- Body: `message` or `cleanup_message`, otherwise `EXPO_BODY_FALLBACK`
- Data: the full webhook payload plus some metadata

## Example payloads

```json
{
  "success": true,
  "message": "Database backup successful",
  "event": "backup_success",
  "database_name": "coolify-db",
  "database_uuid": "i8coskssgsoosgggk80g44sc",
  "database_type": "coolify",
  "frequency": "0 0 * * *",
  "url": "http://cool.jackscode.me/project//environment//database/i8coskssgsoosgggk80g44sc"
}
```

```json
{
  "success": true,
  "message": "Docker cleanup job succeeded",
  "event": "docker_cleanup_success",
  "server_name": "oracle-vps",
  "server_uuid": "cg8g8kkog488cw0o4wgokc0w",
  "cleanup_message": "Forced Docker cleanup job executed successfully. Disk usage before: 8%, Disk usage after: 8%.",
  "url": "http://cool.jackscode.me/server/cg8g8kkog488cw0o4wgokc0w"
}
```

## Local test

```bash
curl -X POST "http://localhost:3000/webhook" \
  -H "Content-Type: application/json" \
  -d '{"event":"backup_success","message":"Database backup successful","success":true}'
```