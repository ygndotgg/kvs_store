use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use crossbeam_utils::sync::WaitGroup;
use kvs::{
    KvServer, KvStore, KvsEngine, Request, Response, SledKvsEngine,
    thread_pool::{RayonThreadPool, SharedQueueThreadPool, ThreadPool},
};
use tempfile::TempDir;

fn kvs_write(c: &mut Criterion) {
    c.bench_function("kvs_write", |b| {
        b.iter(|| {
            let temp_dir = TempDir::new().unwrap();
            let mut store = KvStore::open(temp_dir.path()).unwrap();

            for i in 0..100 {
                store
                    .set(format!("key{}", i), format!("value{}", i))
                    .unwrap();
            }
        });
    });
}

fn sled_write(c: &mut Criterion) {
    c.bench_function("sled_function", |b| {
        b.iter(|| {
            let temp = TempDir::new().unwrap();
            let mut store = SledKvsEngine::open(temp.path()).unwrap();
            for i in 0..100 {
                store
                    .set(format!("key{}", i), format!("value{}", i))
                    .unwrap();
            }
        });
    });
}

fn kvs_read(c: &mut Criterion) {
    let temp = TempDir::new().unwrap();
    let mut store = KvStore::open(temp.path()).unwrap();
    for i in 0..1000 {
        store
            .set(format!("key{}", i), format!("value{}", i))
            .unwrap();
    }
    c.bench_function("kv_read_bench", |b| {
        b.iter(|| {
            for i in 0..1000 {
                store.get(format!("key{}", i)).unwrap();
            }
        });
    });
}

fn sled_read(c: &mut Criterion) {
    let temp = TempDir::new().unwrap();
    let mut store = SledKvsEngine::open(temp.path()).unwrap();
    for i in 0..1000 {
        store
            .set(format!("key{}", i), format!("value{}", i))
            .unwrap();
    }
    c.bench_function("sled_read_bench", |b| {
        b.iter(|| {
            for i in 0..1000 {
                store.get(format!("key{}", i)).unwrap();
            }
        });
    });
}

pub fn thread_counts() -> Vec<u32> {
    // Reduced for faster benchmarking
    vec![1, 4, num_cpus::get() as u32, 32]
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

fn write_rayon_kvstore(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_queued_kvstore");
    let inputs = thread_counts();
    for ine in inputs {
        group.throughput(criterion::Throughput::Bytes(ine as u64));
        group.bench_with_input(BenchmarkId::from_parameter(ine), &ine, |b, &inde| {
            let temp_dir = TempDir::new().unwrap();
            let addr = available_port();
            let engine = KvStore::open(temp_dir.path()).expect("Unable to Start the engine");
            let server_pool =
                SharedQueueThreadPool::new(inde).expect("unable to create the server thread pool");
            let server =
                KvServer::new(addr, engine, server_pool).expect("unable to create the server");

            let shutdown_handle = server.shutdown_handle();
            std::thread::spawn(move || {
                server.run();
            });

            // Give server time to start
            std::thread::sleep(std::time::Duration::from_millis(1000));

            let client_pool =
                RayonThreadPool::new(100).expect("Unable to create the client thread pool");
            b.iter(|| {
                let wg = WaitGroup::new();
                for i in 0..100 {
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
                        drop(wg);
                    });
                }
                wg.wait();
            });
            shutdown_handle.store(true, std::sync::atomic::Ordering::SeqCst);
        });
    }
    group.finish();
}

fn write_queued_kvstore(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_queued_kvstore");
    let inputs = thread_counts();
    for ine in inputs {
        group.throughput(criterion::Throughput::Bytes(ine as u64));
        group.bench_with_input(BenchmarkId::from_parameter(ine), &ine, |b, &inde| {
            let temp_dir = TempDir::new().unwrap();
            let addr = available_port();
            let engine = SledKvsEngine::open(temp_dir.path()).expect("Unable to Start the engine");
            let server_pool =
                SharedQueueThreadPool::new(inde).expect("unable to create the server thread pool");
            let server =
                KvServer::new(addr, engine, server_pool).expect("unable to create the server");

            let shutdown_handle = server.shutdown_handle();
            std::thread::spawn(move || {
                server.run();
            });

            // Give server time to start
            std::thread::sleep(std::time::Duration::from_millis(1000));

            let client_pool =
                SharedQueueThreadPool::new(100).expect("Unable to create the client thread pool");
            b.iter(|| {
                let wg = WaitGroup::new();
                for i in 0..100 {
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
                        drop(wg);
                    });
                }
                wg.wait();
            });
            shutdown_handle.store(true, std::sync::atomic::Ordering::SeqCst);
        });
    }
    group.finish();
}

