use std::fs;
use std::io;
use std::io::Read;
use directories::BaseDirs;

pub fn make_folder() -> io::Result<()> {
    if let Some(config_folder) = BaseDirs::new() {
        fs::create_dir(proj.home_dir().join(".xpend"));
    }
    
    Ok(())
}

pub fn make_file() -> io::Result<()> {
    if let Some(config_folder) = BaseDirs::new() {
        fs::File::create(proj.home_dir().join(".xpend"));
    }
    Ok(())
}

pub fn read_file() -> Result<String, Box<dyn std::error::Error>> {
    if let Some(config_folder) = BaseDirs::new() {
        let mut file = fs::File::open(config_folder.home_dir().join(".xpend").join("config"));
        let mut content = String::new();
        file.read_to_string(&mut content)?;
    }
    Ok(content)
}

