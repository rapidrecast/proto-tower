# Proto-http-1

An implementation of HTTP/1.1 using `tokio-io` and `tower`.

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
| Basic requests and responses | Not started |
| Protocol upgrades            | Not started |
| Chunked encoding             | Not started |
| Keep-alive                   | Not started |
| Efficient packet reading     | Not started |
| Multipart requests           | Not started | 
