fn main(){
    let str: String = String::from("String");
    let first_a = find_first_a(&str);

    match first_a {
        Some(ind) => println!("'a' is present at index: {}", ind),
        None => println!("'a' is not present in the given string")
    }
}

fn find_first_a(str: &String) -> Option<i32>{
    for (index, chars) in str.chars().enumerate(){
        if chars=='a'{
            return Some(index as i32);
        }
    }
    return None;
}
