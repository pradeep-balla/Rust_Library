package com.example.utils;

import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

public class FileUtils {
    
    public static String readFile(String filePath) {
        try {
            Path path = Paths.get(filePath);
            if (!Files.exists(path)) {
                System.err.println("File not found: " + filePath);
                return "";
            }
            return new String(Files.readAllBytes(path));
        } catch (Exception e) {
            System.err.println("Error reading file " + filePath + ": " + e.getMessage());
            return "";
        }
    }
    
    public static boolean writeFile(String filePath, String content) {
        try {
            Path path = Paths.get(filePath);
            Files.createDirectories(path.getParent());
            Files.write(path, content.getBytes());
            return true;
        } catch (Exception e) {
            System.err.println("Error writing file " + filePath + ": " + e.getMessage());
            return false;
        }
    }
    
    public static boolean fileExists(String filePath) {
        return Files.exists(Paths.get(filePath));
    }
}
