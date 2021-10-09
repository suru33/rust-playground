use std::str::FromStr;

pub fn read_arg<T>(arg: Option<String>, pos: u8) -> T
where
    T: FromStr,
{
    match arg {
        Some(arg) => match arg.parse::<T>() {
            Ok(value) => value,
            Err(_) => panic!("Not a valid input for Arg {}", pos),
        },
        None => panic!("Arg {} not found", pos),
    }
}
