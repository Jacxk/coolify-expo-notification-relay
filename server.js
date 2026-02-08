const express = require("express");
const eventParser = require("./event-parser");

const app = express();

const PORT = Number.parseInt(process.env.PORT || "3000", 10);
const WEBHOOK_PATH = process.env.WEBHOOK_PATH || "/webhook";
const WEBHOOK_SECRET = process.env.WEBHOOK_SECRET || "";
const EXPO_PUSH_URL =
  process.env.EXPO_PUSH_URL || "https://exp.host/--/api/v2/push/send";
const EXPO_PUSH_TOKENS = process.env.EXPO_PUSH_TOKENS || "";
const WEBHOOK_RELAY_URLS = process.env.WEBHOOK_RELAY_URLS || "";
const LOG_LEVEL = process.env.LOG_LEVEL || "info";

const log = {
  info: (...args) => {
    if (LOG_LEVEL === "info" || LOG_LEVEL === "debug") {
      console.log(...args);
    }
  },
  debug: (...args) => {
    if (LOG_LEVEL === "debug") {
      console.log(...args);
    }
  },
  warn: (...args) => console.warn(...args),
  error: (...args) => console.error(...args),
};

const tokens = EXPO_PUSH_TOKENS.split(",")
  .map((token) => token.trim())
  .filter(Boolean);
const relayUrls = WEBHOOK_RELAY_URLS.split(",")
  .map((url) => url.trim())
  .filter(Boolean);

if (tokens.length === 0) {
  throw new Error(
    "EXPO_PUSH_TOKENS is required and must contain at least one token.",
  );
}

function isAuthorized(request) {
  if (!WEBHOOK_SECRET) {
    return true;
  }

  const headerSecret = request.get("x-webhook-secret");
  if (headerSecret && headerSecret === WEBHOOK_SECRET) {
    return true;
  }

  const authHeader = request.get("authorization");
  if (authHeader && authHeader.startsWith("Bearer ")) {
    return authHeader.slice("Bearer ".length) === WEBHOOK_SECRET;
  }

  return false;
}

async function relayPayload(payload) {
  if (relayUrls.length === 0) {
    return [];
  }

  const body = JSON.stringify(payload);
  const results = await Promise.all(
    relayUrls.map(async (url) => {
      try {
        const relayResponse = await fetch(url, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            "User-Agent": "Coolify Expo Notification Relay",
          },
          body,
        });

        return {
          url,
          ok: relayResponse.ok,
          status: relayResponse.status,
        };
      } catch (error) {
        return {
          url,
          ok: false,
          error: error instanceof Error ? error.message : "Unknown error",
        };
      }
    }),
  );

  const failedRelays = results.filter((relay) => !relay.ok);
  if (failedRelays.length > 0) {
    log.warn("Relay webhook(s) failed", failedRelays);
  }

  return results;
}

app.use(express.json({ limit: "1mb" }));

app.get("/", (_, response) => {
  response.status(200).json({
    ok: true,
    service: "coolify-expo-webhook-relay",
  });
});

app.get("/health", (_, response) => {
  response.status(200).json({ ok: true });
});

app.post(WEBHOOK_PATH, async (request, response) => {
  if (!isAuthorized(request)) {
    return response.status(401).json({ ok: false, error: "Unauthorized" });
  }

  const payload = request.body || {};
  const eventName =
    payload && typeof payload === "object" && payload.event
      ? String(payload.event)
      : "unknown_event";

  log.info("Received Coolify webhook event", { event: eventName });
  log.debug("Webhook payload received", payload);

  const notification = eventParser(payload);

  const notifications = tokens.map((token) => ({
    ...notification,
    to: token,
    sound: "default",
    data: {
      source: "coolify",
      receivedAt: new Date().toISOString(),
      payload,
    },
  }));

  log.debug("Prepared Expo notifications", {
    count: notifications.length,
    notification,
  });

  relayPayload(payload);

  try {
    const expoResponse = await fetch(EXPO_PUSH_URL, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(notifications),
    });

    const expoBody = await expoResponse.json().catch(() => null);

    if (!expoResponse.ok) {
      log.error("Expo push failed", {
        status: expoResponse.status,
        body: expoBody,
      });
      return response.status(502).json({
        ok: false,
        error: "Expo push failed",
        status: expoResponse.status,
        expo: expoBody,
      });
    }

    log.info("Expo push sent", { status: expoResponse.status });
    return response.status(200).json({
      ok: true,
      expo: expoBody,
    });
  } catch (error) {
    log.error("Failed to send Expo notification", error);
    return response.status(500).json({
      ok: false,
      error: "Failed to send Expo notification",
    });
  }
});

app.listen(PORT, () => {
  log.info(`Webhook relay ready: http://localhost:${PORT}${WEBHOOK_PATH}`);
});
