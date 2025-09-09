package com.example.config;

import java.io.IOException;
import java.io.InputStream;
import java.util.Properties;

public class AppConfig {
    private static AppConfig instance;
    private Properties properties;
    
    private AppConfig() {
        loadProperties();
    }
    
    public static AppConfig getInstance() {
        if (instance == null) {
            instance = new AppConfig();
        }
        return instance;
    }
    
    private void loadProperties() {
        properties = new Properties();
        try (InputStream input = getClass().getClassLoader().getResourceAsStream("application.properties")) {
            if (input != null) {
                properties.load(input);
            } else {
                System.err.println("Warning: application.properties not found, using defaults");
                setDefaultProperties();
            }
        } catch (IOException e) {
            System.err.println("Warning: Could not load application.properties, using defaults: " + e.getMessage());
            setDefaultProperties();
        }
    }
    
    private void setDefaultProperties() {
        properties.setProperty("person.json.path", "../data/person.json");
        properties.setProperty("signature.output.path", "../output/signature.sig.b64");
        properties.setProperty("cert.thumbprint", "472F5B392D52BB109345DA5BD6649E3AE0AE91E0");
        properties.setProperty("webhook.url", "http://localhost:8080/");
    }
    
    public String getPersonJsonPath() {
        return properties.getProperty("person.json.path", "../data/person.json");
    }
    
    public String getSignatureOutputPath() {
        return properties.getProperty("signature.output.path", "../output/signature.sig.b64");
    }
    
    public String getCertThumbprint() {
        return properties.getProperty("cert.thumbprint", "472F5B392D52BB109345DA5BD6649E3AE0AE91E0");
    }
    
    public String getWebhookUrl() {
        return properties.getProperty("webhook.url", "http://localhost:8080/");
    }
}
