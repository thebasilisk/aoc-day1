use std::fs::read_to_string;


fn main() {
    let input = read_to_string("input.txt").unwrap();
    
    let (mut list1, mut list2) : (Vec<u32>, Vec<u32>) = input.lines().map(|line| line.split_once(" ").map(|x| (x.0.parse::<u32>().unwrap(), x.1.trim().parse::<u32>().unwrap())).unwrap()).unzip();
    list1.sort();
    list2.sort();

    println!("{}", list1.iter().zip(list2.iter()).fold(0, |acc, e| acc + e.0.abs_diff(*e.1)));
    println!("{}", list1.iter().fold(0, |acc, e| acc + e * list2.iter().filter(|num| *num == e).count() as u32));
}
