fn main() {
    let x = 5;

    let x = x + 1;
    
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    /*
    let mut spaces = "   ";
    let spaces = spaces.len();
    */
}

//지역변수와 전역변수로 이해하면 됨