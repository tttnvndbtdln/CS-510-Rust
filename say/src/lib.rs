pub fn encode(num: u64) -> String {
  if num == 0 
  { 
    return String::from("zero"); 
  }
  
  let big_num_words = vec![
    "",
    " thousand",
    " million",
    " billion", 
    " trillion", 
    " quadrillion", 
    " quintillion"
  ];
  
  let mut word: Vec<String> = vec![];
  
  for (i, order) in big_num_words.iter().enumerate() 
  {
    //Each iteration works on an order of magnitude in big_num_words
    let temp1 = (i as u32) * 3;

    //Need another temp as temp1 cannot be change due to i as u32 and 
    //need to raise as 64 bits.
    let temp2 = 10u64.pow(temp1);
    let temp = how_big((num / temp2) % 1000);

    //let catch = how_big(temp1);

    if temp != "" 
    {
      word.insert(0, format!("{}{}", temp, order)) 
    }
  }
  
  word.join(" ")
}

pub fn how_big(num: u64) -> String {
  let ones = digits(num % 10);
  let ten_to_twenty = ten_to_twenty(num % 100);
  let twenty_to_ninety = twenty_to_ninety((num % 100)/10);
  let hundreds = digits(num / 100);

  let mut result = String::new();

  //Read the hundreds
  if hundreds != "" 
  { 
    result = format!("{}{} hundred ", result, hundreds) 
  }

  //If there's nothing after hundreds, return, already getting the hundreds above
  if ten_to_twenty != "" 
  { 
    return format!("{}{}", result, ten_to_twenty) 
  } 

  //If there's non-zero digits in tens place and ones place (e.g. 29)
  if twenty_to_ninety != "" && ones != "" 
  { 
    result = format!("{}{}-{}", result, twenty_to_ninety, ones) 
  }

  //If there's only non-zero digits in the tens place (e.g. 30) 
  else if twenty_to_ninety != "" 
  { 
    result = format!("{}{}", result, twenty_to_ninety) 
  }

  //If there's only digits
  else 
  { 
    result = format!("{}{}", result, ones) 
  }

  result.trim().to_string()
}

fn digits(num: u64) -> &'static str 
{
  match num 
  {
    1 => "one",
    2 => "two",
    3 => "three",
    4 => "four",
    5 => "five",
    6 => "six",
    7 => "seven",
    8 => "eight",
    9 => "nine",
    _ => "",
  }
}

fn ten_to_twenty(num: u64) -> &'static str 
{
  match num 
  {
    10 => "ten",
    11 => "eleven",
    12 => "twelve",
    13 => "thirteen",
    14 => "fourteen",
    15 => "fifteen",
    16 => "sixteen",
    17 => "seventeen",
    18 => "eighteen",
    19 => "nineteen",
    _ => "",
  }
}

fn twenty_to_ninety(num: u64) -> &'static str 
{
  //Only interested in the digit that's passed in
  let x = num % 10;

  match x % 10 
  {
    2 => "twenty",
    3 => "thirty",
    4 => "forty",
    5 => "fifty",
    6 => "sixty",
    7 => "seventy",
    8 => "eighty",
    9 => "ninety",
    _ => "",
  }
}
