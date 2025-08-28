## Run (from repo root)

- Build Rust JNI library:
```
cargo build --manifest-path secure_comm/Cargo.toml // from MAINPROJECT directory
```

- Run JAVA 
cd java_test // getting errors when i try from the root dir
mvn clean compile 
mvn exec:java


## Troubleshooting

- DLL not found:
  - Build `secure_comm` first and ensure `secure_comm/target/debug/secure_comm_lib.dll` exists
  - Pass `-Djava.library.path=secure_comm/target/debug` when running Java
- person.json not found from repo root:
  - Set Java `person.json.path` to `data/person.json` (root-relative), or use `-Dexec.workingdir=java_test`
- TPM signing failed / key not found:
  - Ensure cert is in `Cert:\\CurrentUser\\My`, `HasPrivateKey=True`, thumbprint matches
  - TPM is initialized and key was created with Microsoft Platform Crypto Provider


## Configure

- Root Rust signer config (used by secure_comm/src/config.rs):
  - Edit `config/environment.properties` and set:
    - CERT_THUMBPRINT=<your SHA-1 thumbprint, no spaces>
    - PERSON_JSON_PATH=data/person.json
    - SIGNATURE_OUTPUT_PATH=output/signature.sig.b64
- Java client config:
  - Edit `java_test/src/main/resources/application.properties` and ensure from repo root:
    - person.json.path=data/person.json
    - signature.output.path=output/signature.sig.b64
    - cert.thumbprint=<same thumbprint>
    - webhook.url=<your endpoint or webhook.site URL>
