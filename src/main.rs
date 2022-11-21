use num_bigint::{ToBigUint,BigUint, RandBigInt};
use num_traits::{One,ToPrimitive};
use num_format::{Locale, ToFormattedString};
use num_integer::Integer;
//use indicatif::{ProgressBar,ProgressStyle};
use std::time::Instant;
use std::{io, thread};
use clap::Parser;

use crate::algos::{crop_biguint, syracuse, syracuse_bitwise, syracuse_reduced_bitwise, syracuse_reduced_bitwise_while, syracuse_optimum};
pub mod algos;

#[derive(Parser,Default,Debug)]
#[clap(author = "Author: Eric Tellier", version, about)]
/// An comparison of different implementations of the Collatz conjecture sequence for big integer
/// (2^(2^32-1)-1
struct Arguments {
    #[arg(short, long, default_value_t=String::new())]
    test: String,
    #[arg(short, long)]
    power: Option<u32>,
    #[arg(short, long)]
    decay: Option<u32>,
    #[arg(short, long)]
    huge: Option<u32>,
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
        if i.is_odd() {
            i = ((&i <<1) + &i + &one) >> 1;
        }
        else {
            i = &i >> 1;
        }
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
    let mut rng = rand::thread_rng();
    let count = thread::available_parallelism()?.get();
    println!("{count}");
    let my_big_number  = rng.gen_biguint(300_000);
    print!("Using optimal incremental: ");
    let now = Instant::now();
    opt_incremental_syracuse(&my_big_number);
    println!("\t\t...elapsed: {:.2?}", now.elapsed());

    print!("Using incremental: ");
    let now = Instant::now();
    incremental_syracuse(&my_big_number);
    println!("\t\t...elapsed: {:.2?}", now.elapsed());

    println!("{}",crop_biguint(&my_big_number, 100));
    let algos = ["optimum","while","reduced",""];
    thread::spawn(move || {
        for i in algos {
             syracuse(&my_big_number, false, i);
        }
    });

    //
    //

    //
    // print!("Using reduced bitwise while: ");
    // let now = Instant::now();
    // syracuse(&my_big_number, false, "while");
    // println!("\t\t...elapsed: {:.2?}", now.elapsed());
    //
    // print!("Using reduced bitwise : ");
    // let now = Instant::now();
    // syracuse(&my_big_number, false, "reduced");
    // println!("\t\t...elapsed: {:.2?}", now.elapsed());
    //
    // print!("Using bitwise");
    // let now = Instant::now();
    // syracuse(&my_big_number, false,"bitwise");
    // println!("\t\t...elapsed: {:.2?}", now.elapsed());
    //
    // print!("Using basic implementation");
    // let now = Instant::now();
    // syracuse(&my_big_number, false, "");
    // println!("\t\t...elapsed: {:.2?}", now.elapsed());

    Ok(())
}


fn main()-> io::Result<()>  {
    let two = 2.to_biguint().unwrap();
    let args = Arguments::parse();
    if args.test.trim().is_empty() {
        let my_big_number: BigUint;
        print!("Input: ");
        if let Some(n) = args.power {
                print!("2 ^ {n}");
                my_big_number = BigUint::pow(&two,n);
        }
        else {
            if let Some(n) = args.huge{
                let s = n.to_formatted_string(&Locale::fr);
                print!("2 ^ 100 ^ ({})",s);
                let p = BigUint::pow(&two,100.to_u32().unwrap());
                my_big_number = BigUint::pow(&p, n);
            }
            else {
                println!("Picking a random number");
                let mut rng = rand::thread_rng();
                my_big_number = rng.gen_biguint(1000);
            }
        }
        let k = match args.decay {
            Some(n) => {
                print!(" + {}",n);
                n.to_biguint().unwrap()
            },
            None => 0.to_biguint().unwrap(),
        };
        let my_big_number = my_big_number + k;
        let my_bn_str = crop_biguint(&my_big_number,100);
        println!("\n{}", my_bn_str);
        let now = Instant::now();
        println!("Using optimum: ");
        syracuse_optimum(&my_big_number,true);
        println!("\t\t...elapsed: {:.2?}", now.elapsed());

        let now = Instant::now();
        println!("Using bitwise : ");
        syracuse_bitwise(&my_big_number, false);
        println!("\t\t...elapsed: {:.2?}", now.elapsed());

    }
    else {
        println!("Benchmarking:");
        benchmark().unwrap();
    }

    Ok(())
}
