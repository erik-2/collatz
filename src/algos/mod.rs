use std::time::{Instant, Duration};
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
            let s = (*n).to_formatted_string(&Locale::fr);
            if &s.len() < &size {
                return s;
            }
        }
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
    repr
}

pub fn print_results(input:(u64,u64,Duration)) -> (){
    let (mult_counter, div_counter, time) = input;
    let total_iterations = &mult_counter + &div_counter;
    let iters = total_iterations.to_formatted_string(&Locale::fr);
    let mul = mult_counter.to_formatted_string(&Locale::fr);
    let div = div_counter.to_formatted_string(&Locale::fr);
    println!("Iterations = {iters} : * {mul}, / {div}");
    println!("Computation time: {:.2?}", time);
}

pub fn format_results(input:(u64,u64,Duration)) -> String{
    let (mult_counter, div_counter, time) = input;
    format!("{mult_counter},{div_counter},{:.2?}", time)
}


pub fn optimum_syracuse(n: &BigUint) -> (u64, u64, Duration){
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    let mut div_counter: u64 = 0;
    let mut mult_counter: u64 = 0;
    let mut min: BigUint = n.clone();
    let now = Instant::now();
    if i.is_even() {
        let a: u64 = i.trailing_zeros().unwrap();
        i = i >> a;
        div_counter += a;
    }
    loop {
        if i == one {
            break;
        }
        i = (&i << 1) + &i + &one >> 1;
        div_counter += 1;
        mult_counter +=1;
        let a: u64 = i.trailing_zeros().unwrap();
        i = &i >> &a;
        div_counter += a;
        if i < min {
            min = i.clone();
        } else if i == min {
            panic!("-------------- Loop found ! -----------------")
        }
        let total = &div_counter + &mult_counter;
        print!("{total} iterations\r");
    }
    println!("");
    (mult_counter,div_counter,now.elapsed())
}


pub fn incremental(n: &BigUint) -> bool{
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    let min: BigUint = i.clone();
    let now = Instant::now();
    if i < (&one << 64) {
        return true;
    }
    if i.is_even() {
        let a: u64 = i.trailing_zeros().unwrap();
        i = &i >> &a;
    }
    loop {
        if now.elapsed().as_secs() > 10*60 {
            println!("Timeout for n= {min}");
        }

        i = ((&i << 1) + &i + &one) >> 1;
        let a: u64 = i.trailing_zeros().unwrap();
        //i = i >> a; is longer !
        i = &i >> &a;
        if i == one || i < min{
            break;
        }
    }
    println!("True, Computation time: {:.2?}", now.elapsed());
    return true;
}
