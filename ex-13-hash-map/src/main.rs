use std::collections::HashMap;

fn main() {
    let mut m1 = HashMap::new();
    m1.insert(String::from("red"), 100);
    m1.insert(String::from("blue"), -63);
    m1.insert(String::from("green"), 80);
    m1.insert(String::from("yellow"), 33);
    m1.insert(String::from("black"), 412);

    println!("m1 = {:?}", m1);

    // zip

    let keys = vec![
        String::from("red"),
        String::from("blue"),
        String::from("green"),
        String::from("yellow"),
        String::from("black"),
    ];

    let values = vec![100, 63, 80, 33, 412];

    let zip_map: HashMap<String, i32> = keys.into_iter().zip(values.into_iter()).collect();
    println!("zip_map = {:?}", zip_map);

    for (k, v) in &zip_map {
        println!("{} -> {}", k, v);
    }

    println!("zip_map(blue) = {:?}", zip_map.get("blue"));
    println!("zip_map(pink) = {:?}", zip_map.get("pink"));

    // upsert HashMap
    println!("m1 = {:?}", m1);
    m1.entry(String::from("pink")).or_insert(98);
    println!("m1 = {:?}", m1);

    // update a value
    *m1.get_mut("pink").unwrap() = 0;

    // update a value safe
    m1.get_mut("pink").map(|f| { *f = -87 });

    println!("m1 = {:?}", m1);

    // update a value safe
    // orange key not found
    let orange = m1.get_mut("orange");
    match orange {
        None => {
            println!("Orange not found");
        }
        Some(value) => {
            *value = -99;
        }
    }
}
