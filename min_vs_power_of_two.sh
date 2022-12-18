#!/bin/bash
for ((c=9000000;c<=9001000; c++))
do
    ./target/release/collatz -p $c -a 1 -o "min_series_9000000_9001000.csv"
done
