use std::{env, error::Error, fmt, io};
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;

pub struct Server {
    pub ip: String,
    pub port: u16,
    actors: u8
}

impl Server {
    
    pub fn new(ip: String, port: u16) -> Self {
        Self{ip, port}
    }

    pub async fn start(&self) {

        let listener = TcpListener::bind(format!("{}:{}", self.ip, self.port)).await.unwrap();
        loop {
            let (socket, _) = listener.accept().await.unwrap();

            tokio::spawn(async move {
                
            });
        }

    }

    pub fn add_route_actor(&self, route: String, actor: u8) -> Self {

    }

    


}