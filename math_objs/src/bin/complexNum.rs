use std::io;
use std::fs::File;
use std::io::BufRead;
use std::error::Error;

#[warn(non_snake_case)]
#[derive(Debug,Clone)]
struct Complex{
    real: f32,
    imginary: f32,
}
impl Complex{
    fn new(real: f32, imginary: f32) -> Complex {
        Complex {
            real,
            imginary,
        }
    }
    fn adding_complex(&self, other: &Complex) -> Complex {
        Complex::new(self.real + other.real, self.imginary + other.imginary)
    }
    fn subtract_complex(&self, other: &Complex) -> Complex {
        Complex::new(self.real - other.real, self.imginary - other.imginary)
    }
    fn multiply_complex(&self, other: &Complex) -> Complex {
        // (a + bi) * (c + di) = (ac - bd) + (ad + bc)i
        Complex::new(
            self.real * other.real - self.imginary * other.imginary,
            self.real * other.imginary + self.imginary * other.real
        )
    }
}

fn read_data(filename: &str) -> Result<Vec<Complex>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut complex_num = Vec::new();

    for line in reader.lines(){
        let line = line?;
        let component : Vec<f32> = line
                        .split(',')
                        .map(|s| s.trim().parse().unwrap())
                        .collect();
        if component.len() == 2{
            complex_num.push(Complex::new(component[0],component[1]));
        }
    }
    Ok(complex_num)
}


fn main(){
   let mut num_complex = String::new();
   println!("Please enter the number of complex numbers: ");

   io::stdin().read_line(&mut num_complex).expect("Failed to read line");
   let num_complex: usize = match num_complex.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            eprintln!("Number must be positive integer");
            return;
        }
   };
   if num_complex == 0{
        eprintln!("Must be greater than 0;");
        return;
   }

   if let Ok(complex_nums) = read_data("./src/complexNum.csv"){

        if complex_nums.len() < num_complex {
            eprintln!("There are no enough data in file");
            return;
        }

        let complex_to_use = &complex_nums[0..num_complex];
        for (i,complex) in complex_to_use.iter().enumerate(){
            println!("Complex Number {}: {:#?}", i + 1,complex);
        }

        let mut sum = complex_to_use[0].clone();
        for num in &complex_to_use[1..]{
            sum = sum.adding_complex(num);
        }
        println!("Adding complex number: {:?}",sum);

        let mut diff = complex_to_use[0].clone();
        for num in &complex_to_use[1..]{
            diff = diff.adding_complex(num);
        }
        println!("Subtracting complex number: {:?}",diff);

        let mut multiply_complex = complex_to_use[0].clone();
        for num in &complex_to_use[1..]{
            multiply_complex = multiply_complex.multiply_complex(num);
        }
        println!("Multiplying complex number: {:?}",multiply_complex);

   }else{

        eprintln!("Error reading from file");

   }
}

#[test]
fn test_complex(){
    let complex1 = Complex::new(2.0, 3.0);
    let complex2 = Complex::new(1.0, 4.0);

    
    let add_result = complex1.adding_complex(&complex2);
    assert_eq!(add_result.real, 3.0);
    assert_eq!(add_result.imginary, 7.0);

    
    let sub_result = complex1.subtract_complex(&complex2);
    assert_eq!(sub_result.real, 1.0);
    assert_eq!(sub_result.imginary, -1.0);

    
    let mul_result = complex1.multiply_complex(&complex2);
    assert_eq!(mul_result.real, -10.0);
    assert_eq!(mul_result.imginary, 11.0);
}