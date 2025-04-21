mod four_evan_common_file_type;
mod one_georgia_avg_size;
mod three_vlad_sort_by_size;
mod two_tiernan_file_count;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <directory_path>", args[0]);
        return;
    }

    let dir_path = Path::new(&args[1]);
    if !dir_path.is_dir() {
        eprintln!("Provided path is not a directory");
        return;
    }

    // println!("Average file size: {:.2} bytes", /*1_Georgia_AvgSize*/);

    // println!("Number of files: {}", /*2_Tiernan_FileCount*/);

    println!("\nFiles sorted by size:");
    let sorted = three_vlad_sort_by_size::get_sorted_files_by_size(dir_path);
    for (path, size) in sorted {
        println!("{} - {} bytes", path.display(), size);
    }

    // println!("\nMost common file type: {}", /*4_Evan_CommonFileType*/);
}
