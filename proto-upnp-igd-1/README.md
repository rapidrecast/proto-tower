# Proto-UPnP-IGD-1

Universal Plug and Play (UPnP) Internet Gateway Device (IGD) is a protocol that allows a client to automatically
configure a router or gateway to allow inbound connections.
This is useful for applications that need to accept incoming connections from the Internet, such as peer-to-peer
applications.

It is a protocol that runs over HTTP/1.1 and uses SOAP.

https://upnp.org/specs/gw/UPnP-gw-WANIPConnection-v1-Service.pdf

https://openconnectivity.org/upnp-specs/UPnP-arch-DeviceArchitecture-v2.0-20200417.pdf


## Adding dependency to project

```toml
[dependencies]
proto-upnp-igd-1 = { git = "https://github.com/rapidrecast/proto-tower.git", subdir = "proto-upnp-igd-1" }
```
