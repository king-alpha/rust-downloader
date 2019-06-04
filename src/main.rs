extern crate curl;

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::time::Instant;

use curl::easy::Easy;

fn main() {
    // const MEGABYTES: f64 = (1024.0 * 1024.0);

    println!("Enter the full url to the file >>");

    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("Failed to read url");

    println!("\nEnter the file name >>");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read file name");

    name = name.trim().to_string();

    let mut easy = Easy::new();

    easy.url(url.trim()).unwrap();
    easy.progress(true).unwrap();

    easy.progress_function(|dltotal, dlnow, _, _| {
        print!("Progress : {}%\r", ((dlnow / dltotal) * 100.0) as u32);
        return true;
    })
    .unwrap();

    File::create(&name).unwrap();

    let start = Instant::now();

    easy.write_function(move |data| {
        let mut f = OpenOptions::new().append(true).open(&name).unwrap();

        f.write_all(data).unwrap();
        f.flush().unwrap();
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();

    println!("\ncompleted within {:#?}", start.elapsed());
}
