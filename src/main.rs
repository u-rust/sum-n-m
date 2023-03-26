use std::io;
fn main() {
    println!("Приветствую, это программа может находит сумму чисел от n до m.");

    loop {
        println!("Выберите операцию, которую нужно выполнить:");
        println!("1. Найти сумму чисел от n до m (+)");
        println!("2. Выход (или любую другую цифру кроме 1)");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Ошибка чтения строки");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice >= 2 {
            println!("До свидания!");
            break;
        }else{
            println!("Введите n число: ");
            let mut num1 = String::new();
            io::stdin()
                .read_line(&mut num1)
                .expect("Ошибка чтения строки");

            let num1: u32 = num1.trim().parse().expect("Ожидалось целое число");

            println!("Введите m число: ");
            let mut num2 = String::new();
            io::stdin()
                .read_line(&mut num2)
                .expect("Ошибка чтения строки");

            let num2: u32 = num2.trim().parse().expect("Ожидалось неотрицательное целое число");

            if num1 > num2 {
                println!("m число должно быть больше чем n");

                println!("Введите m число: ");
                let mut _num2 = String::new();
                io::stdin()
                    .read_line(&mut _num2)
                    .expect("Ошибка чтения строки");

                let _num2: u32 = _num2.trim().parse().expect("Ожидалось неотрицательное целое число");
                let result = sum(num1, _num2);
                println!("Сумма чисел от {} до {} = {}", num1, _num2, result);
            }else {
                let result = sum(num1, num2);
                println!("Сумма чисел от {} до {} = {}", num1, num2, result);
            }


        }
    }
}
fn sum(num1: u32, num2: u32) -> u32 {
    let mut result = 0;
    for i in num1..=num2 {
        result += i;
    }
    result
}