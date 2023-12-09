pub fn print_help_message() {
    eprintln!("Usage: usask-cba-calc [file_path]\n");
    eprintln!("Arguments:");
    eprintln!("    [file_path]  Path to file containing JSON data.\n");
    eprintln!("If no file path is provided, the program will read from stdin.\n");
    eprintln!("Options:");
    eprintln!("    -h, --help   Prints this message");
    eprintln!("    -s, --schema Creates a boilerplate schema");
    eprintln!("\n{}\n", env!("CARGO_PKG_DESCRIPTION"));
    eprintln!("usask-cba-calc v{}", env!("CARGO_PKG_VERSION"));
}

