mod broadcast;

use log::info;

use serde::{Serialize, Deserialize};
use tokio::{net::{unix::SocketAddr, UnixStream}, io::AsyncWriteExt};

#[derive(Debug)]
pub enum Error {
    SocketError(std::io::Error)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub topic: String,
    pub payload: Vec<u8>,
}


pub struct Client {
    socket: tokio::net::UnixStream,
}


impl Client {
    pub async fn connect(path: String) -> Result<Self, Error> {
	let socket = tokio::net::UnixStream::connect(path).await.map_err(Error::SocketError)?;
	Ok(Self { socket })
    }
}

pub struct Server {
    socket: tokio::net::UnixListener,
}


struct ServerClient {
    socket: tokio::net::UnixStream,
    addr: SocketAddr,
}

impl ServerClient {
    fn from(socket: UnixStream, addr: SocketAddr) -> Self {
	Self { socket, addr }
    }

    async fn run(&mut self, mut subscriber: broadcast::Subscriber<Vec<u8>>) -> Result<(), Error> {
	let (reader, writer) = self.socket.split();
	
	tokio::select! {
	    msg = subscriber.recv() => {
		match msg {
		    Some(msg) => {
			writer.write_all(&msg).await?;
		    },
		    None => {},
		}
	    }
	}

	info!("ServerClient stopped receiving messages from broadcast channel");
	Ok(())
    }
}


impl Server {
    pub async fn bind(path: String) -> Result<Self, Error> {
	let socket = tokio::net::UnixListener::bind(path).map_err(Error::SocketError)?;
	Ok(Self { socket })
    }

    pub async fn run(&mut self) -> Result<(), Error> {
	let mut bc = broadcast::BroadcastChannel::new();
	loop {
	    let (socket, remote) = self.socket.accept().await.map_err(Error::SocketError)?;

	    info!("Connection from {:?}", remote);

	    let mut client = ServerClient::from(socket, remote);

	    let subscriber = bc.subscribe();

	    // spawn the writing part that sends messages to the socket whenever we receive something
	    tokio::spawn(async move {
		client.run_reader(subscriber).await.unwrap();
	    });
	}


	Ok(())
    }
}
