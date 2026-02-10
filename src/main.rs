fn main(){
    // the compiler will warn you if you have "i" but don't use it in the loop
    for _ in 1..6 {
        println!("Hello world");
    }

    for i in 1..6 {
        println!("hello world {i}");
    }
}