fn write_rayon_sled(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_queued_kvstore");
    let inputs = thread_counts();
    for ine in inputs {
        group.throughput(criterion::Throughput::Bytes(ine as u64));
        group.bench_with_input(BenchmarkId::from_parameter(ine), &ine, |b, &inde| {
            let temp_dir = TempDir::new().unwrap();
            let addr = available_port();
            let engine = SledKvsEngine::open(temp_dir.path()).expect("Unable to Start the engine");
            let server_pool =
                SharedQueueThreadPool::new(inde).expect("unable to create the server thread pool");
            let server =
                KvServer::new(addr, engine, server_pool).expect("unable to create the server");

            let shutdown_handle = server.shutdown_handle();
            std::thread::spawn(move || {
                server.run();
            });

            // Give server time to start
            std::thread::sleep(std::time::Duration::from_millis(1000));

            let client_pool =
                SharedQueueThreadPool::new(100).expect("Unable to create the client thread pool");
            b.iter(|| {
                let wg = WaitGroup::new();
                for i in 0..100 {
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
                        drop(wg);
                    });
                }
                wg.wait();
            });
            shutdown_handle.store(true, std::sync::atomic::Ordering::SeqCst);
        });
    }
    group.finish();
}

fn comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("comparison");
    let inputs = thread_counts();
    for ine in inputs {
        group.throughput(criterion::Throughput::Bytes(ine as u64));

        group.bench_with_input(
            BenchmarkId::new("write_rayon_kvstore", ine),
            &ine,
            |b, &inde| {
                let temp_dir = TempDir::new().unwrap();
                let addr = available_port();
                let engine = KvStore::open(temp_dir.path()).expect("Unable to Start the engine");
                let server_pool = SharedQueueThreadPool::new(inde)
                    .expect("unable to create the server thread pool");
                let server =
                    KvServer::new(addr, engine, server_pool).expect("unable to create the server");

                let shutdown_handle = server.shutdown_handle();
                std::thread::spawn(move || {
                    server.run();
                });

                std::thread::sleep(std::time::Duration::from_millis(1000));

                let client_pool =
                    RayonThreadPool::new(100).expect("Unable to create the client thread pool");
                b.iter(|| {
                    let wg = WaitGroup::new();
                    for i in 0..100 {
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
                            drop(wg);
                        });
                    }
                    wg.wait();
                });
                shutdown_handle.store(true, std::sync::atomic::Ordering::SeqCst);
            },
        );

        group.bench_with_input(
            BenchmarkId::new("write_queued_kvstore", ine),
            &ine,
            |b, &inde| {
                let temp_dir = TempDir::new().unwrap();
                let addr = available_port();
                let engine =
                    SledKvsEngine::open(temp_dir.path()).expect("Unable to Start the engine");
                let server_pool = SharedQueueThreadPool::new(inde)
                    .expect("unable to create the server thread pool");
                let server =
                    KvServer::new(addr, engine, server_pool).expect("unable to create the server");

                let shutdown_handle = server.shutdown_handle();
                std::thread::spawn(move || {
                    server.run();
                });

                std::thread::sleep(std::time::Duration::from_millis(1000));

                let client_pool = SharedQueueThreadPool::new(100)
                    .expect("Unable to create the client thread pool");
                b.iter(|| {
                    let wg = WaitGroup::new();
                    for i in 0..100 {
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
                            drop(wg);
                        });
                    }
                    wg.wait();
                });
                shutdown_handle.store(true, std::sync::atomic::Ordering::SeqCst);
            },
        );

        group.bench_with_input(
            BenchmarkId::new("write_rayon_sled", ine),
            &ine,
            |b, &inde| {
                let temp_dir = TempDir::new().unwrap();
                let addr = available_port();
                let engine =
                    SledKvsEngine::open(temp_dir.path()).expect("Unable to Start the engine");
                let server_pool = SharedQueueThreadPool::new(inde)
                    .expect("unable to create the server thread pool");
                let server =
                    KvServer::new(addr, engine, server_pool).expect("unable to create the server");

                let shutdown_handle = server.shutdown_handle();
                std::thread::spawn(move || {
                    server.run();
                });

                std::thread::sleep(std::time::Duration::from_millis(1000));

                let client_pool = SharedQueueThreadPool::new(100)
                    .expect("Unable to create the client thread pool");
                b.iter(|| {
                    let wg = WaitGroup::new();
                    for i in 0..100 {
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
                            drop(wg);
                        });
                    }
                    wg.wait();
                });
                shutdown_handle.store(true, std::sync::atomic::Ordering::SeqCst);
            },
        );

        group.bench_with_input(
            BenchmarkId::new("read_rayon_kvstore", ine),
            &ine,
            |b, &inde| {
                let temp_dir = TempDir::new().unwrap();
                let addr = available_port();
                let engine = KvStore::open(temp_dir.path()).expect("Unable to Start the engine");
                let server_pool = SharedQueueThreadPool::new(inde)
                    .expect("unable to create the server thread pool");
                let server =
                    KvServer::new(addr, engine, server_pool).expect("unable to create the server");

                let shutdown_handle = server.shutdown_handle();
                std::thread::spawn(move || {
                    server.run();
                });

                std::thread::sleep(std::time::Duration::from_millis(1000));

                let client_pool =
                    RayonThreadPool::new(100).expect("Unable to create the client thread pool");
                b.iter(|| {
                    let wg = WaitGroup::new();
                    for i in 0..100 {
                        let wg = wg.clone();
                        client_pool.spawn(move || {
                            let response = send_request(
                                addr,
                                &Request::Get {
                                    key: format!("key{}", i),
                                },
                            );
                            assert!(matches!(response, Response::Ok(_)));
                            drop(wg);
                        });
                    }
                    wg.wait();
                });
                shutdown_handle.store(true, std::sync::atomic::Ordering::SeqCst);
            },
        );

        group.bench_with_input(
            BenchmarkId::new("read_queued_kvstore", ine),
            &ine,
            |b, &inde| {
                let temp_dir = TempDir::new().unwrap();
                let addr = available_port();
                let engine =
                    SledKvsEngine::open(temp_dir.path()).expect("Unable to Start the engine");
                let server_pool = SharedQueueThreadPool::new(inde)
                    .expect("unable to create the server thread pool");
                let server =
                    KvServer::new(addr, engine, server_pool).expect("unable to create the server");

                let shutdown_handle = server.shutdown_handle();
                std::thread::spawn(move || {
                    server.run();
                });

                std::thread::sleep(std::time::Duration::from_millis(1000));

                let client_pool = SharedQueueThreadPool::new(100)
                    .expect("Unable to create the client thread pool");
                b.iter(|| {
                    let wg = WaitGroup::new();
                    for i in 0..100 {
                        let wg = wg.clone();
                        client_pool.spawn(move || {
                            let response = send_request(
                                addr,
                                &Request::Get {
                                    key: format!("key{}", i),
                                },
                            );
                            assert!(matches!(response, Response::Ok(_)));
                            drop(wg);
                        });
                    }
                    wg.wait();
                });
                shutdown_handle.store(true, std::sync::atomic::Ordering::SeqCst);
            },
        );

        group.bench_with_input(
            BenchmarkId::new("read_rayon_sled", ine),
            &ine,
            |b, &inde| {
                let temp_dir = TempDir::new().unwrap();
                let addr = available_port();
                let engine =
                    SledKvsEngine::open(temp_dir.path()).expect("Unable to Start the engine");
                let server_pool = SharedQueueThreadPool::new(inde)
                    .expect("unable to create the server thread pool");
                let server =
                    KvServer::new(addr, engine, server_pool).expect("unable to create the server");

                let shutdown_handle = server.shutdown_handle();
                std::thread::spawn(move || {
                    server.run();
                });

                std::thread::sleep(std::time::Duration::from_millis(1000));

                let client_pool = SharedQueueThreadPool::new(100)
                    .expect("Unable to create the client thread pool");
                b.iter(|| {
                    let wg = WaitGroup::new();
                    for i in 0..100 {
                        let wg = wg.clone();
                        client_pool.spawn(move || {
                            let response = send_request(
                                addr,
                                &Request::Get {
                                    key: format!("key{}", i),
                                },
                            );
                            assert!(matches!(response, Response::Ok(_)));
                            drop(wg);
                        });
                    }
                    wg.wait();
                });
                shutdown_handle.store(true, std::sync::atomic::Ordering::SeqCst);
            },
        );
    }
    group.finish();
}

