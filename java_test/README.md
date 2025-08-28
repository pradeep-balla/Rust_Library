# Java Test Application

A Maven-based Java application for testing secure communication with the Rust TPM module.

## Project Structure

```
java_test/
├── pom.xml                           # Maven configuration
├── src/
│   └── main/
│       ├── java/
│       │   └── com/
│       │       └── example/
│       │           ├── Main.java     # Main application class
│       │           ├── config/
│       │           │   └── AppConfig.java  # Configuration management
│       │           └── utils/
│       │               └── FileUtils.java  # File utility functions
│       └── resources/
│           └── application.properties # Application configuration
└── target/                           # Compiled classes (auto-generated)
```

## Prerequisites

1. **Java 11 or higher**
2. **Maven 3.6 or higher**
3. **Rust toolchain** (for building the native library)

## Setup

1. **Build the Rust library first:**
   ```bash
   cd ../secure_comm
   cargo build
   ```

2. **Build the Java application:**
   ```bash
   cd ../java_test
   mvn clean compile
   ```

## Running the Application

### Option 1: Using Maven Exec Plugin
```bash
mvn exec:java
```

### Option 2: Using Maven Shade Plugin (creates executable JAR)
```bash
mvn clean package
java -jar target/java-test-1.0.0.jar
```

### Option 3: Manual Java execution
```bash
mvn compile
java --enable-native-access=ALL-UNNAMED -Djava.library.path=../secure_comm/target/debug -cp target/classes com.example.Main
```

## Configuration

Edit `src/main/resources/application.properties` to customize:

- `person.json.path`: Path to the person.json file
- `signature.output.path`: Where to save signature files
- `cert.thumbprint`: Certificate thumbprint for TPM operations
- `webhook.url`: Webhook URL for testing HTTP requests

## Features

- **✅ Maven Project Structure**: Proper package organization
- **✅ Configuration Management**: Properties-based configuration
- **✅ No Hardcoded Paths**: All paths are configurable
- **✅ Error Handling**: Proper error messages and fallbacks
- **✅ Native Library Integration**: Seamless Rust library integration
- **✅ Build Automation**: Maven handles compilation and dependencies

## Troubleshooting

### Native Library Issues
If you get "UnsatisfiedLinkError":
1. Make sure Rust library is built: `cargo build --manifest-path ../secure_comm/Cargo.toml`
2. Check that `secure_comm_lib.dll` exists in `../secure_comm/target/debug/`

### File Path Issues
If person.json is not found:
1. Check the path in `application.properties`
2. Ensure the file exists in the `../data/` directory
3. Verify file permissions

### Maven Issues
If Maven fails:
1. Check Java version: `java -version`
2. Check Maven version: `mvn -version`
3. Clean and rebuild: `mvn clean compile`
