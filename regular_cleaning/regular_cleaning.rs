fn main()
{
    // import standard library input read functionality + object
    use std::io;
    use std::io::Stdin;

    // declare variable hold string input
    let mut input: String = String::new();

    // create input reader object
    let input_reader: Stdin = io::stdin();

    // create var to hold integer value and res 
    let mut num: i16 = 0;
    let mut res: i16 = 0;


    // readline of input and hold value in input as string
    input_reader.read_line(&mut input).expect("Readline = Failed");

    // convert input into int
    num = input.trim().parse::<i16>().expect("Convert Number = Failed");

    //lesliecharleyhorse


    // get last digit of number
    res = num % 10;

    // print result
    println!("{}", 10-res);


}
