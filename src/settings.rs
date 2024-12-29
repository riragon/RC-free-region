use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

#[derive(Debug, Clone)]
pub struct Settings {
    pub rc_path: String,
    pub prefix: String,
    pub margin: u8,
    pub number: u32,
}

pub fn load_settings(path: &Path) -> Settings {
    let mut s = Settings {
        rc_path: r"C:\Program Files\Capturing Reality\RealityCapture\RealityCapture.exe".to_string(),
        prefix: "myrcbox".to_string(),
        margin: 3,
        number: 1,
    };

    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for line_result in reader.lines() {
            if let Ok(line) = line_result {
                if let Some((key, val)) = line.split_once('=') {
                    let key = key.trim();
                    let val = val.trim();
                    match key {
                        "rc_path" => s.rc_path = val.to_string(),
                        "prefix" => s.prefix = val.to_string(),
                        "margin" => s.margin = val.parse().unwrap_or(3),
                        "number" => s.number = val.parse().unwrap_or(1),
                        _ => {}
                    }
                }
            }
        }
    } else {
        println!("[INFO] settings.txt not found => using defaults.");
    }
    s
}

pub fn save_settings(path: &Path, s: &Settings) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    writeln!(f, "rc_path={}", s.rc_path)?;
    writeln!(f, "prefix={}", s.prefix)?;
    writeln!(f, "margin={}", s.margin)?;
    writeln!(f, "number={}", s.number)?;
    Ok(())
}
