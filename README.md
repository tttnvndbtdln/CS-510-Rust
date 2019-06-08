Phuong Pham
CS 410/510 Rust Programming
Homework 2

The easier exercises (classify as Easy on Exercism) were easy enough to
implement without much troubles. Further research was needed for the 3
medium exercises. At the moment of submission, I still have not worked
out the bugs of saddle-points. This was the hardest one for me, having
to learn how to work with matrices. 

For proverb, I had to research on the string functions (e.g. the .len 
to get the length and .iter to progress through the list). Other function
I found to be useful was push_str to add the string to a current mutable 
object. In addition, format! was needed to get the right format. 

say was another difficult exercise for me. I really had to dig into vector
manipulation. I also had a bit difficulties with learning to manipulate 
the different bits unsigned integers. For example, if an object is 
assigned to a u32, it cannot be used in an operation with a u64. This should
have been obvious but took a while to figure out. Vector functionalities, 
format!, and other operations like .insert were researched for this program.

For the uncomplete saddle points exercise, I am still figuring out how 
to use vector to finish this and unfortunately have run out of time. I will
keep on doing this and submit later. 

UPDATE:
I have fixed and complete the saddle points exercise (quite silly
considering how I figured it out as soon as I submitted the bulk of the 
homework and then a stress seems to be released and I was able to get a 
clear head to look at the problem). I was struggling to manipulate the 
values in the matrix and getting the correct saddle rows and columns.
I also could not figure out how to push just the column and had to 
push the matrix. I then had problems with this, which I then fixed
with a clone.

Fixed bugs to pass all tests.
