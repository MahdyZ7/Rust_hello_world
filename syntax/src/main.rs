fn main() {
    // mutable vs unmutable
    let mut x = 5;
    let y = 6;
    println!("The value of x is {x}");
    x = 7;
    let y = y + 2; // turns out it can be mutanble
    const SECS_IN_HOUR: u16 = 60 * 60; //constatnts
    println!("The value of x is {x}");
    println!("The value of y is {y}");
    println!("The value of  SECS_IN_HOUR is {SECS_IN_HOUR}");

    // shadowing (scope for normies)
    {
        x += 5;
        let x = 1;
        println!("The value of x is {x}");
    }
    println!("The value of x is {x}");
    println!("The value of y is {y}");

    let name = "Ahmed";
    let len = name.len();
    println!("The varable name: {name}, is {len} letters long");

    // var types
    let num: isize = 98_222;
    let num1: u32 = 0o12;
    let num2: i8 = 0b0111_0000;
    let num3: u8 = b'A';
    // let num4: u8 = 256;
    let d = 2.00000001;
    let f: f32 = 2.000000001;
    println!("The vars declared are {num}, {num1}, {num2}, {num3}");
    println!("The vars declared are {d}, {f}");

    // tuple and array
    let tup: (i32, char) = (500, 'A');
    let x = tup.1;
    println!("The tuple is {x}");

    // array

    let arr = [1, 2, 4, 8];
    let arr1: [i32; 5] = [arr[0], 2, 4, 8, 16];
    let arr2 = [arr1[0]; 5];
    for elem in arr2 {
        print!("{elem} ");
    }
    println!();

    //function

    let m = {
        let y = 1123;
        y + 3 // no ;
    };
    println!("the value of m is {m}");
    let fun_res = another_function(5);
    println!("the function returns {fun_res}");

    let mut count = 0;
    count = loop {
        if count == 5 {
            break count - 10;
        }
        println!("A");
        count += 1;
    };
    println!("count: {count}");

    for number in 1..4 {
        print!("{number}, ");
    }
    println!();
    for number in (1..4).rev() {
        print!("{number}, ");
    }
    println!();
}

fn another_function(x: i32) -> i32 {
    if x <= 1 {
        1
    } else {
        x * another_function(x - 1)
    }
}
