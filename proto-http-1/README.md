# Proto-http-1

An implementation of HTTP/1.1 using `tokio-io` and `tower`.

## Adding dependency to project

```toml
[dependencies]
proto-http-1 = { git = "https://github.com/rapidrecast/proto-tower.git", subdir = "proto-http-1" }
```

## Feature Completion

### Server

| Feature                      | Status                                                       |
|------------------------------|--------------------------------------------------------------|
| Basic requests and responses | Done                                                         |
| Protocol upgrades            | Yes, for `connection: upgrade` such as HTTP/2 and WebSockets |
| Chunked encoding             | Not started                                                  |
| Keep-alive                   | Not started                                                  |
| Efficient packet reading     | Not started                                                  |
| Multipart requests           | Not started                                                  | 

### Client

| Feature                      | Status      |
|------------------------------|-------------|
| Basic requests and responses | Started     |
| Protocol upgrades            | Not started |
| Chunked encoding             | Not started |
| Keep-alive                   | Not started |
| Efficient packet reading     | Not started |
| Multipart requests           | Not started | 

### Data structures

```mermaid
graph TD
%% client declarations
    CML[ProtoHttp1ClientMakeLayer]:::green1
    CL[ProtoHttp1ClientLayer]:::green2
    CINNER["Client inner Service< (Reader,Writer)>"]:::pink1
%% server declarations
    SML[ProtoHttp1ServerMakeLayer]:::red1
    SL[ProtoHttp1ServerLayer]:::red2
    SINNER["Server inner Service< Http1Event>"]:::pink1
%% types
    ARW["(AsyncRead, AsyncWriter)"]:::structure
    E[Http1Event]:::structure
    R[Http1Request]:::structure
    SE[Http1ServerEvent]:::structure
    CR[Http1ClientResponse]:::structure
    Nothing["()"]:::structure
%% relationships
    CML -->|Provides| CL
    CL -->|Returns| CR
    R -->|Inputs to| CL
    SML -->|Provides| SL
    ARW -->|Inputs to| SL
    SL -->|Outputs to| E
    CL -->|Outputs to| ARW
    E -->|Depends on| R
    E -->|Inputs to| SINNER
    SINNER -->|Returns| SE
    SE -->|Inputs to| SL
    ARW -->|Inputs to| CINNER
    CINNER -->|Returns| Nothing
%% notes
    N1["This is the network primitive"]:::note -->|Note for| ARW
    N2["This is the service that would be implemented for the server"]:::note -->|Note for| SINNER
%% style
    classDef green1 fill: #00aa00, stroke: #333, stroke-width: 2px, color: #ffffff;
    classDef green2 fill: #33aa33, stroke: #333, stroke-width: 2px, color: #ffffff;
    classDef green3 fill: #55aa55, stroke: #333, stroke-width: 2px, color: #ffffff;
    classDef red1 fill: #aa0000, stroke: #333, stroke-width: 2px, color: #ffffff;
    classDef red2 fill: #aa3333, stroke: #333, stroke-width: 2px, color: #ffffff;
    classDef red3 fill: #aa5555, stroke: #333, stroke-width: 2px, color: #ffffff;
    classDef blue1 fill: #0000aa, stroke: #333, stroke-width: 2px, color: #ffffff;
    classDef blue2 fill: #3333aa, stroke: #333, stroke-width: 2px, color: #ffffff;
    classDef blue3 fill: #5555aa, stroke: #333, stroke-width: 2px, color: #ffffff;
    classDef pink1 fill: #aa00aa, stroke: #333, stroke-width: 2px, color: #ffffff;
    classDef pink2 fill: #aa33aa, stroke: #333, stroke-width: 2px, color: #ffffff;
    classDef pink3 fill: #aa55aa, stroke: #333, stroke-width: 2px, color: #ffffff;
    classDef grey fill: #ddd, stroke: #333, stroke-width: 2px;
    classDef structure fill: #0000aa, stroke: #333, stroke-width: 2px;
    classDef note fill: #ffeb3b, stroke: #ff9800, stroke-width: 2px, color: #000, font-style: italic, font-size: 12px;
```
