pub fn read_file(file: &str) -> String {
    let current_dir = std::env::current_dir().expect("unable to get current directory");
    let file_path = current_dir.join("src/inputs/").join(file);

    std::fs::read_to_string(file_path).expect("can't read the contents of the file")
}

#[allow(dead_code)]
fn main() {}
