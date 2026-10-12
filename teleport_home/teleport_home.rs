fn main()
{
    // import input object and functionality
    use std::io;
    use std::io::Stdin;

    // import splitwhitespace object and functionality 
    use std::str::SplitWhitespace;



    // create input reader object
    let input_reader: Stdin = io::stdin();

    // var to old input
    let mut input: String = String::new();

    // var to hold split input
    let mut input_split: SplitWhitespace = "".split_whitespace();

    // var vector to hold int values of numbers
    let mut nums: Vec<i16> = vec![0, 0];


    // read input and assign value to input var
    //leslicharleyhorse
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // split input and assign val to input_split var
    input_split = input.split_whitespace();

//lesliecharleyhorse


    // convert stringg to int then assign value to nums vector
    nums[0] = input_split.next().expect("Access index = Failed").parse::<i16>().expect("Convert to i16 = Failed");
    nums[1] = input_split.next().expect("Access index = Failed").parse::<i16>().expect("Convert to i16 = Failed");



    // conditionals to find answer
    if nums[1] > nums[0]
        {
            println!("0")
        }
    else
        {
            println!("{}", nums[0] - nums[1]);
        }


}