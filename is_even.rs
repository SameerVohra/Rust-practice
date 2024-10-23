fn main(){
    let a: i32 = 10;
    let res = iseven(a);

    match res{
        Ok(ans) => println!("{}", ans),
        Err(error) => println!("{}", error)
    }
}

fn iseven(a: i32) -> Result<String, String> {
    if a%2==0{
        return Ok("Number is even".to_string());
    }
    return Err("Number is not even".to_string());
}
