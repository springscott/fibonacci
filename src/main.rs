use std::io;

fn main() {
    //fib number
    let mut fib = 0;
    //fib next number
    let mut fib_next = 1;

    loop {
        //get user input for n
        println!("Please input the nth Fibonacci number to generate.");
        
        let mut nth = String::new();

        //catch error
        io::stdin()
            .read_line(&mut nth)
            .expect("Failed to read line");

        //trim & get the right input
        let nth: u32 = match nth.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //loop for nth fib number
        for _n in 0..nth {
            //save fib_temp
            let fib_temp = fib;
            //calculate fib
            fib = fib_next;
            //calculate fib_next
            fib_next = fib_temp + fib_next;
            //println!("{}, {}", fib, fib_next);
        }

        //print result
        if nth % 10 == 1 {
            println!("{}st Fibonacci number is {}", nth, fib);
        } else if nth % 10 == 2 {
            println!("{}nd Fibonacci number is {}", nth, fib);
        } else if nth % 10 == 3 {
            println!("{}rd Fibonacci number is {}", nth, fib);
        } else {
            println!("{}th Fibonacci number is {}", nth, fib);
        }
        
        break;
    }
}
