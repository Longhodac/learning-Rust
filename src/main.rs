fn main(){
    println!("Hello world!");

    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    }
    else if n > 0 {
        print!("{} is positive", n);
    }
    else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            print!(", and is a small number, increase ten-fold");
            10 * n
        } else {
            print!(", and is a big number, halve the number");
            n / 2
        }; // all let binding need this semicolon
    
        println!("{} -> {}", n, big_n);

        let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);
}