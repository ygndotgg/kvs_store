# KVS: Persistent Key-Value Store in Rust

`kvs` is a Rust-based key-value store that started as a local persistence engine and has grown into a concurrent TCP server with pluggable storage engines and thread pools. The project focuses on core database systems ideas such as append-only logging, in-memory indexing, log compaction, request/response protocols, and concurrent request handling.

The codebase currently supports a custom log-structured engine (`KvStore`) and a `sled`-backed engine behind a shared trait-based abstraction. It is designed as a strong systems project and a foundation for future distributed systems work.

## What This Project Does

- Stores key-value data with persistent on-disk storage
- Supports `set`, `get`, and `remove` operations
- Exposes the store over a TCP server using JSON-based requests and responses
- Handles concurrent client requests with a configurable thread-pool layer
- Benchmarks the custom engine against `sled`
- Includes tests for persistence, compaction, and concurrent access

## Architecture Overview

This project is built in layers:

### 1. Storage Engine

The custom `KvStore` uses:

- An `append-only log` for durable writes
- An in-memory `HashMap<String, LogPointer>` index for fast key lookup
- `log compaction` to reclaim stale entries created by updates and deletes

Each key maps to a log pointer containing:

- file id
- byte offset
- entry length

This lets reads jump directly to the latest value on disk without scanning the entire log.

### 2. Engine Abstraction

The `KvsEngine` trait defines a common interface:

- `set`
- `get`
- `remove`

Current implementations:

- `KvStore`: custom log-structured engine
- `SledKvsEngine`: wrapper over the `sled` embedded database

This keeps the networking layer independent from the underlying storage engine.

### 3. Networking Layer

The server uses JSON over TCP with a simple request/response model:

- `Request::Set { key, value }`
- `Request::Get { key }`
- `Request::Remove { key }`

Responses are returned as:

- `Response::Ok(Option<String>)`
- `Response::Err(String)`

The client writes one request per connection, shuts down the write half of the socket, and waits for the server response.

### 4. Concurrency Model

The server accepts TCP connections and dispatches each request to a thread pool. The project includes multiple thread-pool implementations:

- `NaiveThreadPool`
- `SharedQueueThreadPool`
- `RayonThreadPool`

The current server binary uses `SharedQueueThreadPool` with a worker count derived from available CPU cores.

## Project Structure

```text
src/
  bin/
    kvs-client.rs   # CLI client for get/set/rm over TCP
    kvs-server.rs   # Server binary
  kvs.rs            # Custom persistent engine
  server.rs         # TCP server implementation
  sled_engine.rs    # sled-backed engine
  thread_pool/      # Thread-pool implementations
tests/
  kv_store.rs       # Persistence, compaction, concurrency tests
  cli.rs
  thread_pool.rs
benches/
  benches.rs        # Criterion benchmarks
```

## Running the Project

### Start the server

```bash
cargo run --bin kvs-server -- --addr 127.0.0.1:4000 --engine kvs
```

Use `--engine sled` to run the server with the `sled` backend instead.

### Use the client

Set a value:

```bash
cargo run --bin kvs-client -- set mykey myvalue --addr 127.0.0.1:4000
```

Get a value:

```bash
cargo run --bin kvs-client -- get mykey --addr 127.0.0.1:4000
```

Remove a value:

```bash
cargo run --bin kvs-client -- rm mykey --addr 127.0.0.1:4000
```

## Testing and Benchmarking

Run the test suite:

```bash
cargo test
```

Run benchmarks:

```bash
cargo bench
```

The benchmark suite compares:

- custom `KvStore` read/write behavior
- `sled` read/write behavior
- throughput under concurrent client workloads
- performance across different thread-pool sizes

## Key Systems Concepts Demonstrated

- append-only log design
- crash recovery through log replay
- direct lookups using byte-offset indexing
- compaction of stale log entries
- trait-based engine abstraction
- TCP request/response protocol design
- multi-threaded request serving
- benchmarking and concurrency validation

## Roadmap

This project is not intended to stop at a single-node key-value server.

Planned next steps include evolving it into a `distributed key-value store`, with work such as:

- replication across nodes
- distributed request routing
- fault tolerance and recovery
- cluster membership and coordination
- consistency and durability tradeoff design

The current implementation is the single-node systems foundation for that larger distributed architecture.

## Reference Notes

The repository also includes longer design notes in:

- `kvs_store.md`
- `kv_store.md`

Those files document the progression from a persistent local engine to a concurrent networked service and outline the broader distributed-systems direction of the project.
