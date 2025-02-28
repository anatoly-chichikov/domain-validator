use domain_validator::api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Domain Validator Web Service...");
    println!("Server will be available at http://localhost:3000");
    println!("Use the endpoint: http://localhost:3000/parse?url=<your-url>");
    println!("Press Ctrl+C to stop the server");
    
    api::start_service().await
}
