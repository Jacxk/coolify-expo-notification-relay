use coolify_expo_notification_relay::UpdaterService;

#[tokio::test]
async fn test_update_available() {
    let mut updater = UpdaterService::default();
    updater.current_version = "0.0.0";

    let update = updater.check_for_updates().await;

    match update {
        Ok(Some(release)) => {
            assert!(release.tag_name != updater.current_version, "release tag name should not be the same as the current version");
        }
        Ok(None) => {
            panic!("release is not available");
        }
        Err(error) => {
            panic!("Failed to check for updates: {}", error.message);
        }
    }
}
