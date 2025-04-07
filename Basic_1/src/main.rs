//is even  or odd -->
/* 
    fn main() {
        println!("{}", is_even(5));
        //let ans = is_even(5);
        //println!("{}", ans);
    }
    fn is_even(num: i32) -> bool {
        if num % 2 == 0 {
            return true;
        }
        return false;
    }
*/

//fibonacci -->
/* 
     fn main() {
        println!("{}", fibonacci(5));
     }

     fn fibonacci(num: i32) -> i32 {
        let mut a = 0;
        let mut b = 1;
        if num == 0 {
            return a;
        }
        if num == 1 {
            return b;
        }
        for _i in 1..num -2 {
            let c = a + b;
            a = b;
            b = c;
        }
        return b;
     }
*/

// length of a string -->
/* 
    fn main() {
        let my_string = String::from("hello, world");
        let length = get_string_length(my_string);
        println!("The length of the string is: {}", length);
    }
    fn get_string_length(str: String) -> usize {
       return str.chars().count();
    }
*/ 

// structure -->
/* 
    struct User {
        first_name: String,
        last_name: String,
        age: i32,
    }
    fn main() {
        let user = User {
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            age: 30,
        };
        println!("User: {} {}, Age: {}", user.first_name, user.last_name, user.age);
    }
*/

//struct with implementation way -->
/* 
    struct Rect {
        width: i32,
        height: i32,
    }
    impl Rect {
        fn area(&self) -> i32 {
            self.width * self.height
        }
        fn perimeter(&self) -> i32 {
            2 * (self.width + self.height)
        }
    }
    fn main() {
        let rect = Rect {
            width: 10,
            height: 5,
        };
        println!("Area of rectangle: {}", rect.area());
        println!("Perimeter of rectangle: {}", rect.perimeter());
    }
*/

// struct with diff arg -->
/* 
    struct Rect {
        width: i32,
        height: i32,
    }
    impl Rect {
        fn area(&self) -> i32 {
            self.width * self.height
        }
        fn perimeter(&self, num: i32) -> i32 {
            2 * (self.width + self.height)
        }
        fn debug() -> i32 {
            return 1;
        }
    }
    fn main() {
        let rect = Rect {
            width: 10,
            height: 5,
        };
        println!("Area of rectangle: {}", rect.area());
        println!("Perimeter of rectangle: {}", rect.perimeter(2));
        println!("Debug: {}", Rect::debug()); //remember since method u are calling has no arg (static function) so u cant do rect.debug() 
    }
*/

// enum -->
/* 
    enum Shape {
        Rectange,
        Circle, 
    }
    fn main() {
        let my_shape = Shape::Circle;
        print_area(my_shape);
    }
    fn print_area(shape:Shape) {
        println!("area here");
    }
enum with associated arguments 
    enum Shape {
        Rectangle (f64, f64),
        Square (f64),
        Circle (f64),
    }
    fn calculate_area(shape: Shape) -> f64 {
        match shape {
            Shape::Rectangle(width, height) => width * height,
            Shape::Square(side) => side * side,
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        }
    }
    fn main() {
        let circle = Shape:: Circle(5.0);
        let rectangle = Shape::Rectangle(5.0, 10.0);
        let square = Shape::Square(4.0);
        println!("Area of circle: {}", calculate_area(circle));
        println!("Area of rectangle: {}", calculate_area(rectangle));
        println!("Area of square: {}", calculate_area(square));
    }
*/

// Option enum lets u return either some value or none value -->
// find the 1st a in a string (w/o using Option enum):
/* 
     fn main() {
        let index = find_first_a(String::from("preet"));
        if index==-1 {
            println!("a not found");
        } else {
            println!("a found at index {}", index);
        }
    }
    fn find_first_a(s: String) -> i32 {
        for (index, char) in s.chars().enumerate() {  //s.chars().enumerate() gives u the index and char at that index
            if char == 'a' {
                return index as i32;
            }
        }
        return -1;
    }
*/

// find the 1st a in a string (w using Option enum) -->
/* 
    fn main() {
        let index = find_first_a(String::from("preet"));
        match index {
            Some(value) => println!("index is {}", value),
            None => println!("a not found"),
        }
    }
    fn find_first_a(s: String) -> Option<i32> {
        for (index, char) in s.chars().enumerate() {  //s.chars().enumerate() gives u the index and char at that index
            if char == 'a' {
                return Some(index as i32);
            }
        }
        return None;
    }
*/   

// try custom Option  -->
/* 
    fn main() {
        let index = find_first_a(String::from("preet"));
        match index {
            CustomOption::Some(value) => println!("index is {}", value),
            CustomOption::None => println!("a not found"),
        }
    }
    enum CustomOption {
        Some(i32),
        None,
    }

    fn find_first_a(s: String) -> CustomOption {
        for (index, char) in s.chars().enumerate() {  //s.chars().enumerate() gives u the index and char at that index
            if char == 'a' {
                return CustomOption::Some(index as i32);
            }
        }
        return CustomOption::None;
    }
*/ 

