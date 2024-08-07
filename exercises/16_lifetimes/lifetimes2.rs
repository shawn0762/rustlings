// Don't change this function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: Fix the compiler error by moving one line.

    let string1 = String::from("long string is long");
    let result;
    // 可以把string2移到这里
    let string2 = String::from("xyz");
    {
        // 原来string2定义在这里
        // let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        // 或者把这一行移到这里
        // println!("The longest string is '{result}'");
    }
    println!("The longest string is '{result}'");
    // longest方法表明输入生命周期是x、y两个引用中生命周期较短的那一个，
    // 而返回值的生命周期与较短的生命周期一致。
    // string1的生命周期：从13行到main结束
    // string2的生命周期：19行到23行，因为它是局部作用域的变量，作用域在23行退出时变量被销毁
    // result的生命周期：从14行到main结束
    // 所以string1和string2这两个变量的生命周期重叠的部分就是19到23行；
    // 也就是'a的生命周期只到23行，这是不够的，因为result活到了main结束，生命周期比string2长
    // 反过来说就是string2活得不够久，编译不通过

    // 为什么不让编译？
    // 从引用的角度来看，string2在18行被销毁，如果result引用的是string2的内容，那在18行后result就
    // 变成了悬垂引用——引用了被回收的内存，rust不允许出现悬垂引用
}
