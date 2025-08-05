use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use tokio::runtime::Runtime;

#[unsafe(no_mangle)]
pub extern "system" fn Java_Main_hello<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
) -> jstring {
    let output = env
        .new_string("Connection successful!")
        .expect("Couldn't create Java string!");
    output.into_raw()
}

// Our new generic function for making different types of HTTP requests
#[unsafe(no_mangle)]
pub extern "system" fn Java_Main_makeHttpRequestGeneric<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    java_method: JString<'a>,
    java_url: JString<'a>,
    java_body: JString<'a>,
) -> jstring {
    // Convert Java strings to Rust strings
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
            _ => return "Unsupported HTTP method".to_string(), // Handle unsupported methods
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