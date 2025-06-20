use configparser::ini::Ini;
use std::error::Error;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::env;

const CONFIG_DIR: &str = "clenv";
const CONFIG_FILE: &str = "config.ini";
// Ok so for some reason, when you use configparser, you need the set section to be "DEFAULT" then as you read the values back,
// you need them to be "default"...
const SECTION: &str = "DEFAULT";

#[derive(Clone)]
pub struct Config {
    ini: Ini,
}

impl Config {
    pub fn init() -> Result<Self, Box<dyn Error>> {
        fn prompt(label: &str) -> Result<String, io::Error> {
            print!("{}: ", label);
            io::stdout().flush()?;
            let mut buf = String::new();
            io::stdin().read_line(&mut buf)?;
            Ok(buf.trim().to_string())
        }
        
        fn prompt_path(label: &str) -> Result<String, io::Error> {
            print!("{}: ", label);
            io::stdout().flush()?;

            let mut buf = String::new();
            io::stdin().read_line(&mut buf)?;
            let input = buf.trim();

            let path = Path::new(input);
            if path.exists() {
                return Ok(input.to_string());
            }
            let cwd = env::current_dir()?;
            let full_path = cwd.join(input);

            Ok(full_path.to_string_lossy().to_string())
        }

        let name = prompt("Enter your name")?;
        let db = prompt_path("Enter database name")?;
        let private_key = prompt_path("Enter your name (used for key file)")?;
        let ns = prompt("Enter namespace")?;

        let mut ini = Ini::new();
        ini.set(SECTION, "name", Some(name));
        ini.set(SECTION, "db", Some(db));
        ini.set(SECTION, "private_key", Some(private_key));
        ini.set(SECTION, "ns", Some(ns));

        let config = Config { ini };
        config.save()?;

        Ok(config)
    }

    pub fn load() -> Result<Config, Box<dyn Error>> {
        let path = config_file_path()?;
        if !path.exists() {
            return Err(format!("Config file not found at {}", path.display()).into());
        }

        let mut ini = Ini::new();
        ini.load(&path)?;

        println!("Read config from: {}", path.display());
        Ok(Config { ini })
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.ini.get("default",key)
    }

    pub fn get_path(&self, key: &str) -> Option<PathBuf> {
        self.get(key).map(PathBuf::from)
    }

    pub fn list_all(&self) {
        let map = self.ini.get_map_ref();
        let mut found_entries = false;
        if let Some(section) = map.get("default") {
            if !section.is_empty() {
                found_entries = true;
                for (key, value_opt) in section {
                    if let Some(value) = value_opt {
                        println!("{} => {}", key, value);
                    }
                }
            }
        }

        if !found_entries {
            println!("No entries found in config!");
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.ini.set(SECTION, key, Some(value.to_string()));
        let _ = self.save();
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let config_path = config_file_path()?;
        let config_dir = config_path.parent().ok_or("Invalid config path")?;

        std::fs::create_dir_all(config_dir)?;
        self.ini.write(&config_path)?;

        println!("Config written to: {}", config_path.display());
        Ok(())
    }
}

fn config_file_path() -> Result<PathBuf, Box<dyn Error>> {
    let dir = dirs::config_dir().ok_or("Could not find config directory")?;
    Ok(dir.join(CONFIG_DIR).join(CONFIG_FILE))
}