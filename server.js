const express = require("express");

const app = express();

const PORT = Number.parseInt(process.env.PORT || "3000", 10);
const WEBHOOK_PATH = process.env.WEBHOOK_PATH || "/webhook";
const WEBHOOK_SECRET = process.env.WEBHOOK_SECRET || "";
const EXPO_PUSH_URL =
  process.env.EXPO_PUSH_URL || "https://exp.host/--/api/v2/push/send";
const EXPO_ACCESS_TOKEN = process.env.EXPO_ACCESS_TOKEN || "";
const EXPO_TITLE_PREFIX = process.env.EXPO_TITLE_PREFIX || "Coolify";
const EXPO_BODY_FALLBACK =
  process.env.EXPO_BODY_FALLBACK || "Coolify event received";
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

if (!tokens.length) {
  throw new Error(
    "EXPO_PUSH_TOKENS is required and must contain at least one token."
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

function buildTitle(payload) {
  const event =
    payload && typeof payload === "object" && payload.event
      ? String(payload.event)
      : "";
  if (event) {
    return `${EXPO_TITLE_PREFIX}: ${event}`;
  }
  return EXPO_TITLE_PREFIX;
}

function buildBody(payload) {
  if (payload && typeof payload === "object") {
    if (payload.message) {
      return String(payload.message);
    }
    if (payload.cleanup_message) {
      return String(payload.cleanup_message);
    }
  }
  return EXPO_BODY_FALLBACK;
}

function buildData(payload) {
  const event =
    payload && typeof payload === "object" ? payload.event : undefined;
  const success =
    payload && typeof payload === "object" ? payload.success : undefined;
  const url = payload && typeof payload === "object" ? payload.url : undefined;

  return {
    source: "coolify",
    event,
    success,
    url,
    payload,
    receivedAt: new Date().toISOString(),
  };
}

async function relayPayload(payload) {
  if (!relayUrls.length) {
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
    })
  );

  return results;
}

app.use(express.json({ limit: "1mb" }));

app.get("/", (request, response) => {
  response.status(200).json({
    ok: true,
    service: "coolify-expo-webhook-relay",
  });
});

app.get("/health", (request, response) => {
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

  const notificationTitle = buildTitle(payload);
  const notificationBody = buildBody(payload);
  const notificationData = buildData(payload);

  const notifications = tokens.map((token) => ({
    to: token,
    title: notificationTitle,
    body: notificationBody,
    sound: "default",
    data: notificationData,
  }));

  log.debug("Prepared Expo notifications", {
    count: notifications.length,
    title: notificationTitle,
  });

  const headers = {
    "Content-Type": "application/json",
  };
  if (EXPO_ACCESS_TOKEN) {
    headers.Authorization = `Bearer ${EXPO_ACCESS_TOKEN}`;
  }

  const relayPromise = relayPayload(payload);

  try {
    const expoResponse = await fetch(EXPO_PUSH_URL, {
      method: "POST",
      headers,
      body: JSON.stringify(notifications),
    });

    const expoBody = await expoResponse.json().catch(() => null);

    const relayResults = await relayPromise;
    const failedRelays = relayResults.filter((relay) => !relay.ok);
    if (failedRelays.length) {
      log.warn("Relay webhook(s) failed", failedRelays);
    }

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
        relays: relayResults,
      });
    }

    log.info("Expo push sent", { status: expoResponse.status });
    return response.status(200).json({
      ok: true,
      expo: expoBody,
      relays: relayResults,
    });
  } catch (error) {
    const relayResults = await relayPromise;
    const failedRelays = relayResults.filter((relay) => !relay.ok);
    if (failedRelays.length) {
      log.warn("Relay webhook(s) failed", failedRelays);
    }

    log.error("Failed to send Expo notification", error);
    return response.status(500).json({
      ok: false,
      error: "Failed to send Expo notification",
      relays: relayResults,
    });
  }
});

app.listen(PORT, () => {
  log.info(`Webhook relay listening on http://localhost:${PORT}`);
  log.info(`Webhook endpoint: http://localhost:${PORT}${WEBHOOK_PATH}`);
  log.info(`Health endpoint: http://localhost:${PORT}/health`);
});
