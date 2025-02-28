use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

use crate::domain_parser;

/// Request parameters for the domain parsing endpoint
#[derive(Debug, Deserialize)]
pub struct ParseParams {
    url: String,
}

/// Response structure for the domain parsing endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct ParseResponse {
    pub original_url: String,
    pub root_domain: Option<String>,
    pub error: Option<String>,
}

/// Handler for the domain parsing endpoint
async fn parse_domain(Query(params): Query<ParseParams>) -> impl IntoResponse {
    info!("Received request to parse URL: {}", params.url);
    
    let mut response = ParseResponse {
        original_url: params.url.clone(),
        root_domain: None,
        error: None,
    };
    
    match domain_parser::extract_root_domain_from_url(&params.url) {
        Ok(domain) => {
            response.root_domain = Some(domain);
        },
        Err(e) => {
            response.error = Some(e);
        }
    }
    
    (StatusCode::OK, Json(response))
}

/// Handler for the root endpoint
async fn root() -> &'static str {
    "Domain Validator API\n\nUsage: GET /parse?url=<url>"
}

/// Create and configure the API router
pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/parse", get(parse_domain))
}

/// Start the web service
pub async fn start_service() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    // Create the router
    let app = create_router();
    
    // Define the address to bind to
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting server on {}", addr);
    
    // Create a TCP listener
    let listener = TcpListener::bind(addr).await?;
    
    // Start the server
    axum::serve(listener, app).await?;
    
    Ok(())
} 