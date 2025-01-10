use std::fs;
use std::path::Path;

fn main() {
    // Get command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Use the provided path or default to current directory
    let default_path = String::from(".");
    let dir_path = args.get(1).unwrap_or(&default_path);

    println!("Scanning directory: {}", dir_path);

    // Read the directory
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    print_file_info(&path);
                }
            }
        }
        Err(e) => println!("Error reading directory: {}", e),
    }
}

fn print_file_info(path: &Path) {
    // Get basic file information
    if let Ok(metadata) = fs::metadata(path) {
        // Get filename
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        // Get file type
        let file_type = if metadata.is_dir() {
            "Directory"
        } else {
            path.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("unknown")
        };

        // Print information
        println!("\nFile: {}", filename);
        println!("Type: {}", file_type);
        println!("Size: {} bytes", metadata.len());

        // Print timestamps if available
        if let Ok(modified) = metadata.modified() {
            println!("Modified: {:?}", modified);
        }
    }
}
