use std::io;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;
#[derive(Debug)]
#[derive(PartialEq)]
struct Vector{
    x: f32,
    y: f32,
    z: f32,
}

impl Vector{
    fn new(x: f32, y: f32, z: f32) -> Vector{
        Vector{
            x,
            y,
            z,
        }
    }

    fn adding(&self, other: &Vector) -> Vector {
        let add_x = self.x + other.x;
        let add_y = self.y + other.y;
        let add_z = self.z + other.z;
        return Vector::new(add_x,add_y,add_z);
    }

    fn subtracting(&self, other: &Vector) -> Vector {
        let subtract_x = self.x - other.x;
        let subtract_y = self.y - other.y;
        let subtract_z = self.z - other.z;
        return Vector::new(subtract_x,subtract_y,subtract_z);
    }

    fn scalar_multiply(&self, scalar:f32) -> Vector {
        return Vector::new(self.x * scalar, self.y * scalar, self.z * scalar);
    }

    fn dot_product(&self, other: &Vector) -> f32 {
        let dot_prod = (self.x * other.x) + (self.y * other.y) + (self.z * other.z);
        return dot_prod;
    }
   
    fn cross_product(&self, other: &Vector) -> Vector {
        let cross_x = self.y * other.z - self.z * other.y;
        let cross_y = self.z * other.x - self.x * other.z;
        let cross_z = self.x * other.y - self.y * other.x;
        return Vector::new(cross_x, cross_y, cross_z);
    }

    fn calc_magnitude(&self) -> f32 {
        let magnitude = ((self.x * self.x) + (self.y * self.y)).sqrt();
        return magnitude;
    }

    fn angle_bet_2vec(&self, other: &Vector) -> f32 {
        let dot_product = self.dot_product(other);
        let magnitude_product = self.calc_magnitude() * other.calc_magnitude();
        
        let final_product = dot_product / magnitude_product;
        return final_product.to_degrees();
        
    } 
}

fn read_csv_data(filename: &str) -> Result<Vec<Vector>, Box<dyn Error>> {
    
    let file = File::open(filename)?;

    let mut vector = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let components : Vec<f32> = line
                        .split(',')
                        .map(|s| f32::from_str(s.trim()).unwrap())
                        .collect();
        if components.len() == 3{
            vector.push(Vector::new(components[0],components[1],components[2]));
        }
    }
    Ok(vector)
}

fn main() {
    let mut num_vecs = String::new();
    println!("Enter the numbers of vector: ");
    io::stdin().read_line(&mut num_vecs).expect("Failed to read");

    let num_vecs: usize = match num_vecs.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            eprintln!("Number of vectors shoud be positive integer");
            return;
        } 
    };

    if num_vecs < 2 {
        eprintln!("Number of sets must be at least 2.");
        return;
    }
    let vectors = read_csv_data("./src/vectors.csv").expect("Failed to read lines");
    if vectors.len() < num_vecs {
        eprintln!("There are not enough data in file");
        return;
    }

    let mut vec_list = Vec::new();
    for i in 0..num_vecs{
        let vector = &vectors[i];
        println!("Vector {}: {:?}",i+1,vector);
        vec_list.push(vector.clone());
    }
    
    if num_vecs >= 2 {
        

        let add_result = vec_list[0].adding(&vec_list[1]);
        println!("Adding two vector: {:#?}", add_result);

        let diff_result = vec_list[0].subtracting(&vec_list[1]);
        println!("Subtracing two vector: {:#?}", diff_result);

        let scalar : f32 = 0.5;
        let scalar_result = vec_list[0].scalar_multiply(scalar);
        println!("Scalar Multiplication: {:#?}", scalar_result);

        let dot_product = vec_list[0].dot_product(&vec_list[1]);
        println!("Dot-product: {:?}",dot_product);

        let cross_product = vec_list[0].cross_product(&vec_list[1]);
        println!("Cross Product: {:#?}",cross_product);

        let magnitude1 = vec_list[0].calc_magnitude();
        println!("Magnitude of vector-1: {}",magnitude1);
        let magnitude2 = vec_list[1].calc_magnitude();
        println!("Magnitude of vector-1: {}",magnitude2);

        let angle = vec_list[0].angle_bet_2vec(&vec_list[1]);
        println!("Angle between two vectors is: {}degree",angle);
    }
    
}
    
#[test]
fn test_vector_operations() {
    let vector1 = Vector::new(1.0, 2.0, 3.0);
    let vector2 = Vector::new(4.0, 5.0, 6.0);


    let add_result = vector1.adding(&vector2);
    assert_eq!(add_result, Vector::new(5.0, 7.0, 9.0), "Addition result should be as expected");

    let subtract_result = vector1.subtracting(&vector2);
    assert_eq!(subtract_result, Vector::new(-3.0, -3.0, -3.0), "Subtraction result should be as expected");

    let scalar = 0.5;
    let scalar_result = vector1.scalar_multiply(scalar);
    assert_eq!(scalar_result, Vector::new(0.5, 1.0, 1.5), "Scalar multiplication result should be as expected");

    let dot_product_result = vector1.dot_product(&vector2);
    assert_eq!(dot_product_result, 32.0, "Dot product result should be as expected");

    let cross_product_result = vector1.cross_product(&vector2);
    assert_eq!(cross_product_result, Vector::new(-3.0, 6.0, -3.0), "Cross product result should be as expected");

    let magnitude1_result = vector1.calc_magnitude();
    assert_eq!(magnitude1_result, 2.236068, "Magnitude of vector1 should be as expected");

    let magnitude2_result = vector2.calc_magnitude();
    assert_eq!(magnitude2_result, 6.4031243, "Magnitude of vector2 should be as expected");

    let angle_result = vector1.angle_bet_2vec(&vector2);
    assert_eq!(angle_result, 128.05473, "Angle between two vectors should be as expected");
}
    
    

