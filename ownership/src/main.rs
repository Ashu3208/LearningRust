fn main() {
    // Declaring the strings to be concatenated
    let string1: String = String::from("Hello there! ");
    let string2 : String = String::from("My name is Ashutosh");

    // Calling the concatenate_Strings function and passing the above declared string as the parameter
    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("{}",concatenated_string)
}

fn concatenate_strings(a: &str, b: &str) -> String{
    let mut result = String::new() ; // Declaring a muatble string variable
    result.push_str(a);
    result.push_str(b);
    return result;
}
