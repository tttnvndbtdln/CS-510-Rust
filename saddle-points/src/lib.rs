fn get_column(input: &[Vec<u64>], column: usize) -> Vec<u64>
{
  let mut result: Vec<u64> = vec![];

  for row in input.iter()
  {
    for (saddle_column, matrix) in row.iter().enumerate()
    {
      if saddle_column == column
      {
        result.push(matrix.clone())
      }
    }
  }
  result
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
  let mut result = vec![];

  for (saddle_row, row) in input.iter().enumerate()
  {
    let max_row = row.iter().max().unwrap_or(&0);
  
    for (saddle_column, saddle_value) in row.iter().enumerate()
    {
      if saddle_value >= max_row
      {
        let column_num = get_column(input, saddle_column);
	let min_column = column_num.iter().min().unwrap_or(&10_000);

	if saddle_value <= &min_column
	{
	  result.push((saddle_row, saddle_column))
	}
      }
    }
  }
  result
}

