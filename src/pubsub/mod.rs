use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::web::Bytes;
use actix_web_lab::__reexports::futures_util::StreamExt;
use google_cloud_pubsub::client::Client;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;


pub(crate) async fn publish_opt_file(bytes: Bytes) -> HttpResponse {
    // let mut bytes = web::BytesMut::new();
    // while let Some(item) = req.next().await {
    //     bytes.extend_from_slice(&item.expect("Failed creating bytes"));
    // }

    let client = Client::default().await.unwrap();

    // Create topic.
    let topic = client.topic("opt_files");
    if !topic.exists(None, None).await.expect("Failed checking topic") {
        topic.create(None, None, None).await.expect("Failed creating topic");
    }

    // Start publisher.
    let mut publisher = topic.new_publisher(None);

    // Publish message.
    tokio::spawn(async move {
        let msg = PubsubMessage { data: bytes.to_vec(), ..Default::default() };

        // Send a message. There are also `publish_bulk` and `publish_immediately` methods.
        let awaiter = publisher.publish(msg).await;

        // The get method blocks until a server-generated ID or an error is returned for the published message.
        awaiter.get(None).await.expect("Failed publishing.");

        // Wait for publishers in topic finish.
        publisher.shutdown().await;
    });

    HttpResponse::Ok().finish()
}
