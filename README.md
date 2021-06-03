# rust_hash_map

A simple hash map implemented in rust, all code is found in code/src/main.rs

If you don't have rustup downloaded you can just copy source and [paste code here](https://play.rust-lang.org/) to test.

### Overview

Hashing function takes string as input: --> gets first character, casts to to lowercase ASCII, then if it's an alphabetic character it returns an integer corresponding to the order of the letter in the English alphabet (between 0 - 25), otherwise it returns -1;

Implementation of the hashmap is an array of 26 (corresponding to number of letters in english alphabet) arrays. The index of each array corresponds to it's ordering in the English Alphabet for example 'a' corresponds to 0, 'c' corresponds to 2.

This implementation is a little too simple, and has some obvious downsides:
-> very quickly efficency will drop dramatically because there just aren't many unique hashes.

A more advanced version of this could use the first two letters of the key as a hash value. The downsides of this would be:
-> more memory.

## Commands

Run main
```
cargo run
```

Run tests
```
cargo test
```
