fn main() {
    let string1: String = String::from("Hello there! ");
    let string2 : String = String::from("My name is Ashutosh");
    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("{}",concatenated_string)
}

fn concatenate_strings(a: &str, b: &str) -> String{
    let mut result = String::new() ;
    result.push_str(a);
    result.push_str(b);
    return result;
}
