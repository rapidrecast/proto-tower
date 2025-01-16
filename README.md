# Proto Tower

A collection of protocol implementations using tower.

The repository is a collection of crates that implement various protocols from wire (`(AsyncRead, AsyncWrite)`) and translate the protocol to easy to reason about requests and responses.

Because the implementation is both tower based and standardised, they can be combined.
This means you can serve multiple protocols from the same TCP or UDP binding.

The protocol implementations are not tied to either TCP or UDP, so you are free to adapt them as you need.
You do need to provide consistent readers and writers for "sessions".
That means a single TCP connection, or a single UDP source+destination port pair.

