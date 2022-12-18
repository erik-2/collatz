# About
See the [wiki](https://github.com/erik-2/collatz/wiki) page

This Rust program is a fast implementation of an algorithm that verifies the Collatz conjecture. It is specifically designed to check the conjecture for very large numbers. The input is done using a power of two to which an integer between 1 and 2^32-1 can be added.

# Use
* Use -p to add to input a power of two

* Use -q to add a power of power of two (max = 31)

* Use -a to add an integer between 1 and 2^32 - 1

* Use -m to substract an integer between 1 and 2^32 - 1

# Examples:
For number $2^{99} + 1$
```console
./collatz -p 99 -a 1 
```
For number $2^{2^{9}} + 1$
```console
./collatz -q 9 -a 1
```
