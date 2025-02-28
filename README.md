# Domain Validator

A Rust web service for parsing URLs and extracting their root domains, leveraging the Public Suffix List.

## Features

- **URL Parsing and Normalization**: Extracts and normalizes the host component from URLs.
- **Domain Extraction**: Determines the root domain (eTLD+1) using the Public Suffix List.
- **IDN Support**: Handles Internationalized Domain Names (Unicode domains).
- **Web API**: Simple HTTP endpoint to parse URLs and extract root domains.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021)
- Cargo (included with Rust)

### Setup

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd domain-validator
   ```

2. Build the project:
   ```bash
   cargo build
   ```

## Usage

### Running the Web Service

Start the service with:

```bash
cargo run
```

The service will be available at:
- URL: http://localhost:3000
- API Endpoint: http://localhost:3000/parse?url=<your-url>

### API Documentation

#### GET /parse

Parses a URL and returns the root domain.

**Query Parameters:**
- `url` (required): The URL to parse and extract the root domain from.

**Response Format:**
```json
{
  "original_url": "https://www.example.com/path",
  "root_domain": "example.com",
  "error": null
}
```

**Error Response:**
```json
{
  "original_url": "invalid-url",
  "root_domain": null,
  "error": "Invalid URL format"
}
```

### Examples

#### Using curl

```bash
curl "http://localhost:3000/parse?url=https://www.example.co.uk/path"
```

Response:
```json
{
  "original_url": "https://www.example.co.uk/path",
  "root_domain": "example.co.uk",
  "error": null
}
```

#### IDN Example

```bash
curl "http://localhost:3000/parse?url=https://www.münchen.de/path"
```

Response:
```json
{
  "original_url": "https://www.münchen.de/path",
  "root_domain": "münchen.de",
  "error": null
}
```

## Testing

Run the tests with:

```bash
cargo test
```

The project includes several test categories:
- Domain parser tests
- Domain extraction tests
- API endpoint tests

## Technical Details

### Dependencies

- `url`: URL parsing and normalization
- `publicsuffix`: Public Suffix List integration
- `idna`: Internationalized Domain Name handling
- `axum`: Web framework for the HTTP API
- `tokio`: Asynchronous runtime
- `serde`: Serialization/deserialization

### Project Structure

- `src/main.rs`: Application entry point
- `src/lib.rs`: Library exports
- `src/domain_parser.rs`: Core domain parsing logic
- `src/api.rs`: Web API implementation
- `tests/`: Test files for different components
- `public_suffix_list.dat`: Public Suffix List data file

## License

This project is licensed under the [MIT License](LICENSE). 