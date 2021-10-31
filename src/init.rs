use std::fs;
use std::io;
use std::io::Read;
use directories::BaseDirs;

pub fn make_folder() -> io::Result<()> {
    if let Some(config_folder) = BaseDirs::new() {
        fs::create_dir(config_folder.home_dir().join(".xpend"));
    }
    Ok(())
}


pub fn make_file() -> io::Result<()> {
    if let Some(config_folder) = BaseDirs::new() {
        fs::File::create(config_folder.home_dir().join(".xpend"));
    }
    Ok(())
}

pub fn read_file() -> Result<String, Box<dyn std::error::Error>> {
    let mut content = String::new();

    if let Some(config_folder) = BaseDirs::new() {
        let mut file = fs::File::open(config_folder.home_dir().join(".xpend").join("config"))?;
        file.read_to_string(&mut content)?;
    }
    Ok(content)
}

