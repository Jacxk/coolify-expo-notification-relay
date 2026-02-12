use regex::Regex;
use std::env;

pub fn parse_expo_push_tokens() -> Result<Vec<String>, String> {
    let Ok(expo_push_tokens) = env::var("EXPO_PUSH_TOKENS") else {
        return Err("EXPO_PUSH_TOKENS is not set.".to_string());
    };

    let tokens = expo_push_tokens
        .split(',')
        .map(str::to_string)
        .collect::<Vec<String>>();
    let re = Regex::new(r"ExponentPushToken\[(?<token>[^\]]+)\]");

    let Ok(re) = &re else {
        return Err("Failed to create regex for Expo push tokens validation.".to_string());
    };

    let valid_tokens = tokens
        .iter()
        .filter(|token| re.is_match(token))
        .map(|token| token.to_string())
        .collect::<Vec<String>>();

    if valid_tokens.is_empty() {
        return Err("No valid Expo push tokens found. Please check your EXPO_PUSH_TOKENS environment variable.".to_string());
    }

    Ok(valid_tokens)
}
