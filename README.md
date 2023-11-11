# Envoy Request Header Mutator

## Overview

This Rust-based project implements a custom filter for Envoy proxy using the `proxy_wasm` framework. It is designed to mutate HTTP request headers based on predefined configurations. Supported operations include adding, modifying, replacing, and removing headers.

## Features

- **Add Headers**: Add new headers to the HTTP request.
- **Modify Headers**: Modify existing headers based on regex patterns.
- **Replace Headers**: Replace the value of existing headers.
- **Remove Headers**: Remove specified headers from the request.

## Dependencies

- `proxy_wasm`: Framework for building Envoy WASM filters.
- `serde`: Serialization and deserialization framework.
- `regex`: Regular expression library for header value manipulation.

## Usage

1. **Configuration**: Define your header mutation rules in the configuration. The configuration should be in JSON format, defining the type of operation (`Add`, `Modify`, `Replace`, `Remove`) and the relevant parameters.

2. **Build**: Compile the project to generate a WASM binary.

3. **Deployment**: Deploy the compiled WASM binary as an Envoy filter.

## Example Configuration

```json
{
  "type": "Modify",
  "header": "example-header",
  "regex": "original-value",
  "replace_with": "new-value"
}
```

This example configuration will modify example-header in the HTTP request, replacing original-value with new-value.
Logging

The filter uses proxy_wasm::set_log_level(LogLevel::Trace) for detailed logging, which can be adjusted as needed.
Contributing

Contributions to this project are welcome. Please submit issues and pull requests via GitHub.

## License
