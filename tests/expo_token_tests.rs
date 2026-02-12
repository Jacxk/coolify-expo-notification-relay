use coolify_expo_notification_relay::utils::parse_expo_push_tokens;
use std::env;

#[test]
fn valid_expo_token_format_is_accepted() {
    let valid_token = "ExponentPushToken[abc123xyz]";
    unsafe {
        env::set_var("EXPO_PUSH_TOKENS", valid_token);
    }

    let result = parse_expo_push_tokens();

    unsafe {
        env::remove_var("EXPO_PUSH_TOKENS");
    }

    assert!(result.is_ok(), "valid Expo token should be accepted");

    let tokens = result.unwrap();

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], valid_token);
}

#[test]
fn invalid_expo_token_format_is_rejected() {
    let invalid_token = "ExponentPushToken abc123xyz";
    unsafe {
        env::set_var("EXPO_PUSH_TOKENS", invalid_token);
    }

    let result = parse_expo_push_tokens();

    unsafe {
        env::remove_var("EXPO_PUSH_TOKENS");
    }

    assert!(result.is_err(), "invalid Expo token should be rejected");
}
