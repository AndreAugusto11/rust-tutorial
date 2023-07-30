
// ###### largest ######

fn find_largest_i32(list: &[i32]) -> &i32 {
    let mut max = &list[0];

    for item in list {
        if item > max {
            max = item;
        }
    }
    max
}

fn find_largest_char(list: &[char]) -> &char {
    let mut max = &list[0];

    for item in list {
        if item > max {
            max = item;
        }
    }
    max
}

fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];

    for item in list {
        if item > max {
            max = item;
        }
    }
    max
}

fn find_middle_element<T>(list: &[T]) -> &T {
    &list[list.len() / 2]
}

fn largest() {
    let array_i32 = vec![1, 2, 6, 4, 5, 3, 5, 4, 3, 2, 1];
    let array_char = vec!['a', 'b', 'e', 'd', 'c', 'd', 'c', 'b', 'a'];

    let max_i32 = find_largest_i32(&array_i32);
    let max_char = find_largest_char(&array_char);
    let max_i32_2 = find_largest(&array_i32);
    let max_char_2 = find_largest(&array_char);
    let middle_i32 = find_middle_element(&array_i32);
    let middle_char = find_middle_element(&array_char);

    assert!(max_i32 == max_i32_2, "max_i32 = {}, max_i32 = {}", max_i32, max_i32_2);
    assert!(max_char == max_char_2, "max_char = {}, max_char = {}", max_char, max_char_2);

    println!("Max 1: {}", max_i32);
    println!("Max 2: {}", max_i32_2);
    println!("Min 1: {}", max_char);
    println!("Min 2: {}", max_char_2);
    println!("Middle 1: {}", middle_i32);
    println!("Middle 2: {}", middle_char);
}

// ###### struct ######

struct Point<X1, X2> {
    x: X1,
    y: X2,
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl Point<i32, i32> {
    fn distance_from_origin(&self) -> i32 {
        self.x + self.y
    }
}

fn points() {
    let point_1 = Point { x: 22, y: -44};
    let point_2 = Point { x: 22.4, y: -44.5};

    println!("Point 1: x = {}, y = {}", point_1.x, point_1.y);
    println!("Point 2: x = {}, y = {}", point_2.x, point_2.y);

    println!("Distance - Point 1: dist = {}", point_1.distance_from_origin());
    println!("Distance - Point 2: dist = {}", point_2.distance_from_origin());
}

// ###### monomorphization ######

fn traits() {
    // A trait defines functionality a particular type has and can share with other types.
    // We can use traits to define shared behavior in an abstract way.

    // They are similar to interfaces in other languages

    trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)") // this is the default behavior
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }
    
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    
}

fn main() {
    // largest();
    // points();
    traits();
}
