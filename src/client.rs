use crate::{Request, Response, Result};
use std::net::{Shutdown, SocketAddr, TcpStream};

pub struct KvClient {
    addr: SocketAddr,
}

impl KvClient {
    pub fn connect(addr: SocketAddr) -> Result<Self> {
        Ok(KvClient { addr })
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let response = self.send_request(&Request::Get { key })?;
        match response {
            Response::Ok(value) => Ok(value),
            Response::Err(e) => Err(failure::err_msg(e)),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let response = self.send_request(&Request::Set { key, value })?;
        match response {
            Response::Ok(_) => Ok(()),
            Response::Err(e) => Err(failure::err_msg(e)),
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let response = self.send_request(&Request::Remove { key })?;
        match response {
            Response::Ok(_) => Ok(()),
            Response::Err(e) => Err(failure::err_msg(e)),
        }
    }

    async fn send_request(&mut self, request: &Request) -> Result<Response> {
        let mut stream = TcpStream::connect(self.addr)?;
        serde_json::to_writer(&mut stream, request)?;
        stream.shutdown(Shutdown::Write)?;
        let response: Response = serde_json::from_reader(&stream)?;
        Ok(response)
    }
}
