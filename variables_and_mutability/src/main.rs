const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    println!("Three hours in seconds is {THREE_HOURS_IN_SECONDS}");
    
    let x = 2;
    let x = x + 1;
    
    {
        let x = x * x + 1;
        println!("The value of x in the inner scope is {x}");
    }
    
    println!("The value of x is {x}");
}

fn does_not_work() {
    /*
    let x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    */
}
