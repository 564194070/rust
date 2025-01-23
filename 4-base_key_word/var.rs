fn main() {

    // 不可修改变量
    let x = 5;
    println!("The value of x is: {x}");
    // x=6

    // 加了mut关键字，变量就可以改变了
    let mut y = 5;
    println!("The value of x is: {y}");
    y = 6;
    println!("The value of x is: {y}");


    // 常量默认不可变，加啥都不可变
    const TEST_NUM: u32 = 60 * 60

    // 隐藏变量
    // 隐藏变量有自己的生命周期，生命周期结束后变量回归原内容
    // 隐藏变量可以更新变量类型
    let spaces = "   ";
    let spaces = spaces.len();
}