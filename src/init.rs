use std::fs;
use std::io;

fn make_folder() -> io::Result<()> {
    fs::create_dir("~/.expend")?;
    Ok(())
}

fn make_file() -> io::Result<()> {
    let mut file = fs::File::create("~/.expend/track")?;
    Ok(())
}

fn read_file() -> Result<String, Box<dyn std::error::Error>> {
    let mut file = fs::File::open("~/.expend/track")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

