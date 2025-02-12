pub mod layer;
pub mod make_layer;
#[cfg(test)]
mod test;

#[derive(Clone, Debug)]
pub struct KafkaProtoClientConfig {
    pub timeout: std::time::Duration,
    pub fail_on_inactivity: bool,
    pub client_id: Option<String>,
}
