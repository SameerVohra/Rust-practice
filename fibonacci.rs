fn main(){
    let a: i32 = 3;
    let ans: i64 = find_fib(a);
    println!("{}th fibonacci number is: {}", a, ans);
}

fn find_fib(a: i32) -> i64 {
    if a==0 {
        return 0;
    }
    if a==1{
        return 1;
    }
    if a==2{
        return 1;
    }

    return find_fib(a-1)+find_fib(a-2);
}
