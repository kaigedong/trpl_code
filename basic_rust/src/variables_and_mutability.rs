pub fn run() {
    println!("变量默认不可变");
    const MAX_POINTS: u32 = 100_000;

    let guess: u32 = "42".parse().expect("Not a number!");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;
    println!("five_hundred: {}", five_hundred);

    let a = [1, 2, 3, 4, 5, 6];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let b = [3; 5];

    println!("a: {:?}, b: {:?}", a, b);

    another_function();

    another_function2(7);

    let t = plus_one(8);
    println!("{}", t);

    // if 表达式
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    };

    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    // for
    let a = [10, 20, 30, 40, 50, 60];
    for element in a.iter() {
        println!("The value is: {}", element);
    }
}

fn another_function() {
    println!("Another function.")
}

fn another_function2(x: i32) {
    println!("Another function 2: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
