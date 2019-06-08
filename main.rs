//Phuong Pham
//CS 510 Rust Programming
//Spring 2019

use std::io::Write;
use std::str::FromStr;

fn sum(x: u64, y: u64) -> u64
{
  x + y
}

fn product(x: u64, y: u64) -> u64
{
  x * y
}

fn gcd(mut n: u64, mut m: u64) -> u64 
{
  assert!(n != 0 && m != 0);
  while m != 0 
  {
    if m < n
    {
      let t = m;
      m = n;
      n = t;
    }
    m = m % n;
  }
  n
}

fn lcm(x: u64, y: u64) -> u64
{
  (x * y) / gcd(x, y)  
}

fn main() 
{
  let mut numbers = Vec::new();

  //Read in all the arguments after the function (i.e. the numbers)
  for arg in std::env::args().skip(2)
  {
    numbers.push(u64::from_str(&arg).expect("Error parsing arguments."));
  }

  //Check if there are actually any numbers. 
  if numbers.len() == 0
  {
    println!("0");
    std::process::exit(0);
  }

  if numbers.len() == 1
  {
    println!("{:?}", numbers);
    std::process::exit(0);
  }

  let argue: Vec<_> = std::env::args().collect();
  let mut d = numbers[0];
  
  //See which function was called.
  if argue[1] == "gcd"
  {
    println!("You have chosen the gcd function.");
    for m in &numbers[1..]
    {
      d = gcd(d, *m);
    }
    println!("The greatest common divisor of {:?} is {}", numbers, d);
  }
  else if argue[1] == "sum"
  {
    println!("You have chosen the sum function.");
    for m in &numbers[1..]
    {
      d = sum(d, *m);
    }
    println!("The sum of {:?} is {}", numbers, d);
  }
  else if argue[1] == "product"
  {
    println!("You have chosen the product function.");
    for m in &numbers[1..]
    {
      d = product(d, *m);
    }
    println!("The product of {:?} is {}", numbers, d);
  }
  else if argue[1] == "lcm"
  {
    println!("You have chosen the lcm function.");
    for m in &numbers[1..]
    {
      d = lcm(d, *m);
    }
    println!("The lcm of {:?} is {}", numbers, d);
  }
  else
  {
    writeln!(std::io::stderr(), "Usage: Please chooose between sum, produce, gcd, or lcm for functions.").unwrap();
    std::process::exit(1);
  }
}

#[test]
fn test_sum()
{
  assert_eq!(sum(23,34), 57);
  assert_eq!(sum(2*3+7, 8-2*3), 15);
}

#[test]
fn test_product()
{
  assert_eq!(product(12,34), 408);
  assert_eq!(product(506-34/2, 55+20+10), 41565);
}

#[test]
fn test_gcd()
{
  assert_eq!(gcd(14,15), 1);

  assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}

#[test]
fn test_lcm()
{
  assert_eq!(lcm(65, 10, 5), 130);
  assert_eq!(lcm(25*4+9, 9*5, 54/2), 14715);
}


/*for arg in std::env::args().skip(1)
  {
    println!("{:?}", arg);
    if arg == "gcd"
    {
      for arg in std::env::args().skip(2)
      {
        numbers.push(u64::from_str(&arg).expect("Error parsing arguments."));
      }
      break
    }
    else
    {
      writeln!(std::io::stderr(), "Not implemented yet.").unwrap();
      std::process::exit(1);
    }
  }*/


  //let func = std::env::args().nth(1).expect("No function name given.");
  //println!("Function name is: {:?}", func);

  /*if std::env::args().nth(1) == "gcd"
  {
    for arg in std::env::args().skip(1)
    {
      numbers.push(u64::from_str(&arg).expect("Error parsing arguments."));
    }
  }
  else
  {
    writeln!(std::io::stderr(), "Not implemented yet.").unwrap();
    std::process::exit(1);
  }*/

  /*for arg in std::env::args()
  {
    if arg == "gcd"
    {
      for arg in std::env::args().skip(1)
      {
        numbers.push(u64::from_str(&arg).expect("Error parsing arguments."));
      }
    }
    else
    {
      writeln!(std::io::stderr(), "Not implemented yet.").unwrap();
      std::process::exit(1);
    }
  }*/
 
/*
  //let args: Vec<_> = env::args().collect();
  println!("Hello there.");
  //if args.len() > 1
  //{
    //println!("First argument: {}", args[1]);
  let x = 20;
  let y = 30;
  let z = gcd(x, y);
  print!("Value of z: ");
  println!("{}", z);
  //}
*/
