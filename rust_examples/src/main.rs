// Interesting reference; https://www.youtube.com/watch?v=TJTDTyNdJdY
#![allow(unused)]
use std::{ops::Add, f32::consts::PI, thread, sync::{Arc, Mutex}};

fn use_ownership() {
    struct SomeStruct {
        number: i32
    }

    fn print_value (my_struct: &SomeStruct) {
        println!("Value: {}", my_struct.number);
    }

    let my_struct = SomeStruct { number: 3 };
    
    // print_value(my_struct);
    // print_value(my_struct); -> error if print_value(my_struct: SomeStruct)
    // calling this method twice will generate a compile error
    // this is because the ownership of the struct is passed to the function
    // this happens because rust does not have a garbage collector

    // FIX 1 - DANGEROUS
    // Add #[derive(Clone)] in the beginning of the file
    // When calling the first time we call my_struct.clone() but this creates a
    // clone of the object, not the reference

    // FIX 2 - DANGEROUS
    // Implement the Copy trait
    // Add #[derive(Copy)] in the beginning of the file
    // Implicitly it is copying the struct

    // FIX 3
    // Create a read only reference
    print_value(&my_struct);
    print_value(&my_struct);

    // If we want to change the value we need a mutable references
    // See function use_mutable_references()

}

fn use_mutable_references() {
    struct SomeStruct {
        number: i32
    }

    fn print_value (my_struct: &SomeStruct) {
        println!("Value: {}", my_struct.number);
    }

    // in order to change the value we cannot just pass &SomeStruct because that would
    // be a read-only reference, thus we use &mut
    fn change_number(mut my_struct: &mut SomeStruct, number: i32) {
        my_struct.number = number;
    }

    let mut my_struct = SomeStruct { number: 3 };

    print_value(&my_struct);
    change_number(&mut my_struct, 4);
    print_value(&my_struct);

}

fn use_generics() {   
    fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
        return x + y;
    }
    
    println!("5 + 4 = {}", get_sum_gen(5, 4));
    println!("5.2 + 4.6 = {}", get_sum_gen(5.2, 4.6));
}

fn use_traits() {
    trait Shape {
        fn new(lenght: f32, width: f32) -> Self;
        fn perimeter(&self) -> f32;
    }
    
    struct Rectangle {length: f32, width: f32}
    struct Circle {radius: f32}

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle{length, width};
        }

        fn perimeter(&self) -> f32 {
            return (self.length + self.width) * 2.0;
        }
    }

    impl Shape for Circle {
        fn new(radius: f32, _width: f32) -> Circle {
            return Circle{radius};
        }

        fn perimeter(&self) -> f32 {
            return self.radius / 2.0 * PI * 2.0;
        }
    }
    
    let rec1: Rectangle = Shape::new(50.0, 5.0);
    let rec2: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 0.0); // the second argument will be ignored

    println!("Rec 1 Perimeter = {}", rec1.perimeter());
    println!("Rec 2 Perimeter = {}", rec2.perimeter());
    println!("Circ Perimeter = {}", circ.perimeter());
}

fn use_closures() {
    fn use_fn<T>(a: i32, b: i32, func: T) -> i32
    where T: Fn(i32, i32) -> i32 {
        func(a, b)
    }

    println!("5 + 4 = {}", use_fn(5, 4, |a: i32, b: i32| a+b));

    let prod = |a: i32, b: i32| a*b;

    println!("5 * 4 = {}", use_fn(5, 4, prod));
}

fn use_smart_pointers() {
    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {left: None, right: None, key}
        }

        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }

        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    let node1 = TreeNode::new(24);
    let node2 = TreeNode::new(36);

    let root = TreeNode::new(12)
        .left(node1)
        .right(node2);

    println!("Root Node = {}", root.key);

    match root.left {
        Some(e) => {
            println!("Node Left = {}", e.key)
        },
        None => println!("Error: no left element"),
    }

    match root.right {
        Some(e) => {
            println!("Node Right = {}", e.key)
        },
        None => println!("Error: no left element"),
    }

}

fn use_concurrency() {
    pub struct Bank {
        balance: f32
    }

    fn withdraw(bank: Arc<Mutex<Bank>>, amount: f32){
        let mut bank_ref = bank.lock().unwrap();
        if bank_ref.balance < 5.0 {
            println!("Impossible to withdraw");
        } else {
            bank_ref.balance -= amount;
            println!("Customer withdrew and has balance {}", bank_ref.balance);
        }
    }

    fn deposit(bank: Arc<Mutex<Bank>>, amount: f32){
        let mut bank_ref = bank.lock().unwrap();
        bank_ref.balance += amount;
        println!("Customer deposited and has balance {}", bank_ref.balance);
    }
    
    let bank = Arc::new(Mutex::new(Bank{balance: 50.0}));

    let mut count = 0;
    let handles = (0..100).map(|_| {
        let bank_ref = bank.clone();
        let thread;
        if count % 2 == 0 {
            thread = thread::spawn(|| {
                withdraw(bank_ref, 5.0);
            });
        } else {
            thread = thread::spawn(|| {
                deposit(bank_ref, 5.0);
            });
        }
        count += 1;
        thread
    });

    for handle in handles {
        handle.join().unwrap();
    }

}

fn main() {
    use_ownership();
    use_mutable_references();
    use_generics();
    use_traits();
    use_closures();
    use_smart_pointers();
    use_concurrency();
}
