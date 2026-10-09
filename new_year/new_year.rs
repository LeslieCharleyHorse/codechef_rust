fn main()
{
    // import input reader object and functionality
    use std::io;
    use std::io::Stdin;
    
    //lesliecharleyhorse
    // create input reader object
    let input_reader: Stdin = io::stdin();
    
    // var to hold input
    let mut input: String = String::new();
    
    // var to hold result
    let mut res: i64 = 0; 
    
    
    
    // read number
    input_reader.read_line(&mut input).expect("Readline = Failed");
    
    
    // assign value
    res = 24 - input.trim().parse::<i64>().expect("Convert to int = Failed");
    
    
    println!("{}", res);
    
}
