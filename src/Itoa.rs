use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn search_path_home(line: &str) -> bool {
    line.contains("Itoa")
}
fn search_home_login(line: &str) -> bool {
    line.contains("Atoa")
}

fn main() -> io::Result<()> {
    let start_time = Instant::now();

    let file_paths = vec!["", ""];

    let mut total_path_home_count = 0;
    let mut total_home_login_count = 0;

    for file_path in &file_paths {
        let file = File::open(&file_path)?;
        let reader = io::BufReader::new(file);

        let mut path_home_count = 0;
        let mut home_login_count = 0;

        for line in reader.lines() {
            let line = line?;
            if search_path_home(&line) {
                path_home_count += 1;
            }

            if search_home_login(&line) {
                home_login_count += 1;
            }
        }

        total_path_home_count += path_home_count;
        total_home_login_count += home_login_count;

    }

    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    println!("Jumlah total kata kunci a : {}", total_path_home_count);
    println!("Jumlah total kata kunci b : {}", total_home_login_count);
    println!("Waktu eksekusi program: {} detik", duration.as_secs());

    Ok(())
}
