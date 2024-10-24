fn main(){
    let mut vec1 = Vec::new();
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    vec1.push(4);
    vec1.push(5);
    vec1.push(6);

    println!("{:?}", even_value(&vec1));
    println!("{:?}", vec1);
    even_value2(&mut vec1);
    println!("{:?}", vec1);
}

fn even_value2(vec: &mut Vec<i32>){
    let mut i = 0;

    while i<vec.len(){
        if vec[i]%2!=0{
            vec.remove(i);
        }
        else{
            i+=1;
        }
    }
}

fn even_value(vec: &Vec<i32>) -> Vec<i32>{
    let mut new_vec = Vec::new();

    for val in vec{
        if val%2==0{
            new_vec.push(*val);
        }
    }
    return new_vec;
}
