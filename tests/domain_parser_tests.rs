use domain_validator::domain_parser;

#[test]
fn test_parse_standard_url() {
    let result = domain_parser::parse_url("https://www.example.com/path");
    assert_eq!(result, Ok("www.example.com".to_string()));
}

#[test]
fn test_parse_url_with_user_info() {
    let result = domain_parser::parse_url("http://user:pass@www.example.com");
    assert_eq!(result, Ok("www.example.com".to_string()));
}

#[test]
fn test_parse_url_with_port() {
    let result = domain_parser::parse_url("http://www.example.com:8080");
    assert_eq!(result, Ok("www.example.com".to_string()));
}

#[test]
fn test_parse_url_with_multiple_subdomains() {
    let result = domain_parser::parse_url("https://a.b.c.example.com/path");
    assert_eq!(result, Ok("a.b.c.example.com".to_string()));
}

#[test]
fn test_parse_url_with_trailing_dot() {
    let result = domain_parser::parse_url("https://www.example.com./path");
    assert_eq!(result, Ok("www.example.com".to_string()));
}

#[test]
fn test_parse_url_with_uppercase() {
    let result = domain_parser::parse_url("https://WWW.ExAmPlE.CoM/path");
    assert_eq!(result, Ok("www.example.com".to_string()));
}

#[test]
fn test_parse_url_with_percent_encoding() {
    let result = domain_parser::parse_url("https://www.ex%41mple.com/path");
    assert_eq!(result, Ok("www.example.com".to_string()));
}

#[test]
fn test_parse_url_with_ipv4() {
    let result = domain_parser::parse_url("https://192.168.1.1/path");
    assert!(result.is_err());
}

#[test]
fn test_parse_url_with_ipv6() {
    let result = domain_parser::parse_url("https://[2001:0db8:85a3:0000:0000:8a2e:0370:7334]/path");
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_url() {
    let result = domain_parser::parse_url("not a valid url");
    assert!(result.is_err());
}

#[test]
fn test_parse_url_missing_scheme() {
    let result = domain_parser::parse_url("www.example.com/path");
    assert_eq!(result, Ok("www.example.com".to_string()));
}

#[test]
fn test_parse_url_with_query_and_fragment() {
    let result = domain_parser::parse_url("https://www.example.com/path?query=value#fragment");
    assert_eq!(result, Ok("www.example.com".to_string()));
}

#[test]
fn test_parse_url_with_extra_slashes() {
    let result = domain_parser::parse_url("https:////www.example.com//path");
    assert_eq!(result, Ok("www.example.com".to_string()));
}

#[test]
fn test_parse_url_with_unusual_characters() {
    let result = domain_parser::parse_url("https://sub-domain.example-site.com/path");
    assert_eq!(result, Ok("sub-domain.example-site.com".to_string()));
} 