package com.example;

import com.example.config.AppConfig;
import com.example.utils.FileUtils;

public class Main {
    static {
        // Load the native library using java.library.path (set by Maven exec)
        try {
            System.load("C:\\Users\\pradeep\\Desktop\\MainProject\\secure_comm\\target\\debug\\secure_comm_lib.dll");
        } catch (UnsatisfiedLinkError e) {
            System.err.println("Error loading native library: " + e.getMessage());
            System.err.println("If running manually, set -Djava.library.path=../secure_comm/target/debug or use System.load with full path.");
            System.exit(1);
        }
    }

    // Native method for generic HTTP requests
    private native String makeHttpRequestGeneric(String method, String url, String body);

    // Native method for HelloService gRPC request
    public native String makeGrpcRequest(String json, String serverUrl);

    public static void main(String[] args) {
        Main app = new Main();
        AppConfig config = AppConfig.getInstance();
        
        // Read person.json from the data directory
        String jsonBody = FileUtils.readFile(config.getPersonJsonPath());
        
        if (jsonBody.isEmpty()) {
            System.err.println("Error: Could not read person.json file from: " + config.getPersonJsonPath());
            System.err.println("Make sure the file exists in the data directory");
            return;
        }

        System.out.println("Loaded person.json successfully");
        System.out.println("Content: " + jsonBody);

        // Make POST request to webhook
        String webhookUrl = config.getWebhookUrl();
        //String webhookUrl = "http://localhost:8080/";
        System.out.println("----- Making POST Request to: " + webhookUrl + " -----");
        
        try {
            String response = app.makeHttpRequestGeneric("POST", webhookUrl, jsonBody);
            System.out.println("Response: " + response);
        } catch (Exception e) {
            System.err.println("Error making HTTP request: " + e.getMessage());
        }

        // Uncomment to test gRPC request
        String serverUrl = "http://localhost:50051"; // e.g. "http://localhost:50051"
        // String serverUrl = config.getGrpcServerUrl();
        String grpcResponse = app.makeGrpcRequest(jsonBody, serverUrl);
        System.out.println("gRPC Response: " + grpcResponse);

    }
}
