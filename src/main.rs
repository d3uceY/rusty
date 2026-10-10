const X: i8 = 5;
const B: i16 = 42;

const NUMBERS: [i16; 6] = [1, 23, 5, 56, 34, 34];

fn main() {
    let pr: (&str, i16, i8) = ("bench press", 200, 1);

    println!("{}", X);
    println!("{}", B);
    println!("{}", pr.0);
    println!("{}", pr.1);
    println!("{}", pr.2);

    for number in NUMBERS {
        println!("{number}");
    }
}
