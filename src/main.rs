/* new version number guessing*/
//use rand::Rng;
//use std::cmp::Ordering;
use std::io;

fn main() {


    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
// condition number 1

   if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

//condition 2

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of condition number  2 is: {number}");
    
    
    
    // counter 
    counter();
    
    
    //counter1
    counter_1();
    
    //looping
    looping1();
    
    //looping
    looping2();
    
    //looping3
      looping3(); 
      
      
     // tender_owner
     
     tender_owner();
    
    //   using the structs
        let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
      user1.email = String::from("anotheremail@example.com");
    
    
    

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    //let mut x = 5;
    println!("The value of x is: {x}");
  //  x = 6;
    println!("The value of x is: {x}");


let tup = (500, 6.4, 1);

    let (a, b, c) = tup;

    println!("The value of y is: {b}");

    let j = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

  /*  io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = j[index];

    println!("The value of the element at index {index} is: {element}"); */


another_function(5);
    print_labeled_measurement(5, 'h');

    let x_1 = five();

    println!("The value of x_1 is: {x_1}");
    
    let x_2 = plus_one(5);
    
println!("The value of x_2 is: {x_2}");

/*


    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }  */
}





struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn another_function(x :i32) {
    println!("Another function.");
      println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}



fn counter() {
  println!("begin counter 1 :");

   let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The counter result is {result}");
        println!(" end counter ");
    
      println!("");
      
        println!("");
    
}

fn counter_1() {
   println!(" begin counter _ 1 :");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    
    println!("");
    
      println!("");
      
        println!("");
    
}

fn looping1() {
 println!(" begin looping 1 :");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
    
      println!("End Looping");
    
    println!("");
    
      println!("");
      
        println!("");
}


fn looping2() {
 println!(" begin looping 2:");
 
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    
          println!("End Looping");
    
    println!("");
    
      println!("");
      
        println!("");
}


fn looping3() {
 println!(" begin looping 2:");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
        
                 
    


    }
     println!("End Looping");
     
     println!("      ");
    
    println!("---- begin for---");
       for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    
        println!("");
    
      println!("");
      
        println!("");
}



fn tender_owner() {
println!(" begin ownership");

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
    
    
    println!(" end ownership");
    
    
            println!("");
    
      println!("");
      
        println!("");
}




