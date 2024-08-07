// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?

// TODO: Fix the compiler error by updating the function signature.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 这个方法返回一个字符串slice，这是一个引用
    // 而这个引用它引用的内容在哪里？
    // 可能-1：局部变量，不可能，因为局部变量在函数作用域结束时会被销毁，随后返回的引用会变成悬垂引用，rust不允许出现悬垂引用；
    // 可能-2：字面量，字面量（如"ABC"的内容存放在RODATA区域，这是一个read-only区域，
    //        其生命周期与应用的生命周期一致，对于的生命周期标识为'static
    //        但在这里不是这种情况；
    // println!("{}, {}", x, y);
    // "ABC"
    // 可能-3：引用自参数中的一个或多个引用所指向的内容，这里有两个引用参数（x和y)，这种情况下编译器无法分辨
    //          到底是哪一个，所以需要显式标注生命周期

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
