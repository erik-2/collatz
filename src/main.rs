use num_bigint::{ToBigUint,BigUint};
use num_traits::{One,Zero};
use indicatif::{ProgressBar,ProgressStyle};
use std::time::Instant;
use std::io;

fn crop_biguint(n: &BigUint, size: usize) -> String {
    let mut repr = "..".to_owned();
    let mut s = n.to_str_radix(36);
    let pos = s.len() - size;
    match s.char_indices().nth(pos) {
        Some((pos, _)) => {
            s.drain(..pos);
        }
        None => {
            s.clear();
        }
    }
    repr.push_str(&s);
    repr
}


fn syracuse(n: &BigUint){
    let zero: BigUint = Zero::zero();
    let one: BigUint = One::one();
    let two: BigUint = 2.to_biguint().unwrap();
    let mut i: BigUint = n.clone();
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
            //print!("*: {count_multiply} , / {count_divide}\r");
    } 
    let total_iterations = &count_multiply + &count_divide;
    let max_repr = crop_biguint(&max,100);
    println!("Max = {} \n Iterations = {total_iterations}", max_repr);
    println!("*: {count_multiply} , / {count_divide}");
}

fn syracuse_bitwise(n: &BigUint){
    let zero: BigUint = Zero::zero();
    let one: BigUint = One::one();
    let two: BigUint = 2.to_biguint().unwrap();
    let mut i: BigUint = n.clone();
    let mut count_divide = 0;
    let mut max: BigUint = i.clone();
    let mut count_multiply = 0;
    while i != one {
        if &i % &two == zero {
            count_divide +=1;
            i = &i >> 1;
        }
        else {
            count_multiply += 1;
            i = (&i <<1) + &i + &one ;
        }
        if &i > &max {
            max = i.clone();
        }
        //print!("*: {count_multiply} , / {count_divide}\r");
    }
    let total_iterations = &count_multiply + &count_divide;
    let max_repr = crop_biguint(&max,100);
    println!("Max = {} \n Iterations = {total_iterations}",max_repr);
    println!("*: {count_multiply}, / {count_divide}");
}

fn reduced_syracuse_bitwise(n: &BigUint){

    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    let mut count_divide = 0;
    let mut count_multiply = 0;
    while i != one {
        if &i & &one == one {
            count_multiply += 1;
            i = ((&i <<1) + &i + &one) >> 1;
        }
        else {
            count_divide +=1;
            i = &i >> 1;
        }
        //print!("*: {count_multiply} , / {count_divide}\r");
    }
    let total_iterations = &count_multiply + &count_divide;
    println!("Iterations = {total_iterations}");
    println!("*: {count_multiply}, / {count_divide}");
}


fn incremental_syracuse(n: &BigUint) -> bool{
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    let min: BigUint = i.clone();
    let now = Instant::now();
    if i < (&one << 64) {
        return true;
    }
    loop {
        if now.elapsed().as_secs() > 10*60 {
            println!("Timeout for n= {min}");
        }
        if i == one {
            break;
        }
        if i < min {
            break;
        }
        if &i & &one == one{
            i = ((&i <<1) + &i + &one) >> 1;
        }
        else {
            i = &i >> 1;
        }
    } 
    return true;
}

fn main()-> io::Result<()>  {
    let zero: BigUint = Zero::zero();
    let one = 1.to_biguint().unwrap();
    let two = 2.to_biguint().unwrap();
    let fast = true;
    let power = 1_000_101;
    let my_big_number: BigUint = BigUint::pow(&two,power) - &one;
    if fast == false { 
        let now = Instant::now();
        let my_bn_str = crop_biguint(&my_big_number,100);
        println!("{}", my_bn_str);
        syracuse(&my_big_number);
        println!("Elapsed: {:.2?}", now.elapsed());
    
        let now = Instant::now();
        println!("Using bitwise");
        syracuse_bitwise(&my_big_number);
        println!("Elapsed: {:.2?}", now.elapsed());
    }
    let now = Instant::now();
    println!("Using reduced bitwise : ");
    reduced_syracuse_bitwise(&my_big_number);
    println!("Elapsed: {:.2?}", now.elapsed());

    let now = Instant::now();
    println!("Using incremental: ");
    incremental_syracuse(&my_big_number);
    println!("Elapsed: {:.2?}", now.elapsed());
    
    let now = Instant::now();
    let power:u32 = 64;
    let mut from: BigUint = BigUint::pow(&two,power);
    for i in 1..3 {
        
        let max: BigUint = &from + (std::u32::MAX-1).to_biguint().unwrap();
        if &from % &two == zero {
            from = &from + &one;
        }
        println!("Check from 2^{power} to 2^{power} + {i} * (2^32-1)");
        use num_traits::ToPrimitive;
        let diff = (&max-&from).to_u64().unwrap()/2;
    
        let bar = ProgressBar::new(diff);
        bar.set_style(ProgressStyle::with_template("[{elapsed}] {bar:40} {pos:>7}/{len:7} {msg} ETA:{eta}")
        .unwrap());
        let mut i = from;
        while i < max {
            bar.inc(1);
            incremental_syracuse(&i);
            i += &two;
        }
        bar.finish();
        println!("Last: {i}");
        from = &max + &one;
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    Ok(())
}
