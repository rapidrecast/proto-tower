mod layer;
mod make_layer;
mod parser;
#[cfg(test)]
mod test;

#[derive(Clone, Debug)]
pub struct ProtoHttp1ServerConfig {
    pub max_header_size: usize,
    pub max_body_size: usize,
    pub timeout: std::time::Duration,
}
