use std::io;
fn main(){
    let mut inputs : Vec<bool> = Vec::new();

    let mut num_bool = String::new();
    println!("Enter boolean value to perform logical operations: ");
    io::stdin().read_line(&mut num_bool).expect("Fail to read");

    let num_bool: usize = match num_bool.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            eprintln!("Number of booleans should be positive");
            return;
        }
    };

    for i in 0..num_bool{
        loop {
            println!("Enter a bool value : ");
            let mut bool_input = String::new();
            io::stdin().read_line(&mut bool_input).expect("Fail to read");

            match bool_input.trim(){
                "true" => {
                    inputs.push(true);
                    break;
                }
                "false" => {
                    inputs.push(false);
                    break;
                }
                _ => {
                    eprintln!("Invalid Inputs. Only enter true or false");
                }
            }
        }
    }
    for values in &inputs{

        println!("{}",values);
    }

    let and_result = inputs.iter().all(|&x| x);
    let or_result = inputs.iter().any(|&x| x);
    let not_results: Vec<bool> = inputs.iter().map(|&x| !x).collect();

    println!("AND result: {}", and_result);
    println!("OR result: {}", or_result);
    println!("NOT results: {:?}", not_results);

}
    
    


    
#[test]
    fn test_logical_operations() {
        let mut user_booleans: Vec<bool> = Vec::new();
        
        // Simulate user input
        user_booleans.push(true);
        user_booleans.push(false);
        user_booleans.push(true);
        user_booleans.push(false);
        
        let and_result = user_booleans.iter().all(|&x| x);
        assert_eq!(and_result, false, "AND result should be false");
        
        let or_result = user_booleans.iter().any(|&x| x);
        assert_eq!(or_result, true, "OR result should be true");
        
        let not_results: Vec<bool> = user_booleans.iter().map(|&x| !x).collect();
        assert_eq!(not_results, vec![false, true, false, true], "NOT results should be as expected");
    }

