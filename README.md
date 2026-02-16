# Coolify Expo Notification Relay

This is a simple rust server that relays webhook payloads to the Expo push notification service.

## Instructions For Setting Up Coolifynager

1. Deploy an application in Coolify using the docker image or the github repository. It's recommended to use the docker image instead of repository. `ghcr.io/jacxk/coolify-expo-notification-relay:latest`.
2. Before you deploy the application, you need to set the required [Environment variables](#environment-variables). You can find your Expo push tokens in the app settings.
3. Deploy the application and save the url where the application is deployed. It can be found in the Domains section of the application.
4. Navigate to the "Notifications" page and the "Webhooks" section, and paste the url where the application is deployed in the "Webhook URL" field. \
   *If you added a WEBHOOK_PATH environment variable, you need to append it to the url.*
5. Select any of the events you want to receive notifications for, save and Send Test Notification. You should receive a notification on your device.

## Run with Docker

1. Pull the docker image:
   ```bash
   docker pull ghcr.io/jacxk/coolify-expo-notification-relay:latest
   ```
2. Run the container:
   ```bash
   docker run -d --name coolify-expo-notification-relay /
    -e EXPO_PUSH_TOKENS="token1,token2" \
    -e PORT="3000" \
    ghcr.io/jacxk/coolify-expo-notification-relay:latest
   ```

## Run with Docker Compose

1. Create a `docker-compose.yml` file:
   ```yaml
   services:
     coolify-expo-notification-relay:
       image: ghcr.io/jacxk/coolify-expo-notification-relay:latest
       ports:
         - "3000:3000"
       environment:
         - EXPO_PUSH_TOKENS=token1,token2 # Comma-separated Expo push tokens (e.g. `ExponentPushToken[xxx]`) that receive notifications.
         - PORT=3000 # Port the server listens on.
         # - WEBHOOK_PATH=/ # URL path for the webhook endpoint.
         # - WEBHOOK_RELAY_URLS=url1,url2 # Comma-separated URLs to forward the raw webhook payload to (optional relay).
         # - EXPO_PUSH_URL=https://exp.host/--/api/v2/push/send # Expo push API URL.
       restart: always
    ```
2. Run the container:
   ```bash
   docker-compose up -d
   ```

## Build From Source

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


## Environment variables

| Name                              | Required    | Default                                | Description                                                                                  |
| --------------------------------- | ----------- | -------------------------------------- | -------------------------------------------------------------------------------------------- |
| `EXPO_PUSH_TOKENS`                | Yes         | —                                      | Comma-separated Expo push tokens (e.g. `ExponentPushToken[xxx]`) that receive notifications. |
| `EXPO_PUSH_URL`                   | No          | `https://exp.host/--/api/v2/push/send` | Expo push API URL.                                                                           |
| `COOLIFY_API_URL`                 | No          | —                                      | Coolify base API URL. Polling requests are sent to `{COOLIFY_API_URL}/api/v1/deployments`.   |
| `COOLIFY_API_TOKEN`               | Conditional | —                                      | API token used for Coolify API calls. Required when `COOLIFY_API_URL` is set.                |
| `COOLIFY_API_ENDPOINT`            | No          | `api/v1/deployments`                   | API endpoint for polling deployments.                                                        |
| `COOLIFY_DEPLOYMENT_POLL_SECONDS` | No          | `10`                                   | Polling interval in seconds for checking `/api/v1/deployments`.                              |
| `PORT`                            | No          | `3000`                                 | Port the server listens on.                                                                  |
| `WEBHOOK_PATH`                    | No          | `/`                                    | URL path for the webhook endpoint.                                                           |
| `WEBHOOK_RELAY_URLS`              | No          | —                                      | Comma-separated URLs to forward the raw webhook payload to (optional relay).                 |

