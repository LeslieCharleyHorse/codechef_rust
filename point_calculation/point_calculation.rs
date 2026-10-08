fn main()
{
    // import input reader object and functionality 
    use std::io;
    use std::io::Stdin;


    // import splitwhitespace object and functionality
    use std::str::SplitWhitespace;


    // create input reader object
    let input_reader: Stdin = io::stdin();


    // var to hold input 
    let mut input: String = String::new();


    // iterator to hold split values
    let mut nums: SplitWhitespace = "".split_whitespace();

    // vars to hold wins draws losses as their total 
    let mut wins: i32 = 0;
    let mut draws: i32 = 0;

//lesliecharleyhorse


    // readline
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // split input 
    nums = input.split_whitespace();


    // assign values 
    wins = nums.next().expect("Access element = Failed").parse::<i32>().expect("Convert to int = Failed") * 3;
    draws = nums.next().expect("Access element = Failed").parse::<i32>().expect("Convert to int = Failed");


    // result 
    println!("{}", wins + draws);
    



}