fn read_rayon_kvstore(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_queued_kvstore");
    let inputs = thread_counts();
    for ine in inputs {
        group.throughput(criterion::Throughput::Bytes(ine as u64));
        group.bench_with_input(BenchmarkId::from_parameter(ine), &ine, |b, &inde| {
            let temp_dir = TempDir::new().unwrap();
            let addr = available_port();
            let engine = KvStore::open(temp_dir.path()).expect("Unable to Start the engine");
            let server_pool =
                SharedQueueThreadPool::new(inde).expect("unable to create the server thread pool");
            let server =
                KvServer::new(addr, engine, server_pool).expect("unable to create the server");

            let shutdown_handle = server.shutdown_handle();
            std::thread::spawn(move || {
                server.run();
            });

            // Give server time to start
            std::thread::sleep(std::time::Duration::from_millis(1000));

            let client_pool =
                RayonThreadPool::new(100).expect("Unable to create the client thread pool");
            b.iter(|| {
                let wg = WaitGroup::new();
                for i in 0..100 {
                    let wg = wg.clone();
                    client_pool.spawn(move || {
                        let response = send_request(
                            addr,
                            &Request::Get {
                                key: format!("key{}", i),
                            },
                        );
                        assert!(matches!(response, Response::Ok(_)));
                        drop(wg);
                    });
                }
                wg.wait();
            });
            shutdown_handle.store(true, std::sync::atomic::Ordering::SeqCst);
        });
    }
    group.finish();
}

