fn main() {
    let literal_string = "hello";
    let mut s = String::from("hello");
    println!("literal is {literal_string}");
    println!("String is {s}");
    s.push_str(", world!");
    println!("mutated String is {s}");

    let mut a = String::from("yoikes");
    let copy_to_a = a.clone();
    println!("a is {a}");
    println!("copy is {copy_to_a}");
    a = String::from("hello");
    println!("apllied reassingment to a");
    println!("a is {a}");
    println!("copy is {copy_to_a}");
}
