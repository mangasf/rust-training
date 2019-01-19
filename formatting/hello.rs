fn main() {

    println!("Hello World!");

    println!("Hello World {}!", "Spain");

    println!("Hello {0}, My name is {1}.", "Peter", "Alice");

    println!(
        "Hello {name_1}, My name is {name_2}",
        name_1="Peter",
        name_2="Alice"
    );

    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
}