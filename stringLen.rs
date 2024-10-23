fn main(){
    let str: String = String::from("This is a string");
    println!("The length of the given string is: {}", get_str_len(&str));
}

fn get_str_len(str: &String) -> usize{
    str.chars().count()
}
