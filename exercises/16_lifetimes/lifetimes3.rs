// Lifetimes are also needed when structs hold references.

// TODO: Fix the compiler errors about the struct.
struct Book<'a> {
    // 生命周期'a表明这两个字段所引用的内容的生命周期不能比Book短
    // 否则Book实例可能在某个时刻引用不存在的内存（已被回收的内存）
    author: &'a str,
    title: &'a str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
