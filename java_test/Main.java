
public class Main {
    static {
        // Using the absolute path to load the library
        String libraryPath = "C:\\Users\\pradeep\\Desktop\\MainProject\\secure_comm\\target\\debug\\secure_comm_lib.dll";
        System.load(libraryPath);
    }

    // Previous native methods
    private native String hello();

    // The new generic native method
    private native String makeHttpRequestGeneric(String method, String url, String body);

    public static void main(String[] args) {
        Main app = new Main();
        String response;
        String jsonBody = "{\"name\":\"pradeep\", \"job\":\"developer\"}";

        // --- GET Request ---
        System.out.println("\n--- Making GET Request ---");
        response = app.makeHttpRequestGeneric("GET", "http://httpbin.org/get", ""); // Body is empty for GET
        System.out.println(response);

        // --- POST Request ---
        System.out.println("\n--- Making POST Request ---");
        response = app.makeHttpRequestGeneric("POST", "http://httpbin.org/post", jsonBody);
        System.out.println(response);

        // --- PUT Request ---
        System.out.println("\n--- Making PUT Request ---");
        response = app.makeHttpRequestGeneric("PUT", "http://httpbin.org/put", jsonBody);
        System.out.println(response);

        // --- DELETE Request ---
        System.out.println("\n--- Making DELETE Request ---");
        response = app.makeHttpRequestGeneric("DELETE", "http://httpbin.org/delete", ""); // Body is empty for DELETE
        System.out.println(response);
    }
}