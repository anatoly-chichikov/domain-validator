use domain_validator::domain_parser;

// PSL Tests
#[test]
fn test_extract_root_domain_simple_tld() {
    let result = domain_parser::extract_root_domain("www.example.com");
    assert_eq!(result, Ok("example.com".to_string()));
}

#[test]
fn test_extract_root_domain_compound_tld() {
    let result = domain_parser::extract_root_domain("www.example.co.uk");
    assert_eq!(result, Ok("example.co.uk".to_string()));
}

#[test]
fn test_extract_root_domain_blogspot() {
    let result = domain_parser::extract_root_domain("myblog.blogspot.com");
    assert_eq!(result, Ok("myblog.blogspot.com".to_string()));
}

#[test]
fn test_extract_root_domain_with_subdomain() {
    let result = domain_parser::extract_root_domain("sub.domain.example.com");
    assert_eq!(result, Ok("example.com".to_string()));
}

#[test]
fn test_extract_root_domain_psl_exception() {
    // Test with a PSL exception if available
    let result = domain_parser::extract_root_domain("www.parliament.uk");
    assert_eq!(result, Ok("parliament.uk".to_string()));
}

#[test]
fn test_extract_root_domain_longest_matching_rule() {
    let result = domain_parser::extract_root_domain("test.github.io");
    assert_eq!(result, Ok("test.github.io".to_string()));
}

#[test]
fn test_extract_root_domain_with_hyphen() {
    let result = domain_parser::extract_root_domain("my-domain.example.com");
    assert_eq!(result, Ok("example.com".to_string()));
}

#[test]
fn test_extract_root_domain_invalid_domain() {
    let result = domain_parser::extract_root_domain("not-a-valid-domain");
    assert!(result.is_err());
}

// IDN Tests
#[test]
fn test_extract_root_domain_idn_unicode() {
    let result = domain_parser::extract_root_domain("www.münchen.de");
    assert_eq!(result, Ok("münchen.de".to_string()));
}

#[test]
fn test_extract_root_domain_idn_punycode() {
    let result = domain_parser::extract_root_domain("www.xn--mnchen-3ya.de");
    assert_eq!(result, Ok("xn--mnchen-3ya.de".to_string()));
}

#[test]
fn test_extract_root_domain_idn_mixed_case() {
    let result = domain_parser::extract_root_domain("www.MüNcHen.de");
    assert_eq!(result, Ok("münchen.de".to_string()));
}

#[test]
fn test_extract_root_domain_idn_non_latin() {
    let result = domain_parser::extract_root_domain("www.東京.jp");
    assert_eq!(result, Ok("東京.jp".to_string()));
}

#[test]
fn test_extract_root_domain_idn_with_accents() {
    let result = domain_parser::extract_root_domain("www.académie-française.fr");
    assert_eq!(result, Ok("académie-française.fr".to_string()));
}

#[test]
fn test_extract_root_domain_idn_invalid() {
    // Test with an invalid IDN
    let result = domain_parser::extract_root_domain("www.\u{FFFD}.com");
    assert!(result.is_err());
}

// Combined functionality tests
#[test]
fn test_extract_root_domain_from_url() {
    let result = domain_parser::extract_root_domain_from_url("https://www.example.co.uk/path?query=value");
    assert_eq!(result, Ok("example.co.uk".to_string()));
}

#[test]
fn test_extract_root_domain_from_idn_url() {
    let result = domain_parser::extract_root_domain_from_url("https://www.münchen.de/path");
    assert_eq!(result, Ok("münchen.de".to_string()));
}

#[test]
fn test_extract_root_domain_from_invalid_url() {
    let result = domain_parser::extract_root_domain_from_url("not a valid url");
    assert!(result.is_err());
} 