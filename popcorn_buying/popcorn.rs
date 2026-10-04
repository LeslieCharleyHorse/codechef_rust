fn main() {
    // import read input functionlaity and object
    use std::io;
    use std::io::Stdin;
    
    
    // var to hold input
    let mut input: String = String::new();
    
    // create input reader object
    let input_reader: Stdin = io::stdin();
    
    // var to hold int val
    let mut num: i16 = 0;
    
    
    // read line and hold value in input
    input_reader.read_line(&mut input).expect("Readline = Failed");
    //lesliecharleyhorse
    
    
    // convert input to int
    num = input.trim().parse::<i16>().expect("Convert to int = Failed");
    
    
    // print result
    println!("{}", (num - 100) / 50);
    
}
