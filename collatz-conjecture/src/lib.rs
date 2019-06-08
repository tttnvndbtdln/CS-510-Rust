pub fn collatz(n: u64) -> Option<u64> {
    if n == 0
    {
      return None;
    }
    let mut x = n;
    let mut count = 0;
    while x != 1
    {
      if x % 2 == 0
      {
        x = x / 2;
      }
      else
      {
        x = 3 * x + 1
      }
      count += 1;
    }
    Some(count)
}
