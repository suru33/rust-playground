use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    println!("test-1.txt:\n{:?}", read_file_data(String::from("test-1.txt")));

    println!("test-2.txt:\n{:?}", read_file_data_2(String::from("test-2.txt")));
}

fn read_file_data(file_path: String) -> Result<String, io::Error> {
    let f = File::open(file_path);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut data = String::new();

    match f.read_to_string(&mut data) {
        Ok(_) => Ok(data),
        Err(e) => Err(e),
    }
}

// ? Operator
fn read_file_data_2(file_path: String) -> Result<String, io::Error> {
    let mut f = File::open(file_path)?;
    let mut data = String::new();
    f.read_to_string(&mut data)?;
    Ok(data)
}

fn read_file_data_3(file_path: String) -> Result<String, io::Error> {
    let mut data = String::new();
    File::open(file_path)?.read_to_string(&mut data)?;
    Ok(data)
}
