fn main()
{
    // import input reader object and functionalily
    use std::io;
    use std::io::Stdin;

    // import splitwhitespace object and functionality
    use std::str::SplitWhitespace;


    // create input reader object
    let input_reader: Stdin = io::stdin();

    // var to hold input string 
    let mut input: String = String::new();

    // var to hold split values
    let mut input_split: SplitWhitespace = "".split_whitespace();

    // vector to hold values
    let mut nums: Vec<i32> = vec![0, 0];



    // read input and assign  val to input var
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // split input by space and assign result to input_split var
    input_split = input.split_whitespace();


    // split numbers and assign values
    for i in 0..2
        {
            nums[i] = input_split.next().expect("Access Index = Failed").parse::<i32>().expect("Convert to int = Failed");
        }

    // lesliecharleyhorse

    

    // use conditionals to find answer
    if nums[0] > nums[1]
        {
            println!("Alice");
        }

    else if nums[1] > nums[0]
        {
            println!("Bob");
        }
    else
        {
            println!("Draw")
        }
    
    
    

}



