fn main() {
    another_function(5);
    let my_num = 15;
    another_function(my_num);

    // because my_num is a primitive, stack memory type it would still be valid to access after
    // another_function(my_num);

    println!("my_num is still here, {my_num}");

    // this would not work with something that requires heap storage.

    let my_string = String::from("hello!");
    
    say_string(my_string);

    // my_string should be dropped after the say_string call
    // making the below impossible

    // println!("{}", my_string);
    
    // We can either clone the string (which requires duplicating the heap allocation)
    // or borrow the reference using a function that expects a borrowed value

    let my_string_2 = String::from("second string here!");
    say_string_borrow(&my_string_2);
    println!("{}", my_string_2);

    


    let fib_num = get_fib_at(5);
    println!("The fifth number in the fibonacci sequence as calculated here: {fib_num}");
    let fib_num = get_fib_at_while_version(5);
    println!("The fifth number in the fibonacci sequence as calculated here: {fib_num}");
}
// function is declared as accepting input of type i32 called x
fn another_function(x: i32){
    println!("The value of x is: {x}");
}

fn get_fib_at(x: i16) -> i32 {
    if x < 1 {
        return -1;
    } else if x == 1 {
        return 1
    }
    let mut count: i16 = 2;
    let mut fib_two = (1, 1);
    loop {
        if count >= x {
            break;
        }
        let sum = fib_two.0 + fib_two.1;
        let temp = fib_two.1;
        fib_two.0 = temp;
        fib_two.1 = sum;
        count = count + 1;
    }

    return fib_two.1;
}

fn get_fib_at_while_version(x: i16) -> i32 {
    if x < 1 {
        return -1;
    } else if x == 1 {
        return 1
    }
    let mut count: i16 = 2;
    let mut fib_two = (1, 1);
    while count < x {
        let sum = fib_two.0 + fib_two.1;
        let temp = fib_two.1;
        fib_two.0 = temp;
        fib_two.1 = sum;
        count = count + 1;
    }

    return fib_two.1;
}

fn say_string(str: String) {
    println!("{}",str);
}
fn say_string_borrow(str: &String) {
    println!("{}",str);
}
