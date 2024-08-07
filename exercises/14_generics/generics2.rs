use std::{clone, fmt::Error};

// This powerful wrapper provides the ability to store a positive integer value.
// TODO: Rewrite it using a generic so that it supports wrapping ANY type.
struct Wrapper<T> {
    value: T,
}

// TODO: Adapt the struct's implementation to be generic over the wrapped value.
// 为什么Wrapper后面还要标上<T>？因为rust可以根据不同类型为泛型结构体“额外”实现一些成员方法
// 如impl Wrapper<i32> {} 等等
impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }

    // fn value(&self) -> T {
    // 在这里self是一个引用，rustc不允许通过引用移动所有权
    // self.value
    // }
}

// T=i32时，Wrapper可以使用更多成员方法
impl Wrapper<i32> {
    fn value2(&self) -> &i32 {
        &self.value
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
