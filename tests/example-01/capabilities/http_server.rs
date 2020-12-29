use crate::config::HttpBindConfig;

#[derive(Debug)]
pub struct HttpServer {
    config: HttpBindConfig,
}

impl HttpServer {
    pub fn create(config: &HttpBindConfig) -> Self {
        Self {
            config: config.to_owned(),
        }
    }
}
