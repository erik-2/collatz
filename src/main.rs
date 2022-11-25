use num_bigint::{ToBigUint,BigUint, RandBigInt};
use num_traits::{One, Zero};
use num_format::{Locale, ToFormattedString};
use num_integer::Integer;
//use indicatif::{ProgressBar,ProgressStyle};
use std::time::Instant;
use std::{io, thread};
use clap::{Arg, Command};
use std::process::exit;
use crate::algos::{crop_biguint, syracuse};
pub mod algos;


fn incremental_syracuse(n: &BigUint) -> bool{
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    let min: BigUint = i.clone();
    let now: Instant = Instant::now();
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
        i = if i.is_odd() {
            ((&i <<1) + &i + &one) >> 1
        }
        else {
            &i >> 1
        };
    }
    return true;
}

fn opt_incremental_syracuse(n: &BigUint) -> bool{
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
    return true;
}

fn benchmark() -> io::Result<()> {
    let bit_size = 100_000;
    println!("Bit size: {}",bit_size.to_formatted_string(&Locale::fr));
    let mut rng = rand::thread_rng();
    let my_big_number  = rng.gen_biguint(bit_size);

    print!("Using optimal incremental: ");
    let now = Instant::now();
    opt_incremental_syracuse(&my_big_number);
    println!("\t\t...elapsed: {:.2?}", now.elapsed());

    print!("Using incremental: ");
    let now = Instant::now();
    incremental_syracuse(&my_big_number);
    println!("\t\t...elapsed: {:.2?}", now.elapsed());

    println!("{}",crop_biguint(&my_big_number, 100));

    syracuse(&my_big_number,true, "optimum");
    // NOT WORKING
    let algos = ["optimum","while","reduced",""];
    thread::spawn(move || {
        for i in algos {
             syracuse(&my_big_number, true, i);
        }
    });

    Ok(())
}



fn main()-> io::Result<()>  {
    let two = 2.to_biguint().unwrap();
    let matches = Command::new("Collatz computing program")
                    .version("0.1.0")
                    .author("Eric Tellier <eric.tellier@newick.fr>")
                    .about("ifferent implementations of the Collatz conjecture sequence for big integer (2^(2^32-1)-1)")
                    .arg(Arg::new("benchmark")
                            .short('t')
                            .long("test")
                            .exclusive(true)
                            .action(clap::ArgAction::SetTrue)
                            .help("benchmark with a random number"))
                    .arg(Arg::new("power")
                            .short('p')
                            .long("power")
                            .action(clap::ArgAction::Set)
                            .help("add 2^n to the input number"))
                    .arg(Arg::new("quad")
                            .short('q')
                            .long("quad")
                            .action(clap::ArgAction::Set)
                            .help("add 2^2^n to the input number"))
                    .arg(Arg::new("add")
                            .short('a')
                            .long("add")
                            .help("add n to the input number"))
                    .get_matches();
    if Some(clap::parser::ValueSource::CommandLine) == matches.value_source("benchmark"){
        println!("Benchmarking:");
        benchmark().unwrap();
        exit(0);
    }
    let zero: BigUint = Zero::zero();

    let mut my_big_number: BigUint = Zero::zero();
    print!("Input: ");
    if let Some(n_str) = matches.get_one::<String>("quad") {
        let n = n_str.parse::<u32>().unwrap();
        if n > 31 {
            println!("Number too large 2^2^q, q must be < 32!");
            exit(1);
        }
        let s = n.to_formatted_string(&Locale::fr);
        print!("2 ^ 2 ^({})",s);
        let p = u32::pow(2,n);
        print!("= 2 ^ {}",p.to_formatted_string(&Locale::fr));
        my_big_number += BigUint::pow(&two,p);
    }

    if let Some(n_str) = matches.get_one::<String>("power") {
        let n = n_str.parse::<u32>().unwrap();
        let s = n.to_formatted_string(&Locale::fr);
        if my_big_number > zero {
            print!(" + 2 ^{}",n)
        }
        else {
            print!("2 ^ {}",s);
        }
        my_big_number += BigUint::pow(&two,n)
    }
    
    if let Some(n_str) = matches.get_one::<String>("add") {
        let n = n_str.parse::<u32>().unwrap();
        print!(" + {}",n);
        my_big_number += n.to_biguint().unwrap();
    }
    println!("");
        
    if my_big_number == zero {
        println!("Picking a random number");
        let mut rng = rand::thread_rng();
        my_big_number = rng.gen_biguint(1000);
    }

    let my_bn_str = crop_biguint(&my_big_number,100);
    println!("\n{}", my_bn_str);
    syracuse(&my_big_number,true,"optimum");

    //syracuse(&my_big_number,true,"bitwise");

    Ok(())
}
