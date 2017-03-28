extern crate time;
use time::PreciseTime;
use std::env;

fn main() {
    let start = PreciseTime::now();
    
    let mut input_str = env::args().nth(1).unwrap();

    let permutations = go_deeper(input_str);

    println!("{:?}", permutations);
    let end = PreciseTime::now();

    println!("execution time {:?}", start.to(end));
}

fn go_deeper(str: String) -> Vec<String> {



    let mut permutations = Vec::new();

    for c_idx in 0..str.len() {

        let mut cloned_str = str.clone();
            // println!("{:?}", cloned_str);

        if cloned_str.len() == 1 {
            return vec![cloned_str];
        }

        let popped_char = cloned_str.remove(c_idx);

        let deeper_permutations = go_deeper(cloned_str);

        for d_idx in 0..deeper_permutations.len() {
            let mut deeper_permutation = deeper_permutations[d_idx].clone();
            deeper_permutation.insert(0, popped_char);
            permutations.push(deeper_permutation);
        }

        // println!("{:?}", c);
    }


    // permutations.push(String::from("directions"));

    return permutations;
}