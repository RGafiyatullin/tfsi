use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct HttpBindConfig {
    pub bind_addr: IpAddr,
    pub bind_port: u16,
}

impl Default for HttpBindConfig {
    fn default() -> Self {
        Self {
            bind_addr: IpAddr::from([0, 0, 0, 0]),
            bind_port: 8080,
        }
    }
}
