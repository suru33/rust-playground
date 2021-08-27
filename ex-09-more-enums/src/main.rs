fn plus_n(x: Option<i32>, y: i32) -> Option<i32> {
    match x {
        None => {
            if y == 0 {
                None
            } else {
                Some(y)
            }
        }
        Some(value) => Some(value + y),
    }
}

fn u8_match(v: u8) {
    match v {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        _ => println!("Something else"),
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let five = Some(5);

    println!("five: {:?}", five);
    println!("five + 3: {:?}", plus_n(five, 3));
    println!("None + 3: {:?}", plus_n(None, 3));
    println!("None + 0: {:?}", plus_n(None, 0));

    println!("------- Pattern matching -------");

    // pattern matching
    u8_match(1);
    u8_match(10);

    // The if let syntax lets you combine if and let into a less verbose way
    // to handle values that match one pattern while ignoring the rest

    let coin = Coin::Nickel;
    
    // match coin {
    //     Coin::Penny => println!("Hello penny"),
    //     _ => println!("meeehh!"),
    // } 
    
    // this is equal to

    if let Coin::Penny = coin {
        println!("Hello penny");
    } else {
        println!("meeehh!");
    }
}
