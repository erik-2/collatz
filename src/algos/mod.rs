use std::time::Instant;
use num_bigint::{ToBigUint, BigUint};
use num_traits::One;
use num_format::{Locale, ToFormattedString};
use num_integer::Integer;


pub fn crop_biguint(n: &BigUint, size: usize) -> String {
    let mut repr = "..".to_owned();
    let two: BigUint = 2.to_biguint().unwrap();

    let max_pow: u32 = 250_000;
    if n > &BigUint::pow(&two,max_pow) {
        repr = "Too big... representation would take some time we don't have...".to_owned();
    }
    else {
        let max_pow: u32 = 169;
        if n < &BigUint::pow(&two,max_pow) {
            let mut s = (*n).to_formatted_string(&Locale::fr);
            let pos = s.len() - size;
            if &s.len() > &size {
                s.drain(..pos);
            }
            repr.push_str(&s);
        }
        else {
            let mut s = n.to_str_radix(10);
            let pos = s.len() - size;
            match s.char_indices().nth(pos) {
                Some((pos, _)) => {
                    s.drain(..pos);
                }
                None => {}
            }
            repr.push_str(&s);
        }
    }
    repr
}

fn basic(n: &BigUint) -> bool{
    let one: BigUint = One::one();
    let two: BigUint = 2.to_biguint().unwrap();
    let three: BigUint = 3.to_biguint().unwrap();
    let mut i: BigUint = n.clone();
    while i != one {
        if i.is_even() {
        //if &i % &two == zero {
            i = &i / &two;
        }
        else {
            i = &i * &three + &one;
        }
    }
    true
}

fn basic_with_count(n: &BigUint) -> bool{
    let one: BigUint = One::one();
    let two: BigUint = 2.to_biguint().unwrap();
    let three: BigUint = 3.to_biguint().unwrap();
    let mut i: BigUint = n.clone();
    let mut count_divide = 0;
    let mut max: BigUint = i.clone();
    let mut count_multiply = 0;
    while i != one {
        if i.is_even() {
        //if &i % &two == zero {
            count_divide +=1;
            i = &i / &two;
        }
        else {
            count_multiply += 1;
            i = &i * &three + &one;
        }
        if &i > &max {
           max = i.clone();
        }
        print!("*: {count_multiply} , / {count_divide}\r");
    }
    let total_iterations = &count_multiply + &count_divide;
    //let max_repr = crop_biguint(&max,100);
    //println!("\t Max = {} \n\t Iterations = {total_iterations}",max_repr);
    println!("\t Iterations = {total_iterations}");
    println!("\t *: {count_multiply}, / {count_divide}");
    true
}


pub fn syracuse(n: &BigUint, verbose: bool, method: &str) -> bool{
    match method {
        "optimum" => {
            print!("Using optimum: ");
            let now = Instant::now();
            let res = match verbose {
                false => optimum_syracuse(n),
                true => optimum_syracuse_with_count(n),
            };
            println!("\t\t...elapsed: {:.2?}", now.elapsed());
            res
        },

        "while" => {
            print!("Using while: ");
            let now = Instant::now();
            let res = match verbose {
                false => reduced_syracuse_bitwise_while(n),
                true => reduced_syracuse_bitwise_while_with_count(n),
            };
            println!("\t\t...elapsed: {:.2?}", now.elapsed());
            res
        },
        "reduced" => {
            print!("Using reduced: ");
            let now = Instant::now();
            let res = match verbose {
                false => reduced_bitwise(n),
                true => reduced_bitwise_with_count(n),
            };
            println!("\t\t...elapsed: {:.2?}", now.elapsed());
            res
        },

        "bitwise" => {
            print!("Using while: ");
            let now = Instant::now();
            let res = match verbose {
                false => bitwise(n),
                true => bitwise_with_count(n),
            };
            println!("\t\t...elapsed: {:.2?}", now.elapsed());
            res
        },
        _ => {
            print!("Using basic: ");
            let now = Instant::now();
            let res = match verbose {
                false => basic(n),
                true => basic_with_count(n),
            };
            println!("\t\t...elapsed: {:.2?}", now.elapsed());
            res
        },
    }

}



fn bitwise(n: &BigUint) -> bool{
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();

    while i != one {
        //if &i % &two == zero { //VERY LONG !
        if i.is_even() {
            i = &i >> 1;
        }
        else {
            i = (&i <<1) + &i + &one ;
        }
    }
    true
}

