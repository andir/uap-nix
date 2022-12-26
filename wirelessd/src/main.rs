use futures::TryStreamExt;

#[tokio::main]
async fn main() {
    let (connection, handle, _) = wl_nl80211::new_connection().unwrap();
    tokio::spawn(connection);

    let mut interface_handle = handle.interface().get().execute().await;

    let mut msgs = Vec::new();
    while let Some(msg) = interface_handle.try_next().await.unwrap() {
        msgs.push(msg);
    }
    assert!(!msgs.is_empty());
    for msg in msgs {
        println!("{:?}", msg);
    }
}
