# Rudis
Rudis is a Redis server implementation in Rust.

It speaks the same binary protocol as Redis and implements a subset of all Redis commands. This means you can connect to a Rudis server using any available Redis library.

```sh
$ cargo run --release  # start the rudis server
$ redis-cli -p 8888    # connect to rudis on port 8888
127.0.0.1:8888> set x 123
OK
127.0.0.1:8888> get x
123
```
