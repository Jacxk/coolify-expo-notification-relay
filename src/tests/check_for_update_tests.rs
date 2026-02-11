use coolify_expo_notification_relay::utils::check_for_updates;

#[tokio::test]
async fn test_update_available() {
    let fake_current_version = "0.0.0";
    let update = check_for_updates(fake_current_version.to_string()).await;

    assert!(update.is_ok(), "Failed to check for updates");

    let (update_available, release) = update.unwrap();

    assert!(update_available, "Update is not available");
    assert!(release.is_some(), "Release is not available");
}
