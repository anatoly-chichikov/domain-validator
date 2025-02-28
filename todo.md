# Domain Parser Requirements Checklist

> **Note:** **The implementation MUST be done in Rust.**

---

## Step 1: Core URL Parsing and Normalization

### Functional Requirements
- [ ] **URL Parsing:** 
  - Use a standard URL parsing library (e.g., `url`) to extract the host component from a URL.
- [ ] **Normalization:** 
  - Normalize hostnames to lowercase.
  - Remove any trailing dots.
  - Decode percent-encoded characters in the host.
- [ ] **User Info & Port Stripping:** 
  - Remove any user info (e.g., `user:pass@`) and port numbers from the host.
- [ ] **Exclusion of IPs:** 
  - Detect and exclude IP addresses (both IPv4 and IPv6) from domain parsing.

### Test Requirements (~15 URL Parsing Tests)
- [ ] Validate parsing of standard URLs (e.g., `https://www.example.com/path`).
- [ ] Verify proper handling of URLs containing user info (e.g., `http://user:pass@www.example.com`).
- [ ] Check correct parsing when ports are present (e.g., `http://www.example.com:8080`).
- [ ] Ensure that invalid URL formats are handled gracefully.
- [ ] Test URLs with multiple subdomains.
- [ ] Validate URLs with missing schemes or ambiguous formats.
- [ ] Check parsing for URLs with query parameters and fragments.
- [ ] Test cases with percent-encoded characters in the host.
- [ ] Verify behavior with trailing dots in the host.
- [ ] Validate handling of uppercase and mixed-case URLs.
- [ ] Confirm proper extraction when extra slashes or redundant separators are present.
- [ ] Test URLs with IPv6 literal notation (ensuring they are excluded as required).
- [ ] Validate URLs with unusual but valid characters.
- [ ] Confirm that URL parsing libraries correctly separate the path from the host.
- [ ] Test URLs that are borderline invalid to verify robust error handling.

---

## Step 2: Domain Extraction using PSL and IDN Handling

### Functional Requirements
- [ ] **Public Suffix List (PSL) Integration:**
  - Integrate a Rust crate (e.g., `publicsuffix`) to use the PSL for determining the effective top-level domain plus one (eTLD+1).
  - Ensure extraction of root domains for both simple and compound TLDs.
- [ ] **IDN Handling:**
  - Use an IDN library (e.g., `idna`) to support conversion from Unicode to punycode.
  - Normalize and extract root domains from internationalized domain names.
- [ ] **Flexibility:**
  - Allow the implementation to choose the most suitable Rust methods for PSL and IDN conversion.

### Test Requirements

#### PSL Tests (~12 tests)
- [ ] Test extraction of root domains for simple TLDs (e.g., `example.com`).
- [ ] Test extraction for compound TLDs (e.g., `example.co.uk`, `example.blogspot.com.au`).
- [ ] Verify correct handling of PSL exceptions (e.g., cases like `!parliament.uk`).
- [ ] Validate that the longest matching PSL rule is applied.
- [ ] Confirm behavior with domains having multiple potential PSL matches.
- [ ] Test with domains using new or less common TLDs.
- [ ] Validate extraction for domains with non-standard but valid PSL entries.
- [ ] Test domains where the PSL data is updated dynamically.
- [ ] Confirm correct behavior for domains not present in the PSL (fallback logic).
- [ ] Validate performance for bulk PSL lookups.
- [ ] Test domain extraction with domains that have hyphenated labels.
- [ ] Ensure that the system gracefully handles malformed PSL entries.

#### IDN Tests (~10 tests)
- [ ] Validate conversion from Unicode to punycode for domains (e.g., `münchen.de` → `xn--mnchen-3ya.de`).
- [ ] Test extraction of the root domain for Unicode domains.
- [ ] Verify that mixed-case Unicode domains are normalized correctly.
- [ ] Check conversion accuracy for domains with non-Latin characters (e.g., `東京.jp`).
- [ ] Test handling of IDNs with accented characters.
- [ ] Verify correct round-trip conversion (Unicode to punycode and back).
- [ ] Test handling of domains with multiple Unicode characters.
- [ ] Validate proper extraction when percent-encoding is mixed with Unicode.
- [ ] Ensure that invalid Unicode domains are handled gracefully.
- [ ] Confirm that IDN tests work correctly with the PSL lookup process.

---

## Step 3: Web Service API and End-to-End Functionality

### Functional Requirements
- [ ] **Local Web Service:**
  - Develop a local web service in Rust that binds to localhost.
- [ ] **API Endpoint:**
  - Expose an HTTP API endpoint (e.g., `/parse?url={url}`) to perform domain parsing.
- [ ] **JSON Response:**
  - Return JSON responses that include:
    - Original URL.
    - Extracted root domain.
    - Error details (if applicable).
- [ ] **Curl Compatibility:**
  - Ensure the service can be called via `curl` (command-line HTTP requests).
- [ ] **Minimal Infrastructure:**
  - No extra infrastructure is needed beyond the local service.

### Test Requirements (~10 API Endpoint Tests)
- [ ] Confirm that the web service endpoint is accessible locally.
- [ ] Validate that valid requests (via `curl`) return the correct JSON output.
- [ ] Verify that error responses are meaningful for invalid input.
- [ ] Ensure that headers and HTTP methods follow expected conventions.
- [ ] Test behavior when the API receives unexpected HTTP methods.
- [ ] Validate proper handling of URL-encoded parameters in the API.
- [ ] Confirm that the API returns correct HTTP status codes for various scenarios.
- [ ] Test performance of the endpoint under repeated calls.
- [ ] Verify that the API handles concurrent requests appropriately.
- [ ] Validate that the API logs incoming requests and errors for audit purposes.

---

## Implementation Requirements (General)
- [ ] **Rust Implementation:** The entire solution must be implemented in Rust.
- [ ] **Library Usage:** Leverage existing Rust crates such as:
  - `url` for URL parsing.
  - `publicsuffix` for Public Suffix List handling.
  - `idna` for Internationalized Domain Name conversion.
- [ ] **Documentation Search:** Use console search tools or Rust documentation search engines to identify necessary methods and libraries.
- [ ] **Robust Handling:** Ensure the service correctly processes both typical and edge-case URLs.
- [ ] **Error Handling:** Implement robust logging and error handling to diagnose parsing issues.
- [ ] **Code Documentation:** Document code and API usage clearly, following Rust best practices.

---

## Documentation & Repository Requirements
- [ ] Include this Markdown requirements checklist in the repository.
- [ ] Provide clear instructions on how to run the web service locally.
- [ ] Include detailed instructions for executing tests.
- [ ] Supply examples of valid and invalid URLs for testing purposes.
- [ ] Ensure that the code is well-commented and adheres to Rust coding standards.