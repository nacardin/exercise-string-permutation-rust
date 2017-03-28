extern crate time;
use time::PreciseTime;
use std::env;

fn main() {
    let start = PreciseTime::now();
    
    let input_str = env::args().nth(1).unwrap();

    let permutations = permute(input_str);
    println!("permutations {:?}", permutations);

    let end = PreciseTime::now();
    println!("execution time {:?}", start.to(end));
}

fn permute(input_string: String) -> Vec<String> {

    let mut permutations = Vec::new();

    for input_string_index in 0..input_string.len() {

        let mut cloned_str = input_string.clone();
        if cloned_str.len() == 1 {
            return vec![cloned_str];
        }

        let popped_char = cloned_str.remove(input_string_index);

        let child_permutations = permute(cloned_str);

        for child_permutation in child_permutations.into_iter() {
            let mut child_permutation_clone = child_permutation.clone();
            child_permutation_clone.insert(0, popped_char);
            permutations.push(child_permutation_clone);
        }
    }

    return permutations;
}

#[test]
fn permute_test() {

    fn contains(vec: &Vec<String>, item: &str) -> bool {
        return vec.contains(&String::from(item));
    }

    let results = permute(String::from("abc"));

    assert_eq!(results.len(), 6);
    assert!(contains(&results, "abc"));
    assert!(contains(&results, "acb"));
    assert!(contains(&results, "bac"));
    assert!(contains(&results, "bca"));
    assert!(contains(&results, "cab"));
    assert!(contains(&results, "cba"));


}