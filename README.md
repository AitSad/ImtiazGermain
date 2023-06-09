# ImtiazGermain 
ImtiazGermain is a library intended for number theory, implementing an edited version of Germain primes, which got published in the book [A Young Mathematician](https://www.amazon.com/Young-Mathematician-Aitzaz-Imtiaz-ebook/dp/B0BCH5DWH9/ref=sr_1_4?crid=20G0TUBL90ZKA&keywords=aitzaz+imtiaz&qid=1681238879&sprefix=%2Caps%2C686&sr=8-4). Here is a algorithm for ImtiazGermain primes:

```
Input number n --> Calculate p = (n - 1) / 2 --> Check if n, p, and 2p + 1 are prime ------------------> 
|                                                  |                                                    |
|                                                  No                                                   |
|                                                  |                                                    |
|                                                  V                                                    |
Output "n is not an Imtiaz Germain prime"     <----- Stop                                               |
|                                                                                                       |
Yes                                                                                                     |
|                                                                                                       |
V                                                                                                       |
Calculate m = 2p + 1 --> Check if m is composite --> Calculate q = 2m + 1 --> Check if q is prime -No-->|
|                                                  |                              |                     
|                                                  No                             Yes                   
|                                                  |                              |                     
|                                                  V                              V                     
Output "n is not an Imtiaz Germain prime"     <----- Stop        Output "n is an Imtiaz Germain prime"  

```

## Features

- Germain Primes detection
- Imtiaz-Germain primes detection


## Installation

To use ImtiazGermain in your Rust project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
ImtiazGermain = "0.1.1"
```

Alternatively, you can go on python as:

```
pip install imtiazgermain
```


## Usage
To use ImtiazGermain in your Rust code, import the necessary modules:

```rust
use ImtiazGermain::primecheck::{isgermainprime, isimtiazgermainprime};
```

Then, call the desiredfunction. For example, to check if a number is ImtiazGermain or not:

```rust
use std::io;
use ImtiazGermain::primecheck::{isgermainprime, isimtiazgermainprime};

fn main() {
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u64>().unwrap();

    if primecheck::isimtiazgermainprime(n) {
        println!("{} is an Imtiaz Germain prime", n);
    } else {
        println!("{} is not an Imtiaz Germain prime", n);
    }
}
```

You can use python as follows:
```python
import ImtiazGermain.primecheck as pm

pc = primecheck()
print(pc.isgermainprime(2))    # True
print(pc.isimtiazgermainprime(3))    # True
print(pc.isgermainprime(5)) #True
print(pc.isgermainprime(7))    # True
```
