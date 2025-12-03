use std::fs;
use std::path::Path;

pub fn load_input<P: AsRef<Path>>(path: P) -> String {
    let input = fs::read_to_string(&path);

    match input {
        Ok(content) => content.replace("\r\n", "\n"),
        Err(e) => panic!("Failed to read input file: {}\n -> {}", path.as_ref().display(), e),
    }
}

pub fn dbg_map(map: &[Vec<u8>]) {
    println!(
        "{}",
        map.iter()
            .map(|line| line.iter().map(|c| *c as char).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    )
}
