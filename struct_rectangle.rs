struct Rect{
    height: u32,
    width: u32
}

impl Rect{
    fn area(&self) -> u32{
        self.height*self.width
    }

    fn peri(&self) -> u32{
        2*(self.height+self.width)
    }
}
fn main(){
    let rect1 = Rect{
        height: 10,
        width: 20
    };

    println!("Area of the rectangle with height: {} and width: {} is {}", rect1.height, rect1.width, rect1.area());
    println!("Perimeter of the rectangle with height: {} and width: {} is {}", rect1.height, rect1.width, rect1.peri());
}
