fn main()
{
    // import input + output library
    use std::io;

    // import splitwhitepsaceobject
    use std::str::SplitWhitespace;


    // var to hold input 
    let mut input: String = String::new();

    // var to hold split numbers
    let mut nums: SplitWhitespace = "".split_whitespace();

    // read input and assign value to input var
    io::stdin().read_line(&mut input).expect("Readline = Failed");

    // split input and assign val to nums var
    nums = input.split_whitespace();

    //lesliecharleyhorse


    // result
    println!("{}", nums.next().expect("Access index = Failed").parse::<i32>().expect("Convert to int = Failed") + nums.next().expect("Access index = Failed").parse::<i32>().expect("Convert to int = Failed") * 10);


}


