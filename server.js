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
const EXPO_PUSH_TOKENS =
  process.env.EXPO_PUSH_TOKENS || process.env.EXPO_PUSH_TOKEN || "";
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

app.use(express.json({ limit: "1mb" }));

app.get("/", (request, response) => {
  response.status(200).json({
    ok: true,
    service: "coolify-expo-webhook-relay",
  });
});

app.get("/healthz", (request, response) => {
  response.status(200).json({ ok: true });
});

app.post(WEBHOOK_PATH, async (request, response) => {
  if (!isAuthorized(request)) {
    return response.status(401).json({ ok: false, error: "Unauthorized" });
  }

  if (!tokens.length) {
    return response.status(500).json({
      ok: false,
      error: "EXPO_PUSH_TOKEN(S) not configured",
    });
  }

  const payload = request.body || {};
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

  const headers = {
    "Content-Type": "application/json",
  };
  if (EXPO_ACCESS_TOKEN) {
    headers.Authorization = `Bearer ${EXPO_ACCESS_TOKEN}`;
  }

  try {
    const expoResponse = await fetch(EXPO_PUSH_URL, {
      method: "POST",
      headers,
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
  log.info(`Webhook relay listening on port ${PORT}`);
});