fn read_queued_kvstore(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_queued_kvstore");
    let inputs = thread_counts();
    for ine in inputs {
        group.throughput(criterion::Throughput::Bytes(ine as u64));
        group.bench_with_input(BenchmarkId::from_parameter(ine), &ine, |b, &inde| {
            let temp_dir = TempDir::new().unwrap();
            let addr = available_port();
            let engine = SledKvsEngine::open(temp_dir.path()).expect("Unable to Start the engine");
            let server_pool =
                SharedQueueThreadPool::new(inde).expect("unable to create the server thread pool");
            let server =
                KvServer::new(addr, engine, server_pool).expect("unable to create the server");

            let shutdown_handle = server.shutdown_handle();
            std::thread::spawn(move || {
                server.run();
            });

            // Give server time to start
            std::thread::sleep(std::time::Duration::from_millis(1000));

            let client_pool =
                SharedQueueThreadPool::new(100).expect("Unable to create the client thread pool");
            b.iter(|| {
                let wg = WaitGroup::new();
                for i in 0..100 {
                    let wg = wg.clone();
                    client_pool.spawn(move || {
                        let response = send_request(
                            addr,
                            &Request::Get {
                                key: format!("key{}", i),
                            },
                        );
                        assert!(matches!(response, Response::Ok(_)));
                        drop(wg);
                    });
                }
                wg.wait();
            });
            shutdown_handle.store(true, std::sync::atomic::Ordering::SeqCst);
        });
    }
    group.finish();
}

fn read_rayon_sled(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_queued_kvstore");
    let inputs = thread_counts();
    for ine in inputs {
        group.throughput(criterion::Throughput::Bytes(ine as u64));
        group.bench_with_input(BenchmarkId::from_parameter(ine), &ine, |b, &inde| {
            let temp_dir = TempDir::new().unwrap();
            let addr = available_port();
            let engine = SledKvsEngine::open(temp_dir.path()).expect("Unable to Start the engine");
            let server_pool =
                SharedQueueThreadPool::new(inde).expect("unable to create the server thread pool");
            let server =
                KvServer::new(addr, engine, server_pool).expect("unable to create the server");

            let shutdown_handle = server.shutdown_handle();
            std::thread::spawn(move || {
                server.run();
            });

            // Give server time to start
            std::thread::sleep(std::time::Duration::from_millis(1000));

            let client_pool =
                SharedQueueThreadPool::new(100).expect("Unable to create the client thread pool");
            b.iter(|| {
                let wg = WaitGroup::new();
                for i in 0..100 {
                    let wg = wg.clone();
                    client_pool.spawn(move || {
                        let response = send_request(
                            addr,
                            &Request::Get {
                                key: format!("key{}", i),
                            },
                        );
                        assert!(matches!(response, Response::Ok(_)));
                        drop(wg);
                    });
                }
                wg.wait();
            });
            shutdown_handle.store(true, std::sync::atomic::Ordering::SeqCst);
        });
    }
    group.finish();
}
// criterion_group!(benches, kvs_write, sled_write, kvs_read, sled_read);

// criterion_group! {
//     name = benches;
//     config = Criterion::default()
//         .sample_size(5)
//         .warm_up_time(std::time::Duration::from_millis(100))
//         .measurement_time(std::time::Duration::from_millis(500));
//     targets =
//      write_queued_kvstore,
//               write_rayon_kvstore,
//               write_rayon_sled,
//               read_queued_kvstore,
//               read_rayon_kvstore,
//               read_rayon_sled
// }

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(10)
        .warm_up_time(std::time::Duration::from_millis(100))
        .measurement_time(std::time::Duration::from_millis(500));
    targets = comparison
}
criterion_main!(benches);
