// `Vec<T>` is generic over the type `T`. In most cases, the compiler is able to
// infer `T`, for example after pushing a value with a concrete type to the vector.
// But in this exercise, the compiler needs some help through a type annotation.

fn main() {
    // TODO: Fix the compiler error by annotating the type of the vector
    // `Vec<T>`. Choose `T` as some integer type that can be created from
    // `u8` and `i8`.
    let mut numbers: Vec<i32> = Vec::new();

    // Don't change the lines below.
    let n1: u8 = 42;
    // 因为上面指明了Vec元素为i32，所以push方法的参数类型为i32
    // 所以编译器可以推断出into方法的类型转换的目标类型为i32
    // 同理 let x:i32 = n1.into() ：编译器根据前面的类型标注推断出目标类型i32
    // 如果去掉:i32，编译器无法推断
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
