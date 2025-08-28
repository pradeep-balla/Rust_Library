use tonic::transport::{Channel, ClientTlsConfig};
use crate::proto; // use the proto module

pub async fn make_grpc_request(greeting: String) -> String {
    let tls = ClientTlsConfig::new().domain_name("grpcb.in");

    let channel = Channel::from_static("https://grpcb.in:9001")
        .tls_config(tls)
        .expect("TLS config failed")
        .connect()
        .await;

    match channel {
        Ok(channel) => {
            let mut client = proto::hello_service_client::HelloServiceClient::new(channel);

            let request = tonic::Request::new(proto::HelloRequest {
                greeting: Some(greeting.clone()), // proto2 optional
            });

            match client.say_hello(request).await {
                Ok(response) => response.into_inner().reply,
                Err(e) => format!("gRPC call failed: {}", e),
            }
        }
        Err(e) => format!("Failed to connect: {}", e),
    }
}
