pub mod layer;
pub mod make_layer;
mod parser;
#[cfg(test)]
mod test;

#[derive(Clone, Debug)]
pub struct KafkaProtoServerConfig {
    pub timeout: std::time::Duration,
}
