use configparser::ini::Ini;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub struct Config {
    pub data: Ini,
    path: PathBuf,
}

impl Config {

    pub fn read() -> Result<Self, Box<dyn Error>> {
        let config_file = dirs::config_dir()
            .ok_or("Could not determine config directory")?
            .join("clenv")
            .join("config.ini");

        if !config_file.exists() {
            eprintln!("Configuration file not found. Run `clenv init`.");
            return Err("Missing configuration file".into());
        }

        let mut ini = Ini::new();
        ini.load(&config_file)?;

        Ok(Config { data: ini, path: config_file })
    }

    pub fn get(&self, section: &str, key: &str) -> Option<String> {
        self.data.get(section, key)
    }

    pub fn set(&mut self, section: &str, key: &str, value: &str) {
        self.data.set(section, key, Some(value.to_string()));
        self.save().expect("error saving new value to file");
    }

    // serializer
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        self.data.write(self.path.clone())?;
        Ok(())
    }
}