fn bitwise_with_count(n: &BigUint)-> bool{
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();

    let mut count_divide = 0;
    let mut max: BigUint = i.clone();
    let mut count_multiply = 0;

    while i != one {
        if i.is_even() {
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
        print!("*: {count_multiply} , / {count_divide}\r");

    }
    let total_iterations = &count_multiply + &count_divide;
    let iters = total_iterations.to_formatted_string(&Locale::fr);
    println!("Iterations = {iters}");
    println!("*: {count_multiply}, / {count_divide}");
    let max_repr = crop_biguint(&max,100);
    println!("\t Max = {} \n\t Iterations = {total_iterations}",max_repr);
    true
}

pub fn syracuse_bitwise(n: &BigUint, verbose: bool) -> bool{
    match verbose {
        false => bitwise(n),
        true => bitwise_with_count(n),
    }
}

fn reduced_bitwise(n: &BigUint) -> bool{
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    while i != one {
        if i.is_odd() {
            i = ((&i <<1) + &i + &one) >> 1;
        }
        else {
            i = &i >> 1;
        }
    }
    true
}

fn reduced_bitwise_with_count(n: &BigUint) -> bool{
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    let mut count_divide = 0;
    let mut count_multiply = 0;
    while i != one {
        if i.is_odd() {
            count_multiply += 1;
            count_divide +=1;
            i = ((&i <<1) + &i + &one) >> 1;
        }
        else {
            count_divide +=1;
            i = &i >> 1;
        }
    }
    let total_iterations = &count_multiply + &count_divide;
    let iters = total_iterations.to_formatted_string(&Locale::fr);
    println!("Iterations = {iters}");
    println!("*: {count_multiply}, / {count_divide}");
    true
}

pub fn syracuse_reduced_bitwise(n: &BigUint, verbose: bool) -> bool{
    match verbose {
        false => reduced_bitwise(n),
        true => reduced_bitwise_with_count(n),
    }
}

fn reduced_syracuse_bitwise_while_with_count(n: &BigUint) -> bool {
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    let mut count_divide = 0;
    let mut count_multiply = 0;
    while i != one {
        while i.is_odd() {
            count_multiply += 1;
            i = ((&i <<1) + &i + &one) >> 1;
        }
        while i.is_even() {
            count_divide +=1;
            i >>= 1;
        }
        print!("*: {count_multiply} , / {count_divide}\r");
    }
    let total_iterations = &count_multiply + &count_divide;
    println!("Iterations = {total_iterations}");
    println!("*: {count_multiply}, / {count_divide}");
    true
}

fn reduced_syracuse_bitwise_while(n: &BigUint) -> bool{
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    while i != one {
        while i.is_odd() {
            i = ((&i <<1) + &i + &one) >> 1;
        }
        while i.is_even() {
            i >>= 1;
        }
    }
    true
}

pub fn syracuse_reduced_bitwise_while(n: &BigUint, verbose: bool) -> bool{
    match verbose {
        true => reduced_syracuse_bitwise_while_with_count(n),
        false => reduced_syracuse_bitwise_while(n),
    }
}


pub fn syracuse_optimum(n: &BigUint, verbose: bool) -> bool{
    match verbose {
        true => optimum_syracuse_with_count(n),
        false => optimum_syracuse(n),
    }
}

fn optimum_syracuse(n: &BigUint) -> bool {
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    if i.is_even() {
        let a: u64 = i.trailing_zeros().unwrap();
        i = i >> a;
    }
    if i == one {
        return true;
    }
    loop {
        i = (&i << 1) + &i + &one >> 1;
        // the following line is worse :
        //i = &i >> &i.trailing_zeros().unwrap();
        let a: u64 = i.trailing_zeros().unwrap();
        i = &i >> &a;
        if i == one{
            break;
        }
    }
    true
}

fn optimum_syracuse_with_count(n: &BigUint) -> bool{
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    let mut counter: u64= 0;
    if i.is_even() {
        let a: u64 = i.trailing_zeros().unwrap();
        i = i >> a;
        counter += a;
    }
    if i == one {
        println!("{}",counter);
        return true;
    }
    loop {
        i = (&i << 1) + &i + &one >> 1;
        counter += 2;
        // the following line is worse :
        //i = &i >> &i.trailing_zeros().unwrap();
        let a: u64 = i.trailing_zeros().unwrap();
        i = &i >> &a;
        counter += a;
        if i == one{
            break;
        }
    }
    println!("{}", counter);
    true
}
