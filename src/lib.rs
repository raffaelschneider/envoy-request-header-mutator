use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use serde::{Deserialize};
use regex::Regex;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(CustomFilterRoot {
            config_stringvalue: String::new(),
        })
    });
}}

struct CustomFilter {
    config_stringvalue: String,
}

impl Context for CustomFilter {}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum Operation {
    Add { header: String, value: String},
    Modify { header: String, regex: String, replace_with: String},
    Replace { header: String, replace_with: String },
    Remove { header: String },
}

impl HttpContext for CustomFilter {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        match serde_json::from_str(self.config_stringvalue.as_str()) {
            Err(e) => log::error!("Config deserialization failed. Reason: {}", e),
            Ok(operation) =>
                match &operation {
                    Operation::Add { header, value } => {
                        self.add_http_request_header(header, value);
                        log::info!("Add succeeded. Added header: {}", header);
                    }
                    Operation::Modify { header, regex, replace_with} => {
                        if let Some(header_field) = self.get_http_request_header(header) {
                            let replaced_header_field = Regex::new(regex).unwrap().replace_all(&header_field, replace_with);
                            self.set_http_request_header(header, Some(&replaced_header_field));
                            log::info!("Modify succeeded. Modified header: {}", header);
                        }
                    }
                    Operation::Replace { header, replace_with} => {
                        if let Some(_header_field) = self.get_http_request_header(header) {
                            self.set_http_request_header(header, Some(replace_with));
                            log::info!("Replace succeeded. Replaced header: {}", header);
                        }
                    }
                    Operation::Remove { header } => {
                        if let Some(_header_field) = self.get_http_request_header(header) {
                            self.set_http_request_header(header, None);
                            log::info!("Remove succeeded. Removed header: {}", header);
                        }
                    }
                }
        };
        Action::Continue
    }
}

struct CustomFilterRoot {
    config_stringvalue: String,
}

impl Context for CustomFilterRoot {}

impl RootContext for CustomFilterRoot {
    fn on_configure(&mut self, _: usize) -> bool {
        if let Some(config_bytes) = self.get_plugin_configuration() {
            self.config_stringvalue = String::from_utf8(config_bytes).unwrap()
        }
        true
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(CustomFilter {
            config_stringvalue: self.config_stringvalue.clone(),
        }))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}