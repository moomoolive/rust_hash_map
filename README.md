# Rust Hash Map

A simple hash map implemented in rust, all code is found in code/src/

If you don't have rustup downloaded you can just copy source and [paste code here](https://play.rust-lang.org/) to test.

## Commands

Run main
```
cargo run
```

Run tests
```
cargo test
```


### Overview

This is a simple implemenation of a hash map (or hash table) in rust, collision resolution strategy employed is [linear probing](https://en.wikipedia.org/wiki/Linear_probing#:~:text=Linear%20probing%20is%20a%20scheme,associated%20with%20a%20given%20key.&text=Along%20with%20quadratic%20probing%20and,a%20form%20of%20open%20addressing.).

The implementation has many limitations, mostly because I was too lazy to extend it's functionality 😅. Some limitations include:

* Hash map buffer will not resize if max capacity is met
* Keys cannot be overwritten, but must be removed then re-inserted
* Collision strategy is very simple, which makes lookup performance for all hashes dip severely the more collisions encountered
