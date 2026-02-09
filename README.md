# Coolify Expo Notification Relay

This is a simple rust server that relays webhook payloads to the Expo push notification service.

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

