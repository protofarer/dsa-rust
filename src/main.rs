fn main() {
    let mut x: Option<i32> = None;
    let y = x.take();
    println!("{:?}", y);
}
