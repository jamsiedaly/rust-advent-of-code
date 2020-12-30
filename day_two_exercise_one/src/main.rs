use std::fs;

fn main() {
    let filename = "data.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.split("\n").map(|line| line.to_string()).collect();

    let mut valid_password_count = 0;
    lines.iter().for_each(|line| {
        let parts: Vec<&str> = line.split(":").collect();
        let (rule, password) = (parts[0].trim(), parts[1].trim());
        let rule_parts: Vec<&str> = rule.split(" ").collect();
        let letter: char = rule_parts[1].trim().chars().next().unwrap();
        let bounds: Vec<u32> = rule_parts[0].split("-").map(|digits| digits.parse().unwrap() ).collect();
        let lower_bound = bounds[0];
        let upper_bound = bounds[1];
        let instances_of_letter: usize = password.chars().filter(|char| { *char == letter}).count();
        if instances_of_letter >= lower_bound as usize && instances_of_letter <= upper_bound as usize {
            valid_password_count += 1;
            println!("{}", line)
        }
    });
    println!("There are {} valid passwords", valid_password_count);
}
