# SUPERMARKET MULTITHREADING PROJECT

## Description
This project is made to play and learn how multithreading works in Rust. It simulates the arrival and dispatch of costumers in the supermarket queues.

In order to achieve this, the program make use of _Arc_ and _Mutex_ to share the _Vec_ of queues.

## How to run

Just run the rust project as usual with cargo run and set the number the queues with the first parameter.

```
cargo run -- num_of_queues
```




