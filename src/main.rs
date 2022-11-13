use num_bigint::{ToBigUint,BigUint};
use num_traits::{One,Zero};

fn syracuse(n: BigUint){
    let zero: BigUint = Zero::zero();
    let one: BigUint = One::one();
    let two: BigUint = 2.to_biguint().unwrap();
    let mut i: BigUint = n;
    let mut count_divide = 0;
    let mut max: BigUint = i.clone(); 
    let mut count_multiply = 0;
    while i != one {
        if &i % &two == zero {
            count_divide +=1;
            i = &i / &two;
        }
        else {
            count_multiply += 1;
            i = &i * &two + &i + &one;
        }
        if &i > &max {
            max = i.clone();
        }
    } 
    let total_iterations = &count_multiply + &count_divide;
    println!("Max = {max} | Iterations = {total_iterations}");
    println!("*: {count_multiply} , / {count_divide}");
}


fn main() {
    let power = 126;
    let mut my_big_number: BigUint = i128::pow(2,power).to_biguint().unwrap();
    let n = 10;
    println!("n = {n}");
    let mut size = power;
    for _ in 1..(n+1) {
        size += size;
        my_big_number = my_big_number.clone() * my_big_number;
    }
    my_big_number -= 1.to_biguint().unwrap();
    println!("Length : {}", size);
    use std::time::Instant;
    let now = Instant::now();
    println!("{}",my_big_number);
    syracuse(my_big_number);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}
