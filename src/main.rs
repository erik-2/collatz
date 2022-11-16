use num_bigint::{ToBigUint,BigUint, RandBigInt};
use num_traits::{One,Zero};
use indicatif::{ProgressBar,ProgressStyle};
use std::time::Instant;
use std::io;
use clap::{command,Command,Parser};

#[derive(Parser,Default,Debug)]
#[clap(author = "Author: Eric Tellier", version, about)]
/// An comparison of different implementations of the Collatz conjecture sequence for big integer
/// (2^(2^32-1)-1
struct Arguments {
    #[clap(default_value_t = String::new(), value_parser)]
    test: String,
    power: Option<u32>,
    decay: Option<u32>,
}

fn crop_biguint(n: &BigUint, size: usize) -> String {
    let mut repr = "..".to_owned();
    if n < &1_000_000_000.to_biguint().unwrap() {
        return n.to_string();
    }

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
    /* let mut count_divide = 0;
     * let mut max: BigUint = i.clone(); 
     * let mut count_multiply = 0;
     */
    while i != one {
        if &i % &two == zero {
            //count_divide +=1;
            i = &i / &two;
        }
        else {
            //count_multiply += 1;
            i = &i * &two + &i + &one;
        }
        /* if &i > &max {
         *     max = i.clone();
         *  }
         *  print!("*: {count_multiply} , / {count_divide}\r");
         */
    } 
    /* let total_iterations = &count_multiply + &count_divide;
     * let max_repr = crop_biguint(&max,100);
     * println!("\t Max = {} \n\t Iterations = {total_iterations}",max_repr);
     * println!("\t *: {count_multiply}, / {count_divide}");
     */
}

fn syracuse_bitwise(n: &BigUint){
    let zero: BigUint = Zero::zero();
    let one: BigUint = One::one();
    let two: BigUint = 2.to_biguint().unwrap();
    let mut i: BigUint = n.clone();
    /*
     * let mut count_divide = 0;
     * let mut max: BigUint = i.clone();
     * let mut count_multiply = 0;
     */
    while i != one {
        if &i % &two == zero {
            //count_divide +=1;
            i = &i >> 1;
        }
        else {
            //count_multiply += 1;
            i = (&i <<1) + &i + &one ;
        }
        /*
         * if &i > &max {
         *      max = i.clone();
         *  }
         *  print!("*: {count_multiply} , / {count_divide}\r");
         */
    }
    /* let total_iterations = &count_multiply + &count_divide;
    let max_repr = crop_biguint(&max,100);
    println!("\t Max = {} \n\t Iterations = {total_iterations}",max_repr);
    println!("\t *: {count_multiply}, / {count_divide}");
    */
}

fn reduced_syracuse_bitwise(n: &BigUint){
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    // let mut count_divide = 0;
    // let mut count_multiply = 0;
    while i != one {
        if &i & &one == one {
            //count_multiply += 1;
            i = ((&i <<1) + &i + &one) >> 1;
        }
        else {
           // count_divide +=1;
            i = &i >> 1;
        }
        //print!("*: {count_multiply} , / {count_divide}\r");
    }
    /* let total_iterations = &count_multiply + &count_divide;
     * println!("Iterations = {total_iterations}");
     * println!("*: {count_multiply}, / {count_divide}");
     */
}

fn reduced_syracuse_bitwise_while(n: &BigUint){
    let zero: BigUint = Zero::zero();
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    //let mut count_divide = 0;
    //let mut count_multiply = 0;
    while i != one {
        while &i & &one == one {
            //count_multiply += 1;
            i = ((&i <<1) + &i + &one) >> 1;
        }
        while &i & &one == zero {
            //count_divide +=1;
            i >>= 1;
        }
        //print!("*: {count_multiply} , / {count_divide}\r");
    }
    //let total_iterations = &count_multiply + &count_divide;
    //println!("Iterations = {total_iterations}");
    //println!("*: {count_multiply}, / {count_divide}");
}

fn optimum_syracuse(n: &BigUint) {
    let zero: BigUint = Zero::zero();
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    if &i & &one == zero {
        let a: u64 = i.trailing_zeros().unwrap();
        i = i >> a;
    }
    loop {
        i = (&i << 1) + &i + &one >> 1;
        // the following lines is worse :
        // i = &i >> i.trailing_zeros().unwrap();
        let a: u64 = i.trailing_zeros().unwrap();
        i = i >> a;
        if i == one{ 
            break;
        }
    }
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

fn opt_incremental_syracuse(n: &BigUint) -> bool{
    let zero: BigUint = Zero::zero();
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    let min: BigUint = i.clone();
    let now = Instant::now();
    if i < (&one << 64) {
        return true;
    }
    if &i & &one == zero {
        let a: u64 = i.trailing_zeros().unwrap();
        i = i >> a;
    }
    loop {
        if now.elapsed().as_secs() > 10*60 {
            println!("Timeout for n= {min}");
        }
        
        i = ((&i << 1) + &i + &one) >> 1;
        let a: u64 = i.trailing_zeros().unwrap();
        i = i >> a;
        if i == one || i < min{ 
            break;
        }
    } 
    return true;
}

fn benchmark() {
    let mut rng = rand::thread_rng();
    let my_big_number  = rng.gen_biguint(100_000);
    println!("{}",my_big_number);
    
    let now = Instant::now();
    print!("Using optimal incremental: ");
    opt_incremental_syracuse(&my_big_number);
    println!("\t\t...elapsed: {:.2?}", now.elapsed());
    
    let now = Instant::now();
    print!("Using incremental: ");
    incremental_syracuse(&my_big_number);
    println!("\t\t...elapsed: {:.2?}", now.elapsed());
    
    let now = Instant::now();
    print!("Using optimum: ");
    optimum_syracuse(&my_big_number);
    println!("\t\t...elapsed: {:.2?}", now.elapsed());
    
    let now = Instant::now();
    print!("Using reduced bitwise while: ");
    reduced_syracuse_bitwise_while(&my_big_number);
    println!("\t\t...elapsed: {:.2?}", now.elapsed());

    let now = Instant::now();
    print!("Using reduced bitwise : ");
    reduced_syracuse_bitwise(&my_big_number);
    println!("\t\t...elapsed: {:.2?}", now.elapsed());
    
    let now = Instant::now();
    print!("Using bitwise");
    syracuse_bitwise(&my_big_number);
    println!("\t\t...elapsed: {:.2?}", now.elapsed());
    
    let now = Instant::now();
    print!("Using basic implementation");
    syracuse(&my_big_number);
    println!("\t\t...elapsed: {:.2?}", now.elapsed());
}


fn main()-> io::Result<()>  {
    let two = 2.to_biguint().unwrap();
    let args = Arguments::parse();
    if args.test.trim().is_empty() {
        let my_big_number = match args.power {
            Some(n) => BigUint::pow(&two,n),
            None => {
                let mut rng = rand::thread_rng();
                rng.gen_biguint(1000)
            }
        };
        let k = match args.decay {
            Some(n) => n.to_biguint().unwrap(),
            None => 1.to_biguint().unwrap(),
        };
        let my_big_number = my_big_number + k;
        let zero: BigUint = Zero::zero();
        let one = 1.to_biguint().unwrap();
        //let power = args.number.unwrap();
        //let my_big_number: BigUint = BigUint::pow(&two,power) - &one;
        let my_bn_str = crop_biguint(&my_big_number,100);
        println!("{}", my_bn_str);
    }
    else {
        println!("Benchmarking:");
        benchmark();
    }
    

    /*
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
    */
    Ok(())
}
