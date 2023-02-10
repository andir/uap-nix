use tokio::sync::broadcast::{Sender, Receiver, channel, error::{SendError, RecvError}};
use log::debug;

pub struct Subscriber<T> {
    receiver: Receiver<T>,
}

impl<T> Subscriber<T> where T: Clone {
    pub async fn recv(&mut self) -> Option<T> {
        loop {
            match self.receiver.recv().await.map(Some) {
                Ok(v) => return v,
                // We missed a few messages, doesn't matter, try again
                Err(RecvError::Lagged(n)) => {
                    debug!("Message received lagged, missed {} messages", n);
                    continue;
                },
                Err(e) => return None,
            }
        }
    }
}

impl<T> From<Receiver<T>> for Subscriber<T> {
    fn from(receiver: Receiver<T>) -> Self {
        Self { receiver }
    }
}

pub struct BroadcastChannel<T> {
    sender: Sender<T>,
}


impl<T> BroadcastChannel<T> where T: Clone {
    pub fn new() -> Self {
        let (sender, _rx) = channel(1);
        Self { sender }
    }
    
    pub fn subscribe(&mut self) -> Subscriber<T> {
        Subscriber::from(self.sender.subscribe())
    }
    
    pub fn send(&mut self, msg: T) {
        match self.sender.send(msg) {
            Ok(n) =>{
                debug!("Sent message to {} receivers", n);
            },
            Err(SendError(x)) => {
                debug!("Message not sent to any receivers, there aren't any");
            }
        }
    }
}
