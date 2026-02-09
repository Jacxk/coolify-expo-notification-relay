# Coolify Expo Notification Relay

This is a simple rust server that relays webhook payloads to the Expo push notification service.

## Environment variables

| Name                 | Required | Default                                | Description                                                                                  |
| -------------------- | -------- | -------------------------------------- | -------------------------------------------------------------------------------------------- |
| `EXPO_PUSH_TOKENS`   | Yes      | —                                      | Comma-separated Expo push tokens (e.g. `ExponentPushToken[xxx]`) that receive notifications. |
| `EXPO_PUSH_URL`      | No       | `https://exp.host/--/api/v2/push/send` | Expo push API URL.                                                                           |
| `PORT`               | No       | `3000`                                 | Port the server listens on.                                                                  |
| `WEBHOOK_PATH`       | No       | `/`                                    | URL path for the webhook endpoint.                                                           |
| `WEBHOOK_RELAY_URLS` | No       | —                                      | Comma-separated URLs to forward the raw webhook payload to (optional relay).                 |

## Build

1. **Build the release binary** 
   From the project root, run:
   ```bash
   cargo build --release
   ```

2. **Make the binary executable**  
   Set execute permission on the built binary:
   ```bash
   chmod +x target/release/coolify-expo-notification-relay
   ```

3. **Run the server with your Expo push tokens**  
   Start the relay with the required `EXPO_PUSH_TOKENS` environment variable (replace with your token(s)):
   ```bash
   EXPO_PUSH_TOKENS="token1,token2" ./target/release/coolify-expo-notification-relay
   ```

