fn main() {
    print_labeled_measurement(5, 'V');
    let x = five();
    println!("El valor de x es {x}");
    let x = plus_one(x);
    println!("El valor de x es {x}");
}

fn print_labeled_measurement(value: i32, unit: char) {
    println!("Medidas: {value} {unit}")
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
