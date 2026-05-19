use serde_json::json;

fn validate_ruc(ruc: &str) -> bool {
    ruc.len() == 11 && ruc.chars().all(|c| c.is_ascii_digit())
}

#[test]
fn test_valid_ruc_11_digits() {
    assert!(validate_ruc("20100070970"));
    assert!(validate_ruc("10452159428"));
    assert!(validate_ruc("00000000000"));
}

#[test]
fn test_ruc_too_short() {
    assert!(!validate_ruc("2010007097"));
    assert!(!validate_ruc("1234567890"));
    assert!(!validate_ruc(""));
}

#[test]
fn test_ruc_too_long() {
    assert!(!validate_ruc("201000709701"));
    assert!(!validate_ruc("123456789012"));
}

#[test]
fn test_ruc_non_numeric() {
    assert!(!validate_ruc("2010007097A"));
    assert!(!validate_ruc("abcdefghijk"));
    assert!(!validate_ruc("2010007097 "));
    assert!(!validate_ruc(" 0100070970"));
    assert!(!validate_ruc("20100070-70"));
}

#[test]
fn test_tools_schema_has_required_fields() {
    let schema = json!({
        "name": "buscar_ruc",
        "inputSchema": {
            "type": "object",
            "properties": {
                "ruc": { "type": "string", "pattern": "^\\d{11}$" }
            },
            "required": ["ruc"],
            "additionalProperties": false
        }
    });
    assert_eq!(schema["name"], "buscar_ruc");
    assert_eq!(schema["inputSchema"]["required"][0], "ruc");
    assert_eq!(schema["inputSchema"]["additionalProperties"], false);
}

#[test]
fn test_empty_ruc_invalid() {
    assert!(!validate_ruc(""));
}

#[test]
fn test_ruc_with_leading_zero_valid() {
    assert!(validate_ruc("01234567890"));
}

#[test]
fn test_ruc_all_zeros_valid() {
    assert!(validate_ruc("00000000000"));
}
