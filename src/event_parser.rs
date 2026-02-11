use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct WebhookPayload {
    pub title: Option<String>,
    pub event: Option<String>,
    pub message: Option<String>,

    pub server_name: Option<String>,
    pub database_name: Option<String>,
    pub total_updates: Option<u64>,
    pub disk_usage: Option<f64>,
    pub threshold: Option<f64>,
    pub preview_fqdn: Option<String>,
    pub application_name: Option<String>,
    pub project: Option<String>,
    pub container_name: Option<String>,
    pub affected_servers_count: Option<u64>,
    pub task_name: Option<String>,
}

/// Parsed notification to send to Expo (title + body).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Notification {
    pub title: String,
    pub body: String,
}

pub fn parse_event(payload: &WebhookPayload) -> Notification {
    let event = payload.event.as_deref().unwrap_or("unknown");
    match event {
        // docker
        "docker_cleanup_success" => docker_cleanup_success(payload),
        "docker_cleanup_failed" => docker_cleanup_failed(payload),
        // database
        "backup_success" => backup_success(payload),
        "backup_failed" => backup_failed(payload),
        "backup_success_with_s3_warning" => backup_success_with_s3_warning(payload),
        // server
        "server_patches_available" => server_patches_available(payload),
        "server_patch_check" => server_patch_check(payload),
        "server_patch_check_error" => server_patch_check_error(payload),
        "server_reachable" => server_reachable(payload),
        "server_unreachable" => server_unreachable(payload),
        "high_disk_usage" => high_disk_usage(payload),
        // deployment
        "deployment_success" => deployment_success(payload),
        "deployment_failed" => deployment_failed(payload),
        // container
        "container_stopped" => container_stopped(payload),
        "container_restarted" => container_restarted(payload),
        "status_changed" => status_changed(payload),
        // traefik
        "traefik_version_outdated" => traefik_version_outdated(payload),
        // task
        "task_success" => task_success(payload),
        "task_failed" => task_failed(payload),
        // test
        "test" => test_event(),
        _ => Notification {
            title: format!("Event: {}", event),
            body: payload
                .message
                .clone()
                .unwrap_or_else(|| "No message".to_string()),
        },
    }
}

// ---------------------------------------------------------------------------
// Docker
// ---------------------------------------------------------------------------

fn docker_cleanup_success(payload: &WebhookPayload) -> Notification {
    let server = payload.server_name.as_deref().unwrap_or("unknown");
    Notification {
        title: "Docker Cleanup Success".to_string(),
        body: format!("Docker cleanup job succeeded on server {}", server),
    }
}

fn docker_cleanup_failed(payload: &WebhookPayload) -> Notification {
    let server = payload.server_name.as_deref().unwrap_or("unknown");
    Notification {
        title: "Docker Cleanup Failed".to_string(),
        body: format!("Docker cleanup job failed on server {}", server),
    }
}

// ---------------------------------------------------------------------------
// Database
// ---------------------------------------------------------------------------

fn backup_success(payload: &WebhookPayload) -> Notification {
    let db = payload.database_name.as_deref().unwrap_or("unknown");
    Notification {
        title: "Database Backup Success".to_string(),
        body: format!("Database backup job succeeded on database {}", db),
    }
}

fn backup_failed(payload: &WebhookPayload) -> Notification {
    let db = payload.database_name.as_deref().unwrap_or("unknown");
    Notification {
        title: "Database Backup Failed".to_string(),
        body: format!("Database backup job failed on database {}", db),
    }
}

fn backup_success_with_s3_warning(payload: &WebhookPayload) -> Notification {
    let db = payload.database_name.as_deref().unwrap_or("unknown");
    Notification {
        title: "Local Backup Success, S3 Backup Failed".to_string(),
        body: format!(
            "Local backup of {} was successful, but S3 backup failed",
            db
        ),
    }
}

// ---------------------------------------------------------------------------
// Server
// ---------------------------------------------------------------------------

fn server_patches_available(payload: &WebhookPayload) -> Notification {
    let updates = payload.total_updates.unwrap_or(0);
    let server = payload.server_name.as_deref().unwrap_or("unknown");
    Notification {
        title: "Server Patches Available".to_string(),
        body: format!("{} patches are available for server {}", updates, server),
    }
}

fn server_patch_check(payload: &WebhookPayload) -> Notification {
    let updates = payload.total_updates.unwrap_or(0);
    let server = payload.server_name.as_deref().unwrap_or("unknown");
    Notification {
        title: "Server Patches Available".to_string(),
        body: format!("{} patches are available for server {}", updates, server),
    }
}

fn server_patch_check_error(payload: &WebhookPayload) -> Notification {
    let server = payload.server_name.as_deref().unwrap_or("unknown");
    Notification {
        title: "Failed to Check for Patches".to_string(),
        body: format!("Failed to check for patches on server {}", server),
    }
}

