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
        properties.setProperty("webhook.url", "http://localhost:8080/");
    }
    
    public String getPersonJsonPath() {
        return properties.getProperty("person.json.path", "../data/person.json");
    }
    
    // TPM/signature settings are encapsulated on Rust side (secure_comm)
    
    public String getWebhookUrl() {
        return properties.getProperty("webhook.url", "http://localhost:8080/");
    }
}
