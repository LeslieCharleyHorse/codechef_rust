fn main() {
    // import read input functionlaity + input reader object
    use std::io;
    use std::io::Stdin;
    
    
    // var for input as string
    let mut input: String = String::new();
    
    // create input reader object
    let input_reader: Stdin = io::stdin();
    
    // var to hold input as int 
    let mut num: f32 = 0.0;
    
    
    // readline of input and assign value to input
    input_reader.read_line(&mut input).expect("Readline = Failed");
    
    
    // convert input to number and store value in  num
    num = input.trim().parse::<f32>().expect("Convert to int = Failed");
    
    
    // conditional chain to find answer
    // buys; less than 5
    //lesliecharleyhorse
    if num < 5.0
        {
            println!("{}", num*100.0);
        }
    // buys; more than or equal to 5
    else
        {
            num *= (100.0 * 0.85);
            println!("{}", num);
        }
    
}
