use cleanup::MySmartPointer;
use my_box::MyBox;

mod my_box;
mod cleanup;

fn main() {
    let b = Box::new(100);
    println!("box b = {}", b);

    let mb = MyBox::new(100);
    println!("my box: {:?}", mb);
    println!("my box: {}", *mb.value());

    let mb_str = MyBox::new(String::from("World"));
    hello(&mb_str);

    let msp = MySmartPointer {
        value: String::from("Hello")
    };

    println!("MySmartPointer value: {}", msp.value);
    // Case 1: without the drop statement
    // Case 2: with the drop statement
    drop(msp);
    println!("End of main");
}

/*
Case 1: all variables get dropped after the scope
MySmartPointer value: Hello
End of main
Dropping MySmartPointer: Hello

Case 2: drop the variable before it scope ends
MySmartPointer value: Hello
Dropping MySmartPointer: Hello
End of main
*/



fn hello(s: &str) {
    println!("Hello, {}", s);
}
