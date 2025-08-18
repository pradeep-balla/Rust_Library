public class Main {
    static {
        // Using the absolute path to load the library
        String libraryPath = "C:\\Users\\pradeep\\Desktop\\MainProject\\secure_comm\\target\\debug\\secure_comm_lib.dll";
        System.load(libraryPath);
    }

    // Native method for generic HTTP requests
    private native String makeHttpRequestGeneric(String method, String url, String body);

    // Native method for HelloService gRPC request
    private native String makeGrpcRequest(String name);

    public static void main(String[] args) {
        Main app = new Main();
        String response;
        String jsonBody = "{\"name\":\"pradeep\", \"job\":\"developer\"}";

        // --- GET Request ---
        System.out.println("\n--- Making GET Request ---");
        response = app.makeHttpRequestGeneric("GET", "https://httpbin.org/get", "");
        System.out.println(response);

        // --- POST Request ---
        System.out.println("\n--- Making POST Request ---");
        response = app.makeHttpRequestGeneric("POST", "https://httpbin.org/post", jsonBody);
        System.out.println(response);

        // verify at the server end for https requests
        String webhookUrl = "https://webhook.site/c63c02a7-ae67-4e9f-aa66-a2a3e83da655";

        System.out.println("-----Making Get Request-----");
        response=app.makeHttpRequestGeneric("GET", webhookUrl, "");
        System.out.println(response);

        System.out.println("-----Making Post Request-----");
        response=app.makeHttpRequestGeneric("POST", webhookUrl, jsonBody);
        System.out.println(response);

        // --- gRPC HelloService Request ---
        System.out.println("\n--- Making gRPC HelloService Request from Rust ---");
        response = app.makeGrpcRequest("Pradeep");
        System.out.println("gRPC Response: " + response);
        

    }
}
