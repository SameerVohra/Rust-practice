use std::fs;
fn main(){
    let file_content = fs::read_to_string("./tem.txt");

    match file_content {
        Ok(val)=>{
            println!("The content of the file is: {}", val);
        },
        Err(error)=>println!("Error reading the file: {}", error)
    }
}
