use std::fs;
use std::path::Path;

pub struct Config {
    pub cert_thumbprint: String,
    //pub person_json_path: String,
    pub signature_output_path: String,
    pub tpm_store_name: String,
    pub tpm_algorithm: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Try to load from environment properties file first
        let config_path = Path::new("config/environment.properties");
        let mut config = Config {
            cert_thumbprint: "472F5B392D52BB109345DA5BD6649E3AE0AE91E0".to_string(),
            //person_json_path: "data/person.json".to_string(),
            signature_output_path: "output/signature.sig.b64".to_string(),
            tpm_store_name: "MY".to_string(),
            tpm_algorithm: "BCRYPT_SHA256_ALGORITHM".to_string(),
        };

        if config_path.exists() {
            if let Ok(contents) = fs::read_to_string(config_path) {
                for line in contents.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }
                    
                    if let Some((key, value)) = line.split_once('=') {
                        match key.trim() {
                            "CERT_THUMBPRINT" => config.cert_thumbprint = value.trim().to_string(),
                            //"PERSON_JSON_PATH" => config.person_json_path = value.trim().to_string(),
                            "SIGNATURE_OUTPUT_PATH" => config.signature_output_path = value.trim().to_string(),
                            "TPM_STORE_NAME" => config.tpm_store_name = value.trim().to_string(),
                            "TPM_ALGORITHM" => config.tpm_algorithm = value.trim().to_string(),
                            _ => {}
                        }
                    }
                }
            }
        }

        // Ensure output directory exists
        if let Some(output_dir) = Path::new(&config.signature_output_path).parent() {
            if !output_dir.exists() {
                fs::create_dir_all(output_dir)?;
            }
        }

        Ok(config)
    }

    pub fn get_cert_thumbprint(&self) -> &str {
        &self.cert_thumbprint
    }

    // pub fn get_person_json_path(&self) -> &str {
    //     &self.person_json_path
    // }

    pub fn get_signature_output_path(&self) -> &str {
        &self.signature_output_path
    }

    
}