//Result enum let u return either Ok value or Err value the result enum is how you can do error handling in Rust -->
// eg - write a function that reads the contents of a file:
/*
    use std::fs::read_to_string;

    fn main() {
        let result = read_to_string("s.text");
        println!("result: {:?}", result);
        match result {
            Ok(value) => println!("file contents: {}", value),
            Err(err) => println!("error reading file: {}", err),
        }
    }
*/

//Package management -->
/*
    cargo add create_name : u can add an external create to ur project by running this 
*/
/* 
    use chrono::{Local, Utc};

    fn main() {
        let now = Utc::now();
        println!("Current date and time in UTC: {}", now);

        let formatted = now.format("%Y-%m-%d %H:%M:%S");
        println!("Formatted date and time: {}", formatted);

        let local = Local::now();
        println!("Current date and time in local timezone: {}", local);
    }
*/

// MEMORY MANAGEMENT -->
/*
    Stored on thhe stack 
    1. Numbers: i32, f64, u8, etc.
    2. Booleans: true, false
    3. Characters: 'a', 'b', '1', etc.
    4. Fixed sized arrays - [i32; 4]
    5. Refernces - &i32, &str

    Stored on the heap
    1. Strings: String
    2. Vectors: Vec<i32>
    3. HashMaps: HashMap<String, i32>
    4. Dynamic sized arrays: [i32]
    5. Structs and Enums: Custom data types
    6. Large Arrays/struct that can't fit on the stack
*/

//MUTABILITY -->
/*
    In Rust, variables are immutable by default. This means that once a variable is bound to a value, you cannot change that value.
    To make a variable mutable, you need to use the mut keyword when declaring it. This allows you to change the value of the variable later in your code.

*/

// Ownership and Borrowing -->
/*
    Ownership is a key concept in Rust that ensures memory safety without needing a garbage collector. 
    In Rust, each value has a single owner, and when the owner goes out of scope, the value is automatically deallocated.
    Borrowing allows you to temporarily use a value without taking ownership of it. 
    You can borrow a value either mutably or immutably, but you cannot have mutable and immutable borrows at the same time.

    Ownership rules:
    1. Each value in Rust has a single owner. (only one owner at a time)
    2. When the owner goes out of scope, the value is dropped (deallocated).

    example of ownership:
    fn main() {
        let s1 = String::from("hello");
        let s2 = s1; // ownership of s1 is moved to s2
        // println!("{}", s1); // this will cause a compile-time error
        println!("{}", s2); // this is valid
    }
    example of borrowing:
    fn main() {
        let s1 = String::from("hello");
        let len = calculate_length(&s1); // borrow s1
        println!("The length of '{}' is {}.", s1, len); // s1 is still valid here
    }
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
*/

//MOVING -->
/*
     fn create_string() {
        let s1: String = String::from("hello");
        let s2: String = s1;
        println!("{}", s1); // this will cause a compile-time error
        println!("{}", s2); // this is valid
     }
     fn main() {
     create_string();
     }
*/

/*
    fn main() {
        let s1 =  String::from ("hello");
        do_something(s1);
        println!("number uis {}", s1); // this will cause a compile-time error
    }

    fn do_something(s: String) {
        println!("{}", s);
    }
*/

// if u want to use the value after moving it, u can use the clone method to create a deep copy of the value.
/*
    fn main() {
        let s1 = String::from("hello");
        let s2 = s1.clone(); // create a deep copy of s1
        println!("{}", s1); // this is valid
        println!("{}", s2); // this is also valid
    }
*/
// s1 remains the owner everywhere s2 just borrows the value 
/* 
    fn main() {
        let s1 = String::from("helloworld");
        do_something(s2:&s1); // pass a reference to s1.. s2 borrows from s1
        println!("{}", s1); // owned by s1
    }
    fn do_something(s2: &String) {
        println!("{}", s2); //s2 owns the value
    }
*/ 

// borrowing to mutate the value --> which means u can change the value of the borrowed variable
/*
    fn main() {
        let mut s1 = String::from("hello");
        do_something(s2: &mut s1);
        println!("number uis {}", s1); // this is valid
    }

    fn do_something(s2: &mut String) {
        s2.push_str(string: "world");
        println!("{}", s2); // this is valid
    }
*/

// mutable borrowing -->
  /*
  
  Rule of refernces:
   - at any given time, u can have either one mutable reference or any number of immutable references
   - References must always be valid
  */

  //here if s2 has mutable borrow of s1 then s1 cannot be borrowed immutably or even mutably
/* 
    fn main() {
        let mut s1 = String::from("hello");
        let s2 = &mut s1;
        let s3 = &s1; // this will cause a compile-time error
        println!("{}, {}, {}", s1,s2,s3); // this is valid
    }
*/