fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("x in the inner scope: {x}");
    }

    println!("x in the outer scope: {x}");
}
