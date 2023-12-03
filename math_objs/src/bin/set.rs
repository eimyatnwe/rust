use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn read_sets_from_csv(filename: &str) -> Result<Vec<HashSet<i32>>, Box<dyn Error>> {

    let file = File::open(filename)?;

    let mut sets = Vec::new();
    let mut current_set = HashSet::new();

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let elements: Vec<i32> = line
            .split(',')
            .map(|s| i32::from_str(s.trim()).unwrap())
            .collect();
        for element in elements {
            current_set.insert(element);
        }
        sets.push(current_set.clone());
        current_set.clear();
    }

    Ok(sets)
}


fn main() {
    let mut num_sets = String::new();

    println!("Enter the number of sets: ");
    io::stdin().read_line(&mut num_sets).expect("Failed to read line");

    let num_sets: usize = match num_sets.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input. Please provide a positive integer.");
            return;
        }
    };

    if num_sets == 0 {
        eprintln!("Number of sets must be greater than 0.");
        return;
    }

    if let Ok(sets) = read_sets_from_csv("./src/sets.csv") {
        if sets.len() < num_sets {
            eprintln!("Not enough sets in the file to perform operations.");
            return;
        }

        let sets_to_use = &sets[0..num_sets];
        // println!("{:?}",sets_to_use);
        for (i,set) in sets_to_use.iter().enumerate(){
            println!("Set {}: {:?}", i + 1, set);
            
        }
        println!("Choose set operation: ");
        println!("1. Union");
        println!("2. Intersection");
        println!("3. Difference");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" if num_sets >= 2 => {
                let mut union_result = sets_to_use[0].clone();
                for set in &sets_to_use[1..] {
                    for ele in set{
                        if !union_result.contains(ele){
                            union_result.insert(*ele);
                        }
                    }
                }
                println!("Union result: {:?}",union_result);
            }
            
            "2" if num_sets >= 2 => {
                let mut result = sets_to_use[0].clone();

                for set in &sets_to_use[1..] {
                    let mut temp = HashSet::new();

                    for ele in set{
                        if result.contains(ele) {
                            temp.insert(*ele);
                        }
                    }
                    result = temp;
                }
                println!("Intersection Result: {:?}",result);
            }

            "3" if num_sets >= 2 => {
                let mut result = sets_to_use[0].clone();

                for set in &sets_to_use[1..] {
                    for ele in set {
                        if result.contains(ele) {
                            result.remove(ele);
                        }
                    }
                }
                println!("Difference Result: {:?}",result);
            }
            _ => eprintln!("invalid choice"),
        }

    }else{
        eprintln!("Error reading data");
    }
    
}

#[test]
fn test_read_sets_from_csv() {
    // Create a temporary CSV file with sets for testing
    let csv_data = "1, 2, 3\n4, 5\n6, 7, 8";
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let file_path = temp_dir.path().join("test_sets.csv");
    std::fs::write(&file_path, csv_data).expect("Failed to write to temporary CSV file");

    // Call the function to read sets from the temporary CSV file
    let result = read_sets_from_csv(&file_path.to_str().unwrap());

    // Define the expected result
    let expected_sets: Vec<HashSet<i32>> = vec![
        [1, 2, 3].iter().cloned().collect(),
        [4, 5].iter().cloned().collect(),
        [6, 7, 8].iter().cloned().collect(),
    ];
    match result {
        Ok(sets) => assert_eq!(sets, expected_sets),
        Err(err) => panic!("Test failed: {}", err),
    }
}
