use std::fs::File;
use std::io::{self,BufRead};


#[derive(Debug,Clone)]
struct Matrix{
    data : Vec<Vec<f32>>,
}

impl Matrix{
    fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            data: vec![vec![0.0; cols]; rows],
        }
    }

    fn add_matrices(&self, other: &Matrix) -> Result<Matrix, &'static str> {

        if self.data.len() != other.data.len() || self.data[0].len() != other.data[0].len() {
            return Err("Matrices dimension are not equal to do add-operation.");
        }
        let mut result = Matrix::new(self.data.len(),self.data[0].len());
        for i in 0..self.data.len(){
            for j in 0..self.data[0].len(){
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        Ok(result)
    }

    fn subtract_matrices(&self, other: &Matrix) -> Result<Matrix, &'static str> {
        if self.data.len() != other.data.len() || self.data[0].len() != other.data[0].len() {
            return Err("Matrices dimension are not equal to do subtract-operation.");
        }
        let mut result = Matrix::new(self.data.len(),self.data[0].len());
        for i in 0..self.data.len(){
            for j in 0..self.data[0].len(){
                result.data[i][j] = self.data[i][j] - other.data[i][j];

            }
        }
        Ok(result)
    }

    fn multiply_matrices(&self, other: &Matrix) -> Result<Matrix, &'static str> {
        if self.data[0].len() != other.data.len() {
            return Err("No:of cols in the 1st matrix must be equal to no:of rows in 2nd matrix");

        }
        let mut result = Matrix::new(self.data.len(),other.data[0].len());
        for i in 0..self.data.len(){
            for j in 0..other.data[0].len(){
                
                for k in 0..self.data[0].len(){
                    result.data[i][j] += self.data[i][k] * other.data[k][j];

                }
            }
        }
        Ok(result)
    }
}

fn read_csv_data(filename: &str) -> Result<Vec<Matrix>, io::Error> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut matrices = Vec::new();
    let mut current_matrix = Matrix::new(0,0);

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty(){

            matrices.push(current_matrix);
            current_matrix = Matrix::new(0,0);

        }else{
            let row: Vec<f32> = line
                    .split(',')
                    .map(|s| s.trim().parse().unwrap())
                    .collect();
            current_matrix.data.push(row);
        }
    }
    matrices.push(current_matrix);
    Ok(matrices)
}


fn main() {
    let mut num_matrices = String::new();
    println!("Please enter the number of matrices: ");
    io::stdin().read_line(&mut num_matrices).expect("Failed to read line");

    let num_matrices : usize = match num_matrices.trim().parse(){
        Ok(num) => num,
        _ => {
            eprintln!("Invalid input.Please provide a positive integer");
            return;
        }

    };

    if num_matrices == 0 {
        eprintln!("Number of matrices must be greater than 0");
        return;
    }
    if let Ok(matrices) = read_csv_data("./src/matrices.csv"){
        if matrices.len() < num_matrices {
            eprintln!("There are not enough data in file");
            return;
        }
        let matrices_to_use = &matrices[0..num_matrices];
        
        for (i,matrix) in matrices_to_use.iter().enumerate(){
            println!("Matrix {}:", i + 1);
            for row in &matrix.data {
                print!("{:?}",row); 
            }
        
            println!(); 
            
        }
        println!("Choose set operation: ");
        println!("1. Adding");
        println!("2. Subtracting");
        println!("3. Multiplying");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                if num_matrices < 2 {
                    eprintln!("Number of matrix must be at least two to do operation.");
                    return;
                }
                let mut sum = matrices_to_use[0].clone();
                for i in 1..num_matrices {
                    match sum.add_matrices(&matrices_to_use[i]) {
                        Ok(result) => sum = result,
                        Err(err) => {
                            eprintln!("{}",err);
                            return;
                        }
                    }
                }
                println!("Adding two matrices: {:?}",sum);
            }
            "2" => {
                if num_matrices < 2 {
                    eprintln!("Number of matrix must be at least two to do operation.");
                    return;
                }
                let mut diff = matrices_to_use[0].clone();
                for i in 1..num_matrices{
                    match diff.subtract_matrices(&matrices_to_use[i]) {
                        Ok(result) => diff = result,
                        Err(err) => {
                            eprintln!("{}",err);
                            return;
                        }
                    }
                }
                println!("Subtracting Matrices: {:?}",diff);
            }
            "3" => {
                if num_matrices < 2 {
                    eprintln!("Number of matrices must be at least two to do operation.");
                    return;
                }
                let mut multiply = matrices_to_use[0].clone();
                for i in 1..num_matrices{
                    match multiply.multiply_matrices(&matrices_to_use[i]) {
                        Ok(result) => multiply = result,
                        Err(err) => {
                            eprintln!("{}",err);
                            return;
                        }
                    }
                }
                println!("Multiplying matrices: {:?}",multiply);
            }
            _ => eprintln!("Invalid choice"),
        }
        
    }else{
        eprintln!("Error reading from file");
    }
}

#[test]
fn test_matrix(){
    let matrix1 = Matrix {
        data : vec![
            vec![1.,2.],vec![3.,4.]
        ],
    };
    let matrix2 = Matrix {
        data : vec![
            vec![5.,6.],vec![7.,8.]
        ],
    };

    let sum = matrix1.add_matrices(&matrix2);
    let result = sum.unwrap();
    assert_eq!(result.data,vec![vec![6.,8.],vec![10.,12.]]);

    let diff = matrix1.subtract_matrices(&matrix2);
    let result = diff.unwrap();
    assert_eq!(result.data, vec![vec![-4.,-4.],vec![-4.,-4.]]);

    let multiply = matrix1.multiply_matrices(&matrix2);
    let result = multiply.unwrap();
    assert_eq!(result.data, vec![vec![19.,22.],vec![43.,50.]]);


}