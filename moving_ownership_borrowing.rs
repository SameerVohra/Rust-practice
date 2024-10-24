fn main(){
    let s1: String = String::from("Hello world");

    // function print_str is borrowind the string from s1 and is not the owner of the string.
    print_str(&s1);
    println!("{}", s1);
}

fn print_str(s: &String){
    println!("{}", s);
}
