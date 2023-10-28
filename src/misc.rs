use tokio::io::{self, AsyncReadExt};
use tokio::time::{self, Duration};

pub async fn read_stdin_with_timeout(timeout: Duration) -> Result<String, io::Error> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();

    let read_future = stdin.read_to_string(&mut buffer);
    let timeout_future = time::sleep(timeout);

    tokio::select! {
        result = read_future => {
            result?;
            Ok(buffer)
        }
        _ = timeout_future => {
            Err(io::Error::new(io::ErrorKind::TimedOut, 
                    format!("no input received, waited for {} ms", timeout.as_millis())))
        }
    }
}

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

