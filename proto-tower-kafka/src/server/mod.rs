pub mod layer;

#[derive(Clone, Debug)]
pub struct KafkaProtoServerConfig {
    pub timeout: std::time::Duration,
}
