use configparser::ini::{Ini,WriteOptions};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use dirs::config_dir;

pub struct Config {
    db : PathBuf,
    ns : String,
    private_key : PathBuf
}

impl Config {
    pub fn read() -> Result<Self, Box<dyn Error>> {
        let config_dir = dirs::config_dir()
        .ok_or("Could not find configuration directory")?
            .join("clenv");

        let config_file = config_dir.join("config.ini");

        if !config_file.exists() {
            eprintln!("Configuration file not found. Creating one at {:?}", config_file);
            let mut cfg_vals = Ini::new();
            cfg_vals.set("default", "database", None);
            cfg_vals.set("default", "namespace", None);
            cfg_vals.set("default", "private_key", None);

            if !config_dir.exists() {
                fs::create_dir_all(&config_dir);
            }

            let mut write_options = WriteOptions::default();
            write_options.space_around_delimiters = true;
            write_options.multiline_line_indentation = 2;
            write_options.blank_lines_between_sections = 1;

            cfg_vals.pretty_write(config_file, &write_options);
            // fs::write(&config_file, b"# Default config\n")?;
            return Ok(Config {
                db,
                ns,
                private_key
            })
        }


        Ok(Config {
            db,
            ns,
            private_key
        })
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.get("default", key)
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.data.set("default", key, Some(value.to_string()));
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
