// ---------------------------------------------------------------------------
// Event groups: event name -> handler. Add new events in the appropriate group.
// ---------------------------------------------------------------------------

const EVENT_GROUPS = {
  docker: {
    docker_cleanup_success: dockerCleanupSuccessEvent,
    docker_cleanup_failed: dockerCleanupFailedEvent,
  },
  database: {
    database_backup_success: databaseBackupSuccessEvent,
    database_backup_failed: databaseBackupFailedEvent,
    backup_success_with_s3_warning: backupSuccessWithS3WarningEvent,
  },
  server: {
    server_patches_available: serverPatchesAvailableEvent,
    server_patch_check: serverPatchCheckEvent,
    server_reachable: serverReachableEvent,
    server_unreachable: serverUnreachableEvent,
    high_disk_usage: highDiskUsageEvent,
  },
  deployment: {
    deployment_success: deploymentSuccessEvent,
    deployment_failed: deploymentFailedEvent,
  },
  container: {
    container_stopped: containerStoppedEvent,
    container_restarted: containerRestartedEvent,
    status_changed: statusChangedEvent,
  },
  traefik: {
    traefik_version_outdated: traefikVersionOutdatedEvent,
  },
  task: {
    task_success: taskSuccessEvent,
    task_failed: taskFailedEvent,
  },
  test: {
    test: testEvent,
  },
};

const EVENT_HANDLERS = Object.values(EVENT_GROUPS).reduce(
  (acc, group) => ({ ...acc, ...group }),
  {}
);

function eventParser(payload) {
  const event = payload.event;
  const handler = EVENT_HANDLERS[event];
  if (handler) {
    return handler(payload);
  }
  return {
    title: "Event: " + event,
    body: payload.message,
  };
}

function dockerCleanupSuccessEvent(payload) {
  return {
    title: "Docker Cleanup Success",
    body: `Docker cleanup job succeeded on server ${payload.server_name}`,
  };
}

function dockerCleanupFailedEvent(payload) {
  return {
    title: "Docker Cleanup Failed",
    body: `Docker cleanup job failed on server ${payload.server_name}`,
  };
}

function databaseBackupSuccessEvent(payload) {
  return {
    title: "Database Backup Success",
    body: `Database backup job succeeded on database ${payload.database_name}`,
  };
}

function databaseBackupFailedEvent(payload) {
  return {
    title: "Database Backup Failed",
    body: `Database backup job failed on database ${payload.database_name}`,
  };
}

function serverPatchesAvailableEvent(payload) {
  return {
    title: "Server Patches Available",
    body: `${payload.total_updates} patches are available for server ${payload.server_name}`,
  };
}

function serverPatchCheckEvent(payload) {
  return {
    title: "Server Patches Available",
    body: `${payload.total_updates} patches are available for server ${payload.server_name}`,
  };
}

function serverReachableEvent(payload) {
  return {
    title: "Server Revived",
    body: `Server ${payload.server_name} is back online`,
  };
}

function serverUnreachableEvent(payload) {
  return {
    title: "Server Unreachable",
    body: `Server ${payload.server_name} is unreachable`,
  };
}

function highDiskUsageEvent(payload) {
  return {
    title: "High Disk Usage Detected",
    body: `Server ${payload.server_name} is using ${payload.disk_usage}% of its disk space, which is above the threshold of ${payload.threshold}%`,
  };
}

function deploymentSuccessEvent(payload) {
  let title = "Deployment Success";
  if (payload.preview_fqdn) {
    title = "Preview Deployment Success";
  } else {
    title = "Deployment Success";
  }

  return {
    title,
    body: `${payload.application_name} was deployed successfully for ${payload.project}`,
  };
}

function deploymentFailedEvent(payload) {
  let title = "Deployment Failed";
  if (payload.preview_fqdn) {
    title = "Preview Deployment Failed";
  } else {
    title = "Deployment Failed";
  }

  return {
    title,
    body: `Deployment of ${payload.application_name} for ${payload.project} failed`,
  };
}

function containerStoppedEvent(payload) {
  return {
    title: "Resource Stopped Unexpectedly",
    body: `Resource ${payload.container_name} was stopped unexpectedly on server ${payload.server_name}`,
  };
}

function containerRestartedEvent(payload) {
  return {
    title: "Resource Restarted Automatically",
    body: `Resource ${payload.container_name} was restarted automatically on server ${payload.server_name}`,
  };
}

function statusChangedEvent(payload) {
  if (payload.title.toLowerCase() === "application stopped") {
    return {
      title: "Application Stopped",
      body: `Application ${payload.application_name} has been stopped`,
    }
  }
}

function traefikVersionOutdatedEvent(payload) {
  return {
    title: "Traefik Version Outdated",
    body: `Traefik version for ${payload.affected_servers_count} servers is outdated`,
  };
}

function taskSuccessEvent(payload) {
  return {
    title: "Scheduled Task Success",
    body: `Scheduled task ${payload.task_name} was successful`,
  };
}

function taskFailedEvent(payload) {
  return {
    title: "Scheduled Task Failed",
    body: `Scheduled task ${payload.task_name} failed`,
  };
}

function backupSuccessWithS3WarningEvent(payload) {
  return {
    title: "Local Backup Success, S3 Backup Failed",
    body: `Local backup of ${payload.database_name} was successful, but S3 backup failed`,
  };
}

function testEvent() {
  return {
    title: "Coolify Test Event",
    body: "Test event received",
  };
}
