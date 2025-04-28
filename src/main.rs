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

    println!("\nCounting Files & directories:");
    let counts = two_tiernan_file_count::count_files_and_folders(dir_path); // returns a tuple struct (file_count, folder_count)
    println!("Number of files: {}", counts.0);
    println!("Number of Folders: {}", counts.1);
    

    println!("\nFiles sorted by size:");
    let sorted = three_vlad_sort_by_size::get_sorted_files_by_size(dir_path);
    for (path, size) in sorted {
        println!("{} - {} bytes", path.display(), size);
    }

    println!();

    match four_evan_common_file_type::get_most_common_file_type(dir_path) {
        Some((os_extension, occurrences)) => match os_extension.into_string() {
            Ok(extension) => {
                println!("Most common file extension: {extension} - {occurrences} occurrences")
            }
            Err(_) => println!("No files in that directory have extensions"),
        },
        None => println!("No files in that directory have extensions"),
    }
}
