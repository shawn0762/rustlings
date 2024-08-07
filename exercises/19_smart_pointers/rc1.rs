// In this exercise, we want to express the concept of multiple owners via the
// `Rc<T>` type. This is a model of our solar system - there is a `Sun` type and
// multiple `Planet`s. The planets take ownership of the sun, indicating that
// they revolve around the sun.

use std::rc::Rc;

#[derive(Debug)]
struct Sun;

#[allow(dead_code)]
#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {self:?}!");
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc1() {
        // 引用计数智能指针
        // 一般情况下所有权都是明确的，可以准确知道哪个变量拥有哪个值，
        // 但在某些情况下，单个值可能会有多个所有者，例如图的多条边会执行一个节点
        // 为了启用多所有权，需要显式使用Rc<T>，如果某个值有0个引用，则代表没有任何有效引用，可以被清理
        // 实例化时引用计数为1，每clone一次，计数+1，每当clone出来的引用被回收，计数-1
        let sun = Rc::new(Sun);
        println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

        let mercury = Planet::Mercury(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 2 references
        mercury.details();

        let venus = Planet::Venus(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 3 references
        venus.details();

        let earth = Planet::Earth(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 4 references
        earth.details();

        let mars = Planet::Mars(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 5 references
        mars.details();

        let jupiter = Planet::Jupiter(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 6 references
        jupiter.details();

        // TODO
        let saturn = Planet::Saturn(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 7 references
        saturn.details();

        // TODO
        let uranus = Planet::Uranus(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 8 references
        uranus.details();

        // TODO
        let neptune = Planet::Neptune(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 9 references
        neptune.details();

        assert_eq!(Rc::strong_count(&sun), 9);

        drop(neptune);
        println!("reference count = {}", Rc::strong_count(&sun)); // 8 references

        drop(uranus);
        println!("reference count = {}", Rc::strong_count(&sun)); // 7 references

        drop(saturn);
        println!("reference count = {}", Rc::strong_count(&sun)); // 6 references

        drop(jupiter);
        println!("reference count = {}", Rc::strong_count(&sun)); // 5 references

        drop(mars);
        println!("reference count = {}", Rc::strong_count(&sun)); // 4 references

        // TODO
        drop(earth);
        println!("reference count = {}", Rc::strong_count(&sun)); // 3 references

        // TODO
        drop(venus);
        println!("reference count = {}", Rc::strong_count(&sun)); // 2 references

        // TODO
        drop(mercury);
        println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

        assert_eq!(Rc::strong_count(&sun), 1);
    }
}
