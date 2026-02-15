use coolify_expo_notification_relay::UpdaterService;

#[tokio::test]
async fn test_update_available() {
    let version = "0.0.0";
    let mut updater = UpdaterService::default();
    updater.set_current_version(version);

    let update = updater.check_for_updates().await;

    match update {
        Ok(Some(release)) => {
            assert_ne!(
                release.tag_name, updater.get_current_version(),
                "release tag name should not be the same as the current version"
            );
        }
        Ok(None) => {
            panic!("release is not available");
        }
        Err(error) => {
            panic!("Failed to check for updates: {}", error.message);
        }
    }
}