fn server_reachable(payload: &WebhookPayload) -> Notification {
    let server = payload.server_name.as_deref().unwrap_or("unknown");
    Notification {
        title: "Server Revived".to_string(),
        body: format!("Server {} is back online", server),
    }
}

fn server_unreachable(payload: &WebhookPayload) -> Notification {
    let server = payload.server_name.as_deref().unwrap_or("unknown");
    Notification {
        title: "Server Unreachable".to_string(),
        body: format!("Server {} is unreachable", server),
    }
}

fn high_disk_usage(payload: &WebhookPayload) -> Notification {
    let server = payload.server_name.as_deref().unwrap_or("unknown");
    let usage = payload.disk_usage.unwrap_or(0.0);
    let threshold = payload.threshold.unwrap_or(0.0);
    Notification {
        title: "High Disk Usage Detected".to_string(),
        body: format!(
            "Server {} is using {}% of its disk space, which is above the threshold of {}%",
            server, usage, threshold
        ),
    }
}

// ---------------------------------------------------------------------------
// Deployment
// ---------------------------------------------------------------------------

fn deployment_success(payload: &WebhookPayload) -> Notification {
    let title = if payload.preview_fqdn.is_some() {
        "Preview Deployment Success"
    } else {
        "Deployment Success"
    };
    let app = payload.application_name.as_deref().unwrap_or("application");
    let project = payload.project.as_deref().unwrap_or("project");
    Notification {
        title: title.to_string(),
        body: format!("{} was deployed successfully for {}", app, project),
    }
}

fn deployment_failed(payload: &WebhookPayload) -> Notification {
    let title = if payload.preview_fqdn.is_some() {
        "Preview Deployment Failed"
    } else {
        "Deployment Failed"
    };
    let app = payload.application_name.as_deref().unwrap_or("application");
    let project = payload.project.as_deref().unwrap_or("project");
    Notification {
        title: title.to_string(),
        body: format!("Deployment of {} for {} failed", app, project),
    }
}

// ---------------------------------------------------------------------------
// Container
// ---------------------------------------------------------------------------

fn container_stopped(payload: &WebhookPayload) -> Notification {
    let container = payload.container_name.as_deref().unwrap_or("resource");
    let server = payload.server_name.as_deref().unwrap_or("unknown");
    Notification {
        title: "Resource Stopped Unexpectedly".to_string(),
        body: format!(
            "Resource {} was stopped unexpectedly on server {}",
            container, server
        ),
    }
}

fn container_restarted(payload: &WebhookPayload) -> Notification {
    let container = payload.container_name.as_deref().unwrap_or("resource");
    let server = payload.server_name.as_deref().unwrap_or("unknown");
    Notification {
        title: "Resource Restarted Automatically".to_string(),
        body: format!(
            "Resource {} was restarted automatically on server {}",
            container, server
        ),
    }
}

fn status_changed(payload: &WebhookPayload) -> Notification {
    if payload
        .title
        .as_deref()
        .map(|t| t.to_lowercase() == "application stopped")
        .unwrap_or(false)
    {
        let app = payload.application_name.as_deref().unwrap_or("application");
        return Notification {
            title: "Application Stopped".to_string(),
            body: format!("Application {} has been stopped", app),
        };
    }
    // Fallback: same as unknown event (eventParser in JS returns default when handler returns undefined)
    Notification {
        title: "Event: status_changed".to_string(),
        body: payload
            .message
            .clone()
            .unwrap_or_else(|| "No message".to_string()),
    }
}

// ---------------------------------------------------------------------------
// Traefik
// ---------------------------------------------------------------------------

fn traefik_version_outdated(payload: &WebhookPayload) -> Notification {
    let count = payload.affected_servers_count.unwrap_or(0);
    Notification {
        title: "Traefik Version Outdated".to_string(),
        body: format!("Traefik version for {} servers is outdated", count),
    }
}

// ---------------------------------------------------------------------------
// Task
// ---------------------------------------------------------------------------

fn task_success(payload: &WebhookPayload) -> Notification {
    let task = payload.task_name.as_deref().unwrap_or("task");
    Notification {
        title: "Scheduled Task Success".to_string(),
        body: format!("Scheduled task {} was successful", task),
    }
}

fn task_failed(payload: &WebhookPayload) -> Notification {
    let task = payload.task_name.as_deref().unwrap_or("task");
    Notification {
        title: "Scheduled Task Failed".to_string(),
        body: format!("Scheduled task {} failed", task),
    }
}

// ---------------------------------------------------------------------------
// Test
// ---------------------------------------------------------------------------

fn test_event() -> Notification {
    Notification {
        title: "Coolify Test Event".to_string(),
        body: "Test event received".to_string(),
    }
}
