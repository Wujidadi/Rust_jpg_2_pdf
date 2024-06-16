use std::fs::{self, File};
use std::io::BufWriter;
use std::path::Path;

use clap::{Arg, Command};
use glob::glob;
use jpeg_to_pdf::JpegToPdf;

fn main() {
    let matches = Command::new("image_2_pdf")
        .arg(
            Arg::new("in_dir")
                .help("輸入目錄")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("out_path")
                .help("輸出路徑")
                .required(false)
                .index(2),
        )
        .get_matches();

    let in_dir = matches.get_one::<String>("in_dir").unwrap();
    let out_path = matches.get_one::<String>("out_path").map_or_else(|| {
        let dir_name = Path::new(in_dir)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        format!("{}.pdf", dir_name)
    }, |s| s.clone());

    let mut pdf = JpegToPdf::new();
    for entry in glob(&format!("{}/*.jpg", in_dir)).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                pdf = pdf.add_image(fs::read(path).unwrap());
            }
            Err(e) => println!("{:?}", e),
        }
    }
    let out_file = File::create(out_path).unwrap();
    pdf.create_pdf(&mut BufWriter::new(out_file)).unwrap();
}
