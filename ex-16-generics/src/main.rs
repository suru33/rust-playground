fn main() {
    let v1: Vec<i32> = vec![100, -932, 12, 4, 0, 123];

    println!("largest in {:?} is {}", v1, largest(&v1));
    println!("largest in {:?} is {}", v1, largest_2(&v1));
}

fn largest(vector: &Vec<i32>) -> i32 {
    let mut rv = vector[0];

    for i in vector.iter() {
        if *i > rv {
            rv = *i;
        }
    }

    rv
}

fn largest_2<T: std::cmp::PartialOrd>(vector: &Vec<T>) -> &T {
    let mut rv = vector.get(0).unwrap();

    for i in vector.iter() {
        if *i > *rv {
            rv = i;
        }
    }

    rv
}