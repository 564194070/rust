fn main() {
    let num = 6;

    // 条件必须是一个bool值，不是bool值会报错
    if num < 5 {
        println!("num < 5")
    } else if num < 3 {
        println!("num > 3")
    } else {
        println!("3 < num < 5")
    }

    let num1 = if num == 6 { 5 } else { 6 }

    // loop 无限循环
    loop {
        println! ("无限循环")
        // 结束循环
        break;
    }

    // while 循环
    while num < 10 {
        println!("while 循环中")
        let num = num + 1
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // for each循环
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for num in (1..4).rev() {
        println! ("{num}"!)
    }

}