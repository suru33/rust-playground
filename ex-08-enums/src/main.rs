#[derive(Debug)]
enum IPAddressKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IPAddressKind2 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
struct IPAddress {
    kind: IPAddressKind,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
} // which is equal to but you can't pattern match over same type
// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn main() {
    println!("IPAddr v4: {:?}", IPAddressKind::V4);

    let home_address = IPAddress {
        kind: IPAddressKind::V4,
        address: String::from("127.0.0.1"),
    };

    let public_address = IPAddress {
        kind: IPAddressKind::V6,
        address: String::from("2001:db8:3333:4444:CCCC:DDDD:EEEE:FFFF"),
    };

    println!("home address {:#?}", home_address);
    println!("public address {:#?}", public_address);

    let home_address_k2 = IPAddressKind2::V4(String::from("192.168.1.1"));
    let public_address_k2 = IPAddressKind2::V6(String::from("2001:db8:3333:4444:CCCC:DDDD:EEEE:FFFF"));

    println!("home address {:?}", home_address_k2);
    println!("public address {:?}", public_address_k2);

    println!("Nickel value in cents: {}", Coin::Nickel.value_in_cents())
}
