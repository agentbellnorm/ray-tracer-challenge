use std::fs::File;
use std::io::Write;

pub fn save_to_file(filename: &str, content: String) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())
}
