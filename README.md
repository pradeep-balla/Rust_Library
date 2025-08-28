# MainProject — TPM-backed Digital Signature Client 

A Windows-based system that uses the **TPM (Trusted Platform Module)** to sign JSON files with a **non-exportable RSA-2048 private key**.  
The signed payload is then sent to a server over **HTTP/gRPC**.  

The system ensures that the **private key never leaves the TPM**, while allowing the server to verify authenticity using the public key (from a certificate).  

---

## 📌 How It Works (Flow)

JSON File (data/person.json)
        |
        v
Java Client (reads JSON and calls Rust JNI)
        |
        v
Rust Library (secure_comm) - hash and sign
        |
        v
TPM (RSA-2048 Private Key) - NCryptSignHash
        |
        v
Signature Generated (output/signature.sig.b64)
        |
        v
Java Client (adds X-Signature and Thumbprint)
        |
        v
Server (receives request)
        |
        v
Verification (certificate chain and signature check)


1. **Provisioning**
   - Create an RSA-2048 key in the TPM (`Microsoft Platform Crypto Provider`).
   - Generate a certificate for that key (signed by your CA).
   - Install certificate into **CurrentUser\MY** store.

2. **Client Run**
   - Java reads input JSON (`data/person.json`).
   - Rust (via JNI):
     - Hashes payload (`SHA-256`).
     - Signs hash using **TPM-backed RSA private key** (`NCryptSignHash`).
     - Verifies signature locally using the public key.
     - Saves signature to `output/signature.sig.b64`.
   - Java sends HTTP request with headers:
     - `X-Signature`: Base64 signature
     - `X-Thumbprint`: Cert thumbprint

3. **Server (future stage)**
   - Resolves device certificate (from thumbprint or request).
   - Validates certificate chain (Device → Intermediate → Root CA).
   - Verifies the signature over the payload using the public key.

---

## 🔑 Why TPM?

- **Private key is non-exportable**: attackers cannot steal it.  
- **Certificate-based trust**: server verifies that the client key is part of a trusted CA chain.  
- **End-to-end security**: signed payloads cannot be modified without detection.  

---

## 📂 Project Structure

```
MainProject/
├── config/                       
│   ├── environment.properties    # Root config (thumbprint, paths for Rust)
│   └── environment.properties.example # For git
│
├── data/
│   └── person.json               # Sample payload to sign
│
├── output/
│   └── signature.sig.b64         # Signature output from Rust signer
│
├── logs/                         # Reserved for logs
│
├── secure_comm/                  # Rust crate (JNI lib)
│   ├── src/
│   │   ├── bindings.rs           # JNI exports (Java ↔ Rust bridge)
│   │   ├── tpmr.rs               # TPM signing + local verification
│   │   ├── https_client.rs       # Sends signed HTTP requests
│   │   ├── grpc_client.rs        # (Optional) gRPC client
│   │   └── config.rs             # Loads Rust-side config
│   └── Cargo.toml
│
├── java_test/                    # Java Maven project
│   ├── src/main/java/com/example/
│   │   ├── Main.java             # Loads Rust DLL, orchestrates signing + HTTP
│   │   ├── config/AppConfig.java # Reads application.properties
│   │   └── utils/FileUtils.java  # File helpers
│   ├── src/main/resources/
│   │   └── application.properties# Java-side config (paths, URL)
│   ├── config/environment.properties # Optional Java overrides
│   └── pom.xml
```

## What each part does

- config/: Single root config for the Rust signer. Critical keys:
  - CERT_THUMBPRINT: SHA-1 thumbprint of the TPM-backed cert in CurrentUser\\MY
  - PERSON_JSON_PATH: Usually `data/person.json` when running from repo root
  - SIGNATURE_OUTPUT_PATH: Usually `output/signature.sig.b64`
- secure_comm/: Rust JNI library that performs:
  - SHA-256 hash of payload, signing via NCryptSignHash with the TPM-backed RSA key
  - Local verification using the public key from the device certificate
  - HTTP request including `X-Signature` and `X-Thumbprint` headers
- java_test/: Java client that:
  - Loads the Rust DLL (`secure_comm_lib.dll`) via System.loadLibrary
  - Reads payload JSON (path from application.properties)
  - Invokes the Rust signer via JNI and sends the HTTP request


## Certificate Authority (how it fits)

- Device cert: public key bound to the TPM-held private key; installed on the client
- CA chain: Root → Intermediate → Device cert. The server trusts the Root (and Intermediate) and rejects chains that don’t anchor to it
- Verification on server: validate chain + expiry/revocation, recompute hash and verify signature using the device cert public key

