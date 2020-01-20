use rand::Rng; // Rng是一个trait，它定义了随机数生成器应该实现的方法。
use std::cmp::Ordering;
use std::io;

// cargo doc --open 构建所有本地依赖提供的文档

fn main() {
    println!("Guessing the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // :: 表明是内联函数，是针对类型实现的，而不是类型的实例。
                                       // io的stdin关联函数，返回一个std::io::Stdin实例，
                                       // 代表终端标准输入句柄的类型。
                                       // read_line方法从标准输入句柄获取用户的输入，存入字符串中。
                                       // 最后用Result类型处理潜在的错误
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // String 实例需要去除开头和结尾的空白字符。字符串的parse方法将字符串解析成数字。
        println!("You guessed {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
