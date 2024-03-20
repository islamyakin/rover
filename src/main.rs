use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {

    let file_paths = vec!["", ""];
    let mut total_line_count = 0;

    for file_path in &file_paths {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut line_count = 0;
        for _line in reader.lines() {
            line_count += 1;
        }
        println!("Jumlah baris dalam file {}: {}", file_path, line_count);
        total_line_count += line_count;
    }

    println!("Jumlah total baris dalam semua file: {}", total_line_count);

    Ok(())
}
