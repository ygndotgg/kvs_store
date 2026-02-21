use std::{
    io::BufReader,
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use log::{error, info};

use crate::thread_pool::ThreadPool;
use crate::{KvsEngine, Request, Response};

pub struct KvServer<E, P>
where
    E: KvsEngine,
    P: ThreadPool,
{
    engine: Arc<E>,
    pool: P,
    listener: TcpListener,
    shutdown: Arc<AtomicBool>,
}

impl<E, P> KvServer<E, P>
where
    E: KvsEngine + Sync + 'static,
    P: ThreadPool,
{
    pub fn new(addr: SocketAddr, engine: E, pool: P) -> std::io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        Ok(KvServer {
            engine: Arc::new(engine),
            pool,
            listener,
            shutdown: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn run(&self) {
        self.listener.set_nonblocking(true).ok();
        loop {
            match self.listener.accept() {
                Ok((stream, _)) => {
                    let engine = Arc::clone(&self.engine);
                    self.pool.spawn(move || {
                        handle_client(stream, &*engine);
                    });
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    if self.shutdown.load(Ordering::SeqCst) {
                        break;
                    }
                    thread::sleep(Duration::from_millis(1));
                }
                Err(e) => {
                    error!("Error: {}", e);
                }
            }
        }
    }

    pub fn shutdown(&self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }

    pub fn shutdown_handle(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.shutdown)
    }

    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.listener.local_addr()
    }
}

fn handle_client<E: KvsEngine>(stream: TcpStream, engine: &E) {
    let reader = BufReader::new(&stream);
    let peer = stream.peer_addr().ok();
    let request: Request = match serde_json::from_reader(reader) {
        Ok(req) => req,
        Err(_) => return,
    };
    let response = match request {
        Request::Set { key, value } => match engine.set(key, value) {
            Ok(()) => Response::Ok(None),
            Err(e) => Response::Err(e.to_string()),
        },
        Request::Get { key } => match engine.get(key) {
            Ok(s) => Response::Ok(s),
            Err(e) => Response::Err(e.to_string()),
        },
        Request::Remove { key } => match engine.remove(key) {
            Ok(_) => Response::Ok(None),
            Err(e) => Response::Err(e.to_string()),
        },
    };
    if let Some(peer) = peer {
        info!("handled request from {}", peer);
    }
    if serde_json::to_writer(&stream, &response).is_err() {
        error!("Connection failed");
    }
}
