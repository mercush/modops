# modops
Modular Exponentiation

This repository is for faster modular arithmetic for Risc0. 

The multiplication algorithm is based on this paper: 
https://static1.squarespace.com/static/61f7cacf2d7af938cad5b81c/t/62deb4e0c434f7134c2730ee/1658762465114/modular_multiplication.pdf
Unfortunately, I didn't find a statistically significant improvement between this algorithm and the original modular multiplication algorithm.

I spent a good amount of time trying to build montgomery exponentiation but found out it was implemented by default in a rust package already. 
I found that montgomery exponentiation was slower than naive exponentiation (exponentiation by squaring) was faster for integers of less than 16 bits.
