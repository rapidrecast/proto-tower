# Proto Tower

A collection of protocol implementations using tower.

The repository is a collection of crates that implement various protocols from wire (`(AsyncRead, AsyncWrite)`) and translate the protocol to easy to reason about requests and responses.

Because the implementation is both tower based and standardised, they can be combined.
This means you can serve multiple protocols from the same TCP or UDP binding.

The protocol implementations are not tied to either TCP or UDP, so you are free to adapt them as you need (ex. gRPC over HTTP/3 or QUIC).
You do need to provide consistent readers and writers for "sessions".
That means a single TCP connection, or a single UDP source+destination port pair.

Feature Flags 

| Protocol | Feature Flag | Description |
|----------|--------------|-------------|
| DTLS | `dtls-data` | DTLS data structures |
| DTLS | `dtls-client` | DTLS client code |
| DTLS | `dtls-server` | DTLS server code |
| gRPC | `grpc-data` | gRPC data structures |
| gRPC | `grpc-client` | gRPC client code |
| gRPC | `grpc-server` | gRPC server code |
| HTTP/1.1 | `http-1-data` | HTTP/1.1 data structures |
| HTTP/1.1 | `http-1-client` | HTTP/1.1 client code |
| HTTP/1.1 | `http-1-server` | HTTP/1.1 server code |
| HTTP/2 | `http-2-data` | HTTP/2 data structures |
| HTTP/2 | `http-2-client` | HTTP/2 client code |
| HTTP/2 | `http-2-server` | HTTP/2 server code |
| HTTP/3 | `http-3-data` | HTTP/3 data structures |
| HTTP/3 | `http-3-client` | HTTP/3 client code |
| HTTP/3 | `http-3-server` | HTTP/3 server code |
| ICE | `ice-data` | ICE data structures |
| ICE | `ice-client` | ICE client code |
| ICE | `ice-server` | ICE server code |
| Kafka | `kafka-data` | Kafka data structures |
| Kafka | `kafka-client` | Kafka client code |
| Kafka | `kafka-server` | Kafka server code |
| MQTT | `mqtt-data` | MQTT data structures |
| MQTT | `mqtt-client` | MQTT client code |
| MQTT | `mqtt-server` | MQTT server code |
| QUIC | `quic-data` | QUIC data structures |
| QUIC | `quic-client` | QUIC client code |
| QUIC | `quic-server` | QUIC server code |
| STUN | `stun-data` | STUN data structures |
| STUN | `stun-client` | STUN client code |
| STUN | `stun-server` | STUN server code |
| TLS | `tls-data` | TLS data structures |
| TLS | `tls-client` | TLS client code |
| TLS | `tls-server` | TLS server code |
| TURN | `turn-data` | TURN data structures |
| TURN | `turn-client` | TURN client code |
| TURN | `turn-server` | TURN server code |
| WebRTC | `webrtc-data` | WebRTC data structures |
| WebRTC | `webrtc-client` | WebRTC client code |
| WebRTC | `webrtc-server` | WebRTC server code |




