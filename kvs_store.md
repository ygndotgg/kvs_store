## PART I: The Persistence Journey

### Chapter 1: The Birth of a Key-Value Store
[Context: Every database starts somewhere - understanding the fundamentals before tackling complexity]

#### The In-Memory Problem
The initial implementation (`Project 1 Completed`) was a simple in-memory key-value store. Data lived only in a `HashMap<String, String>`. 

**Why naive approaches fail:**
- Restart the process? All data gone.
- System crash? Everything lost.
- No durability guarantee means no real database.

#### The Solution: Append-Only Log
The journey began with the simplest persistence strategy: write every command to a log file.

```rust
// The core idea: never modify, only append
let serialized = serde_json::to_string(&cmd)?;
writeln!(inner.writer, "{}", serialized)?;
inner.writer.flush()?;
```

This is the foundation of write-ahead logging (WAL) - a technique used by SQLite, PostgreSQL, and many production databases.

---

### Chapter 2: The Pointer Revolution
[Context: Storing data is easy. Finding it again is the problem.]

#### The Full Scan Problem
With just a log file, reading a key meant scanning from the beginning:
- O(n) reads for every get operation
- Performance degrades linearly with data size
- Unacceptable for any real workload

#### The Solution: LogPointer Index
```rust
struct LogPointer {
    offset: u64,    // byte position where command starts
    length: u64,    // serialized command size
    file_id: u64,   // which log file
}
```

An in-memory `HashMap<String, LogPointer>` now provides O(1) lookups while keeping the append-only log for durability. This is the essence of an LSM-tree's memtable.

```
┌─────────────────────────────────────────┐
│           In-Memory Index               │
│  ┌─────────────────────────────────┐   │
│  │ "user:1" → {offset: 0, len: 45} │   │
│  │ "user:2" → {offset: 45, len: 50}│   │
│  └─────────────────────────────────┘   │
└─────────────────┬───────────────────────┘
                  │ direct jump
                  ▼
┌─────────────────────────────────────────┐
│         Disk Log File                   │
│  {"Set":{"key":"user:1","value":"..."}}│
│  {"Set":{"key":"user:2","value":"..."}}│
└─────────────────────────────────────────┘
```

---

### Chapter 3: The Compaction Chronicles
[Context: Append-only logs grow forever. Updates create orphans. Deletions leave ghosts.]

#### The Space Problem
Every update writes a new entry. The old entry becomes "dead" but still occupies disk space:

```
Set key="x" value="1"  ← dead
Set key="x" value="2"  ← dead  
Set key="x" value="3"  ← live
```

The log file grows unbounded. 90% could be garbage.

#### The Solution: Log Compaction
When `uncompacted_bytes > COMPACTION_THRESHOLD`:

```rust
impl KvStoreInner {
    pub fn compact(&mut self) -> Result<()> {
        // Create new compacted file with only live entries
        for (_key, log_ptr) in store.iter_mut() {
            // Read old, write to new, update pointer
            log_ptr.file_id = compaction_file_id;
        }
        // Delete old files, use new compacted file
    }
}
```

```
Before Compaction:          After Compaction:
┌─────────────────┐         ┌─────────────────┐
│ 1.log (10MB)    │         │ 3.log (1MB)     │
│   x=1 (dead)    │   ───►  │   x=3 (live)    │
│   x=2 (dead)    │         │   y=5 (live)    │
│   x=3 (live)    │         └─────────────────┘
│   y=5 (live)    │
└─────────────────┘
```

---

## PART II: The Network Layer

