use url::Url;

/// Parses a URL string and extracts the normalized host component.
/// 
/// This function:
/// - Normalizes hostnames to lowercase
/// - Removes any trailing dots
/// - Decodes percent-encoded characters in the host
/// - Removes user info and port numbers
/// - Excludes IP addresses (both IPv4 and IPv6)
/// 
/// # Arguments
/// 
/// * `url_str` - A string slice containing the URL to parse
/// 
/// # Returns
/// 
/// * `Ok(String)` - The normalized host component if successful
/// * `Err(String)` - An error message if parsing fails
pub fn parse_url(url_str: &str) -> Result<String, String> {
    // Try to parse the URL with a scheme
    let url_result = Url::parse(url_str);
    
    // If parsing fails, try adding a default scheme and parse again
    let url = match url_result {
        Ok(url) => url,
        Err(_) => {
            // Check if it might be a URL without a scheme
            if url_str.contains('.') && !url_str.contains(' ') {
                match Url::parse(&format!("http://{}", url_str)) {
                    Ok(url) => url,
                    Err(e) => return Err(format!("Invalid URL: {}", e)),
                }
            } else {
                return Err("Invalid URL format".to_string());
            }
        }
    };
    
    // Check if the URL has an IP address as host
    if url.has_host() {
        if let Some(host) = url.host() {
            match host {
                url::Host::Ipv4(_) => return Err("IPv4 addresses are not valid domains".to_string()),
                url::Host::Ipv6(_) => return Err("IPv6 addresses are not valid domains".to_string()),
                url::Host::Domain(domain) => {
                    // Normalize the host: lowercase and remove trailing dots
                    let mut normalized_host = domain.to_lowercase();
                    if normalized_host.ends_with('.') {
                        normalized_host.pop();
                    }
                    return Ok(normalized_host);
                }
            }
        }
    }
    
    Err("URL has no valid host component".to_string())
} 