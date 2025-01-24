
enum flash_message {
    success,
    warning { category : i32, message: String},
}

mod greetingsMod;
mod mylib;

fn assert_test () -> &'static str {
    "Hello, this is assert_test."
}

fn print_flash_message (m : flash_message){
    match m {
        flash_message::success => println!("Success"),
        flash_message::warning { category, message } =>
        println!("This is warning :  {} {}", category, message),    
    }
}

fn print_generic < T: std::fmt::Debug + std::fmt::Display > ( value : T) {
    println! ( "Print_generic {}", value);
}

trait Animal {
    fn sound(&self) -> &str;
    fn name(&self) -> &str;
    
    fn talk(&self) {
        println! ("{} says {}", self.name(), self.sound())
    }
}

struct Dog {
    name: String,
}
struct Cat {
    name: String,
}

mod greetings {
    pub fn sayHello () {
        println!("Hello, Module");
    }
}

impl Animal for Dog {
    fn sound(&self) -> &str{
        "Woow"
    }
    fn name(&self) -> &str {
        &self.name
    }
}
impl Animal for Cat {
    fn sound(&self) -> &str{
        "Mew"
    }
    fn name(&self) -> &str {
        &self.name
    }
}


fn main() {

    println!("Hello, world!");
    print_sum(23, 41);
    println!("{}", plus_one(100));
    let square = | i : i32 | -> i32 {
        i * i
    };
    println!("{}", square(25));
    let x : char = 'x';
    println!("{x}");


    let (s1, s2) = ("Hello", "String");
    let mut s = String::from(s1) + s2;
    println!("{s}");
    s.push_str("Hi");
    s.push_str(s2);
    println!("{s}");
    let shirt_size = 14 ;
    let shirt_test = match shirt_size {
        11 => "Small",
        12 | 13 => "Medium",
        14 => "Large",
        _ => "Unknown",
    };

    let aa = 13 ;
    let bb = 20 ;
    let grouptest = match (aa, bb) {
        (10, 10) => "Low",
        (14, _) => "A",
        (_, 25) => "B",
        (x,y) if x > 10 && y < 30 => "Perfect",
        (_, _) => "Unknown",
    };

    println!("{shirt_test}");
    println!("{grouptest}");

    let peoplelist : [&str; 3 ] = ["A", "B", "C"];
    for i in peoplelist.iter(){
        println!("{}", i);
    } 
    for (index,name) in peoplelist.iter().enumerate() {
        println!("Person : {} {}", index, name);
    }

    let state_message = flash_message::warning { category: 2, message: String::from("I am Warning") };
    print_flash_message(flash_message::success);
    print_flash_message(state_message);

    print_generic("Hello, This is printing generic");
    print_generic(5);

    let my_Pet1 = Dog { name : String::from("Meng Meng")};
    let my_Pet2 = Cat { name : String::from("Ya Ung")};
    my_Pet1.talk();
    my_Pet2.talk();

    greetings::sayHello();
    greetingsMod::hello_import_mod();
    mylib::hello_in_lib();

    assert_eq!(assert_test(), "Hello, this is assert_test.");
    dependancy::dependancy_hello();


}

fn print_sum ( a:i8 , b:i8) {
    println!("Sum is total {}", a + b );
}

fn plus_one (a:i32 ) -> i32{
    a + 2
}
