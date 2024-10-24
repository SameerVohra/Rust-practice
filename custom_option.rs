enum custom_option{
    Some(i32),
    None
}

fn main(){
    let str: String = String::from("this is a string");
    let is_present = find_first_a(&str);

    match is_present {
        custom_option::Some(val) => println!("'a' is present at index: {}", val),
        custom_option::None => println!("'a' is not present in the given string")
    }
}

fn find_first_a(str: &String) -> custom_option{
    for (index, chars) in str.chars().enumerate(){
        if chars=='a'{
            return custom_option::Some(index as i32);
        }
    }
    return custom_option::None;
}
