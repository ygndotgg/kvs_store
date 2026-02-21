use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};

use criterion::{BenchmarkId, Criterion};
use crossbeam_utils::sync::WaitGroup;
use kvs::{
    KvServer, KvStore, Request, Response,
    thread_pool::{SharedQueueThreadPool, ThreadPool},
};
use panic_control::ThreadResultExt;
use tempfile::TempDir;

pub fn thread_counts() -> Vec<u32> {
    let cpu = num_cpus::get() as u32;
    let mut counts = vec![1, 2, 4];
    let mut n = 6;
    while n <= cpu * 2 {
        counts.push(n);
        n += 2;
    }
    counts
}

pub fn send_request(addr: SocketAddr, request: &Request) -> Response {
    let mut stream = TcpStream::connect(addr).expect("Failed to connect");
    serde_json::to_writer(&mut stream, request).expect("Failed to write");
    stream
        .shutdown(Shutdown::Write)
        .expect("Failed to shutdown write");
    serde_json::from_reader(&mut stream).expect("Failed to read response")
}

pub fn available_port() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
    let addr = listener.local_addr().unwrap();
    drop(listener);
    addr
}

fn write_queued_kvstore(c: &mut Criterion, input: Vec<u32>) {
    let mut group = c.benchmark_group("write_queued_kvstore");
    for ine in input {
        group.throughput(criterion::Throughput::Bytes(ine as u64));
        group.bench_with_input(BenchmarkId::from_parameter(ine), &ine, |b, &inde| {
            let temp_dir = TempDir::new().unwrap();
            let addr = available_port();
            let engine = KvStore::open(temp_dir.path()).expect("Unable to Start the engine");
            let server_pool =
                SharedQueueThreadPool::new(inde).expect("unable to create the server thread pool");
            let mut server =
                KvServer::new(addr, engine, server_pool).expect("unable to create the server");
            std::thread::spawn(move || {
                server.run();
            });
            let client_pool =
                SharedQueueThreadPool::new(1000).expect("Unable to create the client thread pool");
            b.iter(|| {
                let wg = WaitGroup::new();
                for i in 0..1000 {
                    let wg = wg.clone();
                    client_pool.spawn(move || {
                        let response = send_request(
                            addr,
                            &Request::Set {
                                key: format!("key{}", i),
                                value: "value".to_string(),
                            },
                        );
                        assert!(matches!(response, Response::Ok(_)));
                    });
                    drop(wg);
                }
                wg.wait();
            });
            server.shutdown();
        });
    }
}

pub fn main() {}
