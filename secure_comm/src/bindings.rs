use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

use crate::tpmr::sign_json_with_tpm;
use crate::https_client::make_http_request;
use crate::grpc_client::make_grpc_request;
use crate::config::Config;
use crate::versionCheck::is_tpm2;

#[no_mangle]
pub extern "system" fn Java_com_example_Main_makeHttpRequestGeneric<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    java_method: JString<'a>,
    java_url: JString<'a>,
    java_body: JString<'a>,
) -> jstring {
    let method: String = env.get_string(&java_method).expect("Couldn't get method string").into();
    let url: String = env.get_string(&java_url).expect("Couldn't get url string").into();
    let body: String = env.get_string(&java_body).expect("Couldn't get body string").into();

    // Load configuration
    let config = match Config::new() {
        Ok(cfg) => cfg,
        Err(e) => {
            let s = env.new_string(format!("Configuration error: {}", e)).unwrap();
            return s.into_raw();
        }
    };

    let response_body = if is_tpm2() {
        // TPM 2.0 → sign and send signature + thumbprint
        let signature_b64 = match sign_json_with_tpm(
            body.as_bytes(),
            config.get_cert_thumbprint(),
            config.get_signature_output_path(),
        ) {
            Ok(sig) => sig,
            Err(e) => {
                let s = env.new_string(format!("TPM signing failed: {}", e)).unwrap();
                return s.into_raw();
            }
        };

        make_http_request(
            method,
            url,
            body,
            Some(signature_b64),
            Some(config.get_cert_thumbprint()),
        )
    } else {
        // TPM < 2.0 → skip signing and thumbprint
        make_http_request(method, url, body, None, None)
    };

    let output = env
        .new_string(response_body)
        .expect("Couldn't create Java string!");
    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_example_Main_makeGrpcRequest<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    java_json: JString<'a>,
    java_server_url: JString<'a>,
) -> jstring {
    // Get JSON + URL from Java
    let json_data: String = env.get_string(&java_json)
        .expect("Couldn't get JSON").into();
    let server_url: String = env.get_string(&java_server_url)
        .expect("Couldn't get server URL").into();

    // Load config (thumbprint etc.)
    let config = match Config::new() {
        Ok(cfg) => cfg,
        Err(e) => {
            let s = env.new_string(format!("Configuration error: {}", e)).unwrap();
            return s.into_raw();
        }
    };

    // Sign with TPM only if TPM2
    let (signature_b64, thumbprint) = if is_tpm2() {
        match sign_json_with_tpm(
            json_data.as_bytes(),
            config.get_cert_thumbprint(),
            config.get_signature_output_path(),
        ) {
            Ok(sig) => (sig, config.get_cert_thumbprint().to_string()),
            Err(e) => {
                let s = env.new_string(format!("TPM signing failed: {}", e)).unwrap();
                return s.into_raw();
            }
        }
    } else {
        ("".to_string(), "".to_string())
    };

    // Send gRPC request (metadata + payload) inside Tokio runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    let response = rt.block_on(make_grpc_request(
        json_data.into_bytes(), // payload bytes
        server_url,
        Some(signature_b64),
        Some(thumbprint),
    ));

    let output = env.new_string(response.unwrap_or_else(|e| e.to_string()))
        .expect("Couldn't create Java string!");
    output.into_raw()
}