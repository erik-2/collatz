#!/bin/bash
for ((c=999000;c<=1000000; c++))
do
    ./target/release/collatz -p $c -a 1 -o "min_series_990000_1000000.csv"
done
