use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::path::Path;

/// Gets the most common file type (meaning extension) in the given directory and all of its subdirectories
///
/// > **Note**: If two file extensions have the same rate of occurrence, the function arbitrarily
/// > picks one of the two.
///
/// #Examples
///
/// ```
/// let directory = Path::new("./directory/with/lots/of/rust/files/");
/// let rust_extension = OsString::from("rs");
/// let number_of_rust_files = 16;
/// assert_eq!(get_most_common_file_type(directory), Some((rust_extension, number_of_rust_files)));
/// ```
pub fn get_most_common_file_type(dir: &Path) -> Option<(OsString, u64)> {
    let mut extensions: HashMap<OsString, u64> = HashMap::new();
    process_extensions(dir, &mut extensions);
    let mut most_common: Option<(OsString, u64)> = None;
    for (ext, &num) in extensions.iter() {
        most_common = Some(match most_common {
            Some((old_ext, old_num)) => {
                if num > old_num {
                    (ext.clone(), num)
                } else {
                    (old_ext, old_num)
                }
            }
            None => (ext.clone(), num),
        })
    }
    most_common
}

fn process_extensions(dir: &Path, extensions: &mut HashMap<OsString, u64>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                process_extensions(&entry.path(), extensions);
            } else {
                if let Some(extension) = path.extension() {
                    let count = extensions.entry(extension.to_os_string()).or_insert(0);
                    *count += 1;
                }
            }
        }
    }
}
