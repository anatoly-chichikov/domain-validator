use domain_validator::domain_parser;

fn main() {
    let url = "https://www.example.com/path";
    match domain_parser::parse_url(url) {
        Ok(host) => println!("Parsed host: {}", host),
        Err(e) => println!("Error parsing URL: {}", e),
    }
}
