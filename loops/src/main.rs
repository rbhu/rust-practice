fn main() {



    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    array_loop();
}

fn array_loop() {
    let a = [10,20,30,40,50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}
