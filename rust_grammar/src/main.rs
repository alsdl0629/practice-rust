fn main() {
    let mut x = 5;
    println!("x is: {x}");
    x = 6;
    println!("x is: {x}");

    let y = 5;

    let y = 6;

    {
        let y = y * 2;
        println!("y is {y}");
    }

    println!("y is = {y}");

    let mut spaces = "   ";
    println!("{spaces}");
    spaces = spaces.len();
    println!("{spaces}");

    let a = 5;
    let b = 5;
    println!("{a} is {}", b);
}


