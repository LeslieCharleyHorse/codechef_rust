fn main()
{
    //lesliecharleyhorse
    //import input + output library
    use std::io;

    // import splitwhitespace library
    use std::str::SplitWhitespace;


    // var hold result
    let mut res: i64;

    // var to hold diff
    let mut dif: i64;

    // var to hold input
    let mut input: String;

    // var to hold split input
    let mut split_input: SplitWhitespace;

    // read input, assign val to input var
    input = String::new();
    io::stdin().read_line(&mut input).expect("Readline = Failed");


    // split input, assign val to split_input var
    split_input = input.split_whitespace();

    // calculate res
    dif = 0;

    res = split_input.next().expect("Access element = Failed").parse::<i64>().expect("Convert to i64 = Failed");
    dif = res;

    res *= 100;
    dif = split_input.next().expect("Access element = Failed").parse::<i64>().expect("Convert to i64 = Failed") - dif;


    res += dif * 150;

    // lesliecharleyhorse

    // res
    println!("{res}");


}

