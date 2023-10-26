fn for_loops() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("{element}");
    }

    for element in (1..4).rev() {
        println!("{element}");
    }
}


fn multiple_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("cuenta: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}


fn loop_me() -> i32 {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    result
}


fn main() {
    let condition = false;
    let number = if condition {5} else {6};

    if number % 4 == 0 {
        println!("4");
    } else if number % 3 == 0 {
        println!("3");
    } else if number % 2 == 0 {
        println!("2");
    } else {
        println!("X");
    }

    let number = loop_me();
    println!("Número: {number}");

    multiple_loops();
    for_loops();
}
