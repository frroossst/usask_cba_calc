use std::{fs::File, io::Read};
use toml::Table;
 
pub fn read_and_parse_file(file_path: String) -> Table {
    let mut fobj = match File::open(file_path.clone()) {
        Ok(fobj) => fobj,
        Err(e) => panic!("Error opening file: {}", e),
    };

    let mut content = String::new();

    fobj.read_to_string(&mut content).expect("Error reading file");

    match content.parse::<Table>() {
        Ok(table) => return table,
        Err(e) => panic!("Error parsing file: {}", e),
    };
}
