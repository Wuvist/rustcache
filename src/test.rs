

fn longest<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    // {
    let string2 = String::from("xyz");
    result = longest(&string1, &string2);
    println!("The longest string is {}", string1);
    println!("The longest string is {}", string2);
    // }
    println!("The longest string is {}", result);
}