### Chapter 4: The Single-Process Illusion
[Context: A database that can't be accessed remotely is just a library.]

#### The Local-Only Problem
The store worked perfectly in-process. But real databases serve multiple clients over the network.

#### The Solution: Client-Server Architecture
A protocol built on JSON over TCP:

```rust
#[derive(Serialize, Deserialize)]
pub enum Request {
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Ok(Option<String>),
    Err(String),
}
```

Client flow:
```rust
// 1. Connect
let mut stream = TcpStream::connect(addr)?;
// 2. Send request
serde_json::to_writer(&mut stream, &request)?;
// 3. Signal end of request
stream.shutdown(Shutdown::Write)?;
// 4. Read response
let response: Response = serde_json::from_reader(&mut stream)?;
```

---

### Chapter 5: The Concurrency Challenge
[Context: One client is easy. Multiple concurrent clients is where databases earn their keep.]

#### Attempt 1: The Naive Thread-Per-Request
```rust
impl ThreadPool for NaiveThreadPool {
    fn spawn<F>(&self, job: F) {
        std::thread::spawn(job);  // Unbounded thread creation!
    }
}
```

**Why it fails:** 10,000 concurrent requests = 10,000 threads. Memory exhaustion. Context switching overhead.

#### Attempt 2: Shared Queue ThreadPool
```rust
pub struct SharedQueueThreadPool {
    sender: mpsc::Sender<Message>,
    workers: Vec<Worker>,
}

enum Message {
    Terminate,
    Job(Box<dyn FnOnce() + Send + 'static>),
}
```

Workers pull jobs from a shared queue:
```
                ┌──────────────┐
   Job ────────►│   Channel    │
                └──────┬───────┘
                       │
        ┌──────────────┼──────────────┐
        ▼              ▼              ▼
   ┌─────────┐   ┌─────────┐   ┌─────────┐
   │Worker 1 │   │Worker 2 │   │Worker N │
   └─────────┘   └─────────┘   └─────────┘
```

#### Attempt 3: Rayon ThreadPool
For CPU-bound workloads, Rayon's work-stealing scheduler provides better load balancing:

```rust
impl ThreadPool for RayonThreadPool {
    fn new(threads: u32) -> Result<Self> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads as usize)
            .build()?;
        Ok(RayonThreadPool { pool })
    }
}
```

---

## PART III: The Engine Abstraction

### Chapter 6: The Fallacy of One True Engine
[Context: Why build your own storage engine when battle-tested alternatives exist?]

#### The Monolithic Problem
The key-value store was tightly coupled. Hard to compare, hard to test alternatives.

#### The Solution: The KvsEngine Trait
```rust
pub trait KvsEngine: Clone + Send + 'static {
    fn set(&self, key: String, value: String) -> Result<()>;
    fn get(&self, key: String) -> Result<Option<String>>;
    fn remove(&self, key: String) -> Result<()>;
}
```

Now multiple engines implement the same interface:

**Custom KvStore:**
```rust
impl KvsEngine for KvStore {
    fn set(&self, key: String, value: String) -> Result<()> {
        // Append to log, update index, maybe compact
    }
}
```

**Sled Engine (embedded database):**
```rust
impl KvsEngine for SledKvsEngine {
    fn set(&self, key: String, value: String) -> Result<()> {
        self.db.insert(key.as_bytes(), value.as_bytes())?;
        self.db.flush()?;
        Ok(())
    }
}
```

The server becomes engine-agnostic:
```rust
pub struct KvServer<E, P>
where
    E: KvsEngine,
    P: ThreadPool,
{
    engine: Arc<E>,
    pool: P,
    listener: TcpListener,
}
```

---

### Chapter 7: The Server Architecture
[Context: Clean separation of concerns makes systems maintainable.]

#### The Modular Server
```
src/
├── bin/
│   ├── kvs-server.rs    # CLI entry point
│   └── kvs-client.rs    # CLI client
├── kvs.rs               # KvStore implementation
├── sled_engine.rs       # Sled wrapper
├── server.rs            # Network server
├── thread_pool/
│   ├── mod.rs           # ThreadPool trait
│   ├── naive.rs         # Thread-per-request
│   ├── shared_queue.rs  # Bounded pool
│   └── rayon.rs         # Work-stealing pool
└── lib.rs               # Public API
```

---

## PART IV: The Benchmark Arena

### Chapter 8: The Performance Question
[Context: Which engine? Which thread pool? Let the numbers decide.]

#### The Comparison Suite
```rust
fn comparison(c: &mut Criterion) {
    let inputs = vec![1, 4, num_cpus::get() as u32, 32];
    
    // Matrix of tests:
    // - Engines: KvStore vs SledKvsEngine
    // - Operations: Read vs Write
    // - Thread counts: 1, 4, N, 32
    // - Client pools: SharedQueue vs Rayon
}
```

The benchmark spawns a server, fires 100 concurrent requests per iteration, and measures throughput:

```rust
b.iter(|| {
    let wg = WaitGroup::new();
    for i in 0..100 {
        let wg = wg.clone();
        client_pool.spawn(move || {
            let response = send_request(addr, &Request::Set { ... });
            drop(wg);
        });
    }
    wg.wait();
});
```

---

### Chapter Final: The Code Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         CLI Layer                           │
│  ┌─────────────────┐          ┌─────────────────┐         │
│  │  kvs-server     │          │  kvs-client     │         │
│  │  (clap parser)  │          │  (clap parser)  │         │
│  └────────┬────────┘          └─────────────────┘         │
└───────────┼───────────────────────────────────────────────┘
            │
┌───────────┼───────────────────────────────────────────────┐
│           ▼         Server Layer                           │
│  ┌─────────────────────────────────────────┐              │
│  │            KvServer<E, P>               │              │
│  │  ┌─────────────┐  ┌─────────────────┐  │              │
│  │  │ ThreadPool  │  │   TcpListener   │  │              │
│  │  │ (trait)     │  │   (non-blocking)│  │              │
│  │  └─────────────┘  └─────────────────┘  │              │
│  └────────────────────┬────────────────────┘              │
└───────────────────────┼───────────────────────────────────┘
                        │
┌───────────────────────┼───────────────────────────────────┐
│                       ▼         Engine Layer              │
│  ┌─────────────────────────────────────────────────────┐ │
│  │              KvsEngine (trait)                      │ │
│  └──────────────────────┬──────────────────────────────┘ │
│           ┌─────────────┴─────────────┐                  │
│           ▼                           ▼                  │
│  ┌─────────────────┐       ┌─────────────────┐          │
│  │    KvStore      │       │ SledKvsEngine   │          │
│  │  (custom WAL)   │       │   (sled db)     │          │
│  └─────────────────┘       └─────────────────┘          │
└──────────────────────────────────────────────────────────┘
```

---

## Epilogue: The Rust Advantage

**What the journey taught:**

1. **Type-driven design:** The `KvsEngine` trait makes swapping implementations trivial. The type system ensures thread safety with `Send + 'static`.

2. **Fearless concurrency:** `Arc<Mutex<T>>` for shared state, channels for message passing. No data races, guaranteed by the compiler.

3. **Zero-cost abstractions:** Traits and generics compile down to direct calls. No virtual dispatch overhead unless explicitly wanted.

4. **Error handling:** The `Result<T>` type forces handling failure paths. No silent exceptions.

**Key architectural wins:**
- Append-only log for durability
- In-memory index for fast reads
- Compaction for space reclamation
- Trait abstraction for engine flexibility
- Thread pool abstraction for concurrency strategies

---

The Code: https://github.com/ygndotgg/kvs

**KVS: A Key-Value Store from Scratch** | my thoughts
