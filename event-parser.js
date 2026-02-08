function eventParser(payload) {
  const event = payload.event;
  switch (event) {
    case "docker_cleanup_success":
      return dockerCleanupSuccessEvent(payload);
    case "docker_cleanup_failed":
      return dockerCleanupFailedEvent(payload);
    case "database_backup_success":
      return databaseBackupSuccessEvent(payload);
    case "database_backup_failed":
      return databaseBackupFailedEvent(payload);
    case "server_patches_available":
      return serverPatchesAvailableEvent(payload);
    case "server_patch_check":
      return serverPatchCheckEvent(payload);
    case "server_reachable":
      return serverReachableEvent(payload);
    case "server_unreachable":
      return serverUnreachableEvent(payload);
    case "high_disk_usage":
      return highDiskUsageEvent(payload);
    case "deployment_success":
      return deploymentSuccessEvent(payload);
    case "deployment_failed":
      return deploymentFailedEvent(payload);
    case "container_stopped":
      return containerStoppedEvent(payload);
    case "container_restarted":
      return containerRestartedEvent(payload);
    case "status_changed":
      return statusChangedEvent(payload);
    case "traefik_version_outdated":
      return traefikVersionOutdatedEvent(payload);
    case "task_success":
      return taskSuccessEvent(payload);
    case "task_failed":
      return taskFailedEvent(payload);
    case "backup_success_with_s3_warning":
      return backupSuccessWithS3WarningEvent(payload);
    case "test":
      return testEvent();
    default:
      return {
        title: "Event: " + event,
        body: payload.message,
      };
  }
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
