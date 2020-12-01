use std::fs;

fn main() {
    let filename = "data.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let numbers: Vec<i32> = contents.split("\n").map(|value| value.parse().unwrap()).collect();
    for n1 in numbers.clone() {
        for n2 in numbers.clone() {
            for n3 in numbers.clone() {
                if (n1 + n2 + n3) == 2020 {
                    println!("{} + {} + {} = {}", n1, n2, n3, n1 + n2 + n3);
                    println!("Answer is equal to {}", n1 * n2 * n3);
                    return;
                }
            }
        }
    }
}
