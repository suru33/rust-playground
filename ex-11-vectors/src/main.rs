use std::alloc::Global;

fn main() {
    let v1: Vec<i32> = Vec::new();
    println!("{:?}", v1);

    let mut v2: Vec<i32> = Vec::new();

    v2.push(10);
    v2.push(-10);
    v2.push(1710);
    v2.push(-56);

    println!("v2 = {:?}", v2);
    let v2_pop = v2.pop();
    println!("Popped from v2: {:?}", v2_pop);
    println!("v2 = {:?}", v2);

    let v3 = vec![10, 22, 3, -92, 123];
    let v3_i100 = v3.get(100);
    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100'
    // println!("v3[100] = {:?}", v3[100]);
    println!("v3[100] = {:?}", v3_i100);

    // Iterating over the Values in a Vector
    for i in v3.iter() {
        println!("{}", i);
    }
    for (i, val) in v3.iter().enumerate() {
        println!("v3[{}] = {}", i, val);
    }

    // update vector over iterator
    println!("v2 = {:?}", v2);
    for i in &mut v2 {
        *i += 3;
    }
    println!("v2 = {:?}", v2);


    // different types of data
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("sheet row: {:?}", row);
}
