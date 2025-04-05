# Sindri Proof Management with RocksDB

This example demonstrates how to use RocksDB to store and manage Sindri proof jobs. 
If anything should happen to your machine or network, the program can gracefully resume what its doing from a local state, rather losing track of jobs.

> ⚠️ **WARNING!**<br>
> The database design and access patterns are created for simple illustration, not performance.

### Usage
To run within this directory, run 
```SINDRI_API_KEY=<your_api_key> cargo run --release```

