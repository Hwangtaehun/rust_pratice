fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("five_hundred = {five_hundred}");
    println!("six_point_four = {six_point_four}");
    println!("one = {one}");
}
