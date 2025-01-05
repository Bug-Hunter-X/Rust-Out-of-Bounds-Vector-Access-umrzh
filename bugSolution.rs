fn main() {    let mut vec = Vec::new();    vec.push(1);    vec.push(2);    vec.push(3);    let index = 2; // Valid index    if index < vec.len() {        println!("Value at index {}: {}", index, vec[index]);    } else {        println!("Index out of bounds");    }    // Alternatively, use the get() method for safer access
    match vec.get(5) {
        Some(value) => println!("Value at index 5: {}", value),
        None => println!("Index out of bounds")
    }
} 