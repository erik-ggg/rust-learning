# Tokio Server

This proyects contains a server made with tokio runtime. The server reads the data and sends it
back to all the sockets connected that not have the same address that the socket that sended the date
to the server.

## Run and Test

Run with:

```rust
cargo run
```

To connect and send data to the server open another console and create a telnet conexion:

```rust
telnet localhost 8080
```
