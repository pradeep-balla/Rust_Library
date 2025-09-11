# MainProject — TPM-backed Digital Signature Client 

A Windows-based system that uses the **TPM (Trusted Platform Module)** to sign JSON files with a **non-exportable RSA-2048 private key**.
The signed payload is then sent to a server over **HTTP or gRPC**.

The system ensures that the **private key never leaves the TPM**, while allowing the server to verify authenticity using the public key (from a certificate).

---

## 📌 How It Works (Flow)

JSON File (data/person.json)
        |
        v
Java Client (reads JSON and calls Rust JNI)
        |
        v
Rust Library (secure_comm) - TPM detect + hash and sign
        |
        v
TPM (RSA-2048 Private Key) - NCryptSignHash
        |
        v
Signature Generated (output/signature.sig.b64)
        |
        v
Client Request
  - HTTP: headers X-Signature, X-Thumbprint
  - gRPC: metadata signature, thumbprint; payload as bytes
        |
        v
Server (receives request) → Verification (certificate chain and signature check)

1. **Provisioning**
   - Create an RSA-2048 key in the TPM (`Microsoft Platform Crypto Provider`).
   - Generate a certificate for that key (signed by your CA).
   - Install certificate into **CurrentUser\\MY** store.

2. **Client Run**
   - Java reads input JSON (`data/person.json`).
   - Rust (via JNI):
     - Detects TPM version (TPM 2.0 supported) and signs accordingly.
     - Hashes payload (`SHA-256`).
     - Signs hash using **TPM-backed RSA private key** (`NCryptSignHash`).
     - Saves signature to `output/signature.sig.b64`.
   - Request options:
     - HTTP: sends `X-Signature` and `X-Thumbprint` headers
     - gRPC: sends metadata `signature` and `thumbprint` with a bytes payload

3. **Server**
   - Resolves device certificate (from thumbprint or provided cert).
   - Validates certificate chain (Device → Intermediate → Root CA).
   - Verifies the signature over the payload using the public key.

---

## 🔑 Why TPM?

- **Private key is non-exportable**: attackers cannot steal it.
- **Certificate-based trust**: server verifies that the client key is part of a trusted CA chain.
- **End-to-end security**: signed payloads cannot be modified without detection.

---

## 📂 Project Structure (updated)

```
MainProject/
├── data/
│   └── person.json                     # Sample payload to sign
│
├── output/
│   └── signature.sig.b64               # Signature output from Rust signer
│
├── logs/                               # Reserved for logs
│
├── secure_comm/                        # Rust crate (JNI lib)
│   ├── build.rs                        # Proto generation (vendored protoc)
│   ├── Cargo.toml
│   ├── config/
│   │   └── environment.properties      # TPM/signature/thumbprint (source of truth)
│   ├── proto/
│   │   └── verifier.proto              # gRPC proto (client side)
│   └── src/
│       ├── bindings.rs                 # JNI exports (Java ↔ Rust bridge)
│       ├── grpc_client.rs              # gRPC client (metadata + bytes payload)
│       ├── https_client.rs             # HTTP client (X-Signature, X-Thumbprint)
│       ├── tpmr.rs                     # TPM signing via NCrypt
│       ├── versionCheck.rs             # TPM 2.0 detection
│       └── config.rs                   # Loads secure_comm/config/environment.properties
│
├── java_test/                          # Java example app
│   ├── src/main/java/com/example/
│   │   ├── Main.java                   # Loads Rust DLL, orchestrates gRPC/HTTP
│   │   └── config/AppConfig.java       # App-level settings (no TPM values)
│   ├── src/main/resources/
│   │   └── application.properties      # App-only config (e.g., person path, URLs)
│   ├── config/environment.properties   # Optional app overrides (no TPM values)
│   └── pom.xml
│
└── README.md
```

Notes:
- TPM/signature/thumbprint settings live exclusively in `secure_comm/config/environment.properties`.
- The root `config/` folder is no longer required and has been removed.
---

## 🚀 Build and Run

Prereqs: Rust toolchain, Java (JDK), Go (for sample gRPC server if used), and protoc plugins on the Go side.

1) Build Rust JNI library
```
cd secure_comm
cargo build
```
This will also generate Rust gRPC stubs from `proto/verifier.proto` using vendored `protoc`.

2) Run Go gRPC server (in sibling folder, not part of this repo’s build)
- Ensure your Go server is running (default `:50051`) and matches the same `verifier.proto`.

3) Run Java example
- Ensure `secure_comm/target/debug/secure_comm_lib.dll` path is valid in `Main.java` (or set `-Djava.library.path`).
- Start the app from `java_test` (via your IDE or Maven exec). It will:
  - Read `data/person.json`
  - Call Rust via JNI
  - Send gRPC request with metadata `signature` and `thumbprint` and bytes payload

---

## 🧩 gRPC Contract (client)
- Service: `Verifier.Verify(PayloadBytes) returns (VerifyResponse)`
- Metadata:
  - `signature`: base64-encoded signature
  - `thumbprint`: SHA-1 certificate thumbprint
- Message types:
```
message PayloadBytes { bytes data = 1; }
message VerifyResponse { string result = 1; }
```

---

## 📎 Notes & Tips
- If you need HTTP instead of gRPC, use `https_client.rs` which adds headers `X-Signature` and `X-Thumbprint`.
- Keep all TPM-related configuration inside `secure_comm/config/` to avoid leakage across projects.
- For TLS gRPC, enable certificates on both sides and update the Rust client transport settings.

