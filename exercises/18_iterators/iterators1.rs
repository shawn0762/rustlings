// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterators() {
        let my_fav_fruits = ["banana", "custard apple", "avocado", "peach", "raspberry"];

        // TODO: Create an iterator over the array.
        // let mut fav_fruits_iterator: std::array::IntoIter<&str, 5> = my_fav_fruits.into_iter();
        let mut fav_fruits_iterator = my_fav_fruits.iter();

        // 首先my_fav_fruits中的元素时字符串slice，是引用：&str
        // 调用iter方法后，Iter中的T也是&str
        // next方法返回的是Some(&T)，也就是&&str
        // 所以assert_eq!宏的第二个参数也必须是Some(&&str)，即Some(&"abc")
        assert_eq!(fav_fruits_iterator.next(), Some(&"banana"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"custard apple")); // TODO: Replace `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"avocado"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"peach")); // TODO: Replace `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"raspberry"));
        assert_eq!(fav_fruits_iterator.next(), None); // TODO: Replace `todo!()`
    }
}
