use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use tokio::runtime::Runtime;

pub mod hello {
    tonic::include_proto!("hello"); // package hello from hello.proto
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_Main_makeHttpRequestGeneric<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    java_method: JString<'a>,
    java_url: JString<'a>,
    java_body: JString<'a>,
) -> jstring {
    let method: String = env.get_string(&java_method).expect("Couldn't get method string").into();
    let url: String = env.get_string(&java_url).expect("Couldn't get url string").into();
    let body: String = env.get_string(&java_body).expect("Couldn't get body string").into();

    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    let response_body = rt.block_on(async {
        let client = reqwest::Client::new();
        let result = match method.to_uppercase().as_str() {
            "GET" => client.get(&url).send().await,
            "POST" => client.post(&url).header("Content-Type", "application/json").body(body).send().await,
            "PUT" => client.put(&url).header("Content-Type", "application/json").body(body).send().await,
            "DELETE" => client.delete(&url).send().await,
            _ => return "Unsupported HTTP method".to_string(),
        };

        match result {
            Ok(response) => response.text().await.unwrap_or_else(|e| e.to_string()),
            Err(e) => e.to_string(),
        }
    });

    let output = env
        .new_string(response_body)
        .expect("Couldn't create Java string!");
    output.into_raw()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_Main_makeGrpcRequest<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    java_greeting: JString<'a>,
) -> jstring {
    let greeting: String = env.get_string(&java_greeting).expect("Couldn't get greeting").into();

    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    let response_message = rt.block_on(async {
        // TLS for grpcb.in
        let tls = tonic::transport::ClientTlsConfig::new()
            .domain_name("grpcb.in");

        let channel = tonic::transport::Channel::from_static("https://grpcb.in:9001")
            .tls_config(tls)
            .expect("TLS config failed")
            .connect()
            .await;

        match channel {
            Ok(channel) => {
                let mut client = hello::hello_service_client::HelloServiceClient::new(channel);

                let request = tonic::Request::new(hello::HelloRequest {
                    greeting: Some(greeting.clone()), // proto2 optional
                });

                match client.say_hello(request).await {
                    Ok(response) => response.into_inner().reply,
                    Err(e) => format!("gRPC call failed: {}", e),
                }
            }
            Err(e) => format!("Failed to connect: {}", e),
        }
    });

    let output = env.new_string(response_message).expect("Couldn't create Java string!");
    output.into_raw()
}
