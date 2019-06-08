Phuong Pham
CS 510 Rust Programming
Spring 2019

Using the gcd example in Chapter 2 from the course text, I was able
to implement the gcd function using the same structure. The second 
half of the chapter also explains how to parse the command line 
argument, which I found helpful in implementing and calling the different
functions, depending on what the input is when doing cargo build/run.

The sum and product functions were easy enough. Some research was 
done to understand the lcm function. Turns out it is quite simple:
Simply take the product of the arguments and divide it by the gcd,
which is already implemented. 

Testing was easy enough for the four different functions, all of which
require simple algebra and assertions statement were made to ensure
that the calculation was correct. For example, the sum function was 
tested using the assertion and knowing that the sum of 1, 2, and 3
is 6. The test was ran with cargo test.
