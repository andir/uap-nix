use capnp_rpc::{pry, rpc_twoparty_capnp};
use futures::AsyncReadExt;

mod broadcast;

pub mod nanobus_capnp {
    include!(concat!(env!("OUT_DIR"), "/src/nanobus_capnp.rs"));
}

pub use nanobus_capnp::Topic;

use capnp::capability::Promise;
use log::{info,debug,error};

#[derive(Debug)]
pub enum Error {
    SocketError(std::io::Error),
}

pub struct CapnpClient {
    tx: tokio::sync::mpsc::Sender<(nanobus_capnp::Topic, String)>,
}

impl CapnpClient {
    pub async fn run(
        path: &str,
	topics: Vec<nanobus_capnp::Topic>
    ) -> Result<
        (
            impl futures::Future<Output = ()>,
            tokio::sync::mpsc::Receiver<(nanobus_capnp::Topic, String)>,
        ),
        Error,
	> {
        let (tx, rx) = tokio::sync::mpsc::channel::<(nanobus_capnp::Topic, String)>(16);

        let socket = tokio::net::UnixStream::connect(path)
            .await
            .map_err(Error::SocketError)?;

        let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(socket).split();
        let rpc_network = Box::new(capnp_rpc::twoparty::VatNetwork::new(
            reader,
            writer,
            rpc_twoparty_capnp::Side::Client,
            Default::default(),
        ));
	let client: nanobus_capnp::subscriber::Client = capnp_rpc::new_client(CapnpClient { tx });
        let mut rpc_system = capnp_rpc::RpcSystem::new(rpc_network, Some(client.clone().client));

        let broker: nanobus_capnp::broker::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);


	let local_set = tokio::task::LocalSet::new();

	local_set.spawn_local(async move {
	    let mut sub_req = broker.subscribe_request();
	    sub_req.get().set_subscriber(client);
	    let mut t = sub_req.get().init_topics(topics.len() as u32);
	    for (n, topic) in topics.iter().cloned().enumerate() {
		t.set(n as u32, topic);
	    }

	    match sub_req.send().promise.await {
		Ok(x) => {
		    info!("Subscribed on nanobus");
		},
		Err(e) => {
		    error!("Failed to subscribe on nanobus: {:?}", e);
		},
	    }
	});

	local_set.spawn_local(rpc_system);
	

	Ok((local_set, rx))
    }
}

impl nanobus_capnp::subscriber::Server for CapnpClient {
    fn send(&mut self, params: nanobus_capnp::subscriber::SendParams, _results: nanobus_capnp::subscriber::SendResults) -> Promise<(), capnp::Error> {

	Promise::ok(())
    }
}

pub struct Server;

#[derive(Default)]
pub struct CapnpServer {
    subscribers: Vec<(nanobus_capnp::subscriber::Client, Vec<nanobus_capnp::Topic>)>,
}

impl nanobus_capnp::broker::Server for CapnpServer {
    fn subscribe(
        &mut self,
        params: nanobus_capnp::broker::SubscribeParams,
        mut results: nanobus_capnp::broker::SubscribeResults,
    ) -> Promise<(), capnp::Error> {
        let params = pry!(params.get());
        let caller = pry!(params.get_subscriber());
        let mut topics = vec![];

        for topic in pry!(params.get_topics()).iter() {
            topics.push(pry!(topic));
        }

        self.subscribers.push((caller, topics));

        Promise::ok(())
    }

    fn publish(
        &mut self,
        params: nanobus_capnp::broker::PublishParams,
        _results: nanobus_capnp::broker::PublishResults,
    ) -> Promise<(), capnp::Error> {
        let params = pry!(params.get());
        let topic = pry!(params.get_topic());
        let message = pry!(params.get_message());

        info!("{:?}: {}", topic, message);

        use futures::stream::futures_unordered::FuturesUnordered;
        use futures::FutureExt;
        use futures::StreamExt;
        let promises = self
            .subscribers
            .iter()
            .filter_map(|(subscriber, topics)| {
                if topics.contains(&topic) || topics.contains(&Topic::All) {
                    let mut req = subscriber.send_request();
                    req.get().set_message(message);
                    req.get().set_topic(topic);
                    return Some(req.send().promise.map(|_| ()));
                } else {
                    return None;
                }
            })
            .collect::<FuturesUnordered<_>>();
        let r = promises.collect::<Vec<_>>().map(|_| Ok(()));
        Promise::from_future(r)
    }
}

impl Server {
    pub async fn run(
        path: &str,
    ) -> Result<
        (
            impl futures::Future<Output = ()>,
            tokio::sync::mpsc::Sender<(nanobus_capnp::Topic, String)>,
        ),
        Error,
    > {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<(nanobus_capnp::Topic, String)>(16);

        let cps = CapnpServer::default();
        let broker: nanobus_capnp::broker::Client = capnp_rpc::new_client(cps);
        let socket = tokio::net::UnixListener::bind(path).map_err(Error::SocketError)?;

        let b = broker.clone();

        let handle_incoming = async move {
            loop {
                let (socket, remote) = socket.accept().await.map_err(Error::SocketError)?;
                info!("Connection from {:?}", remote);

                let (reader, writer) =
                    tokio_util::compat::TokioAsyncReadCompatExt::compat(socket).split();
                let network = capnp_rpc::twoparty::VatNetwork::new(
                    reader,
                    writer,
                    rpc_twoparty_capnp::Side::Server,
                    Default::default(),
                );
                let rpc_system =
                    capnp_rpc::RpcSystem::new(Box::new(network), Some(broker.clone().client));

                tokio::task::spawn_local(rpc_system);
            }

            Result::<(), Error>::Ok(())
        };

        let set = tokio::task::LocalSet::new();

        set.spawn_local(handle_incoming);
        set.spawn_local(async move {
            while let Some((topic, msg)) = rx.recv().await {
                debug!("Received message {:?}: {}", topic, msg);
                let mut req = b.publish_request();
                req.get().set_topic(topic);
                req.get().set_message(&msg);
                req.send()
                    .promise
                    .await
                    .expect("Failed to send message to broker");
            }
        });

        Ok((set, tx))
    }
}
