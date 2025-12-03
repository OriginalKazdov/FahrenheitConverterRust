use std::io;

fn main() {
    println!("Welcome to the Degrees calculator");
    println!("Type 1 to convert Fahrenheit to Celsius or Type 2 for the other way");
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Failed to read");
    let option: u32 = option.trim().parse().expect("Invalid number");
    let number = read_number();

    if option == 1{
      let c = fahren_to_celsius(number);
      println!("The temperature is {} celsius", c);
    } else {
      let f = celsius_to_fahren(number);
      println!("The temperature is {} fahrenheit", f);
    }
}

fn read_number() -> f32 {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read");
  input.trim().parse().expect("Invalid Number")
}

fn fahren_to_celsius(f: f32) -> f32{
    (f - 32.0) * (5.0 / 9.0)
}

fn celsius_to_fahren(c: f32) -> f32{
  (c * (9.0 / 5.0)) + 32.0
}
