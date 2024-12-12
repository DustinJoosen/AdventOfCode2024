
#[derive(Debug)]
pub struct NumberTable {
	pub grid: Vec<Vec<i32>>,
}

impl NumberTable {

	/// Creates a new NumberTable.
	///
	/// # Arguments
	///
	/// * `grid` - the grid to base the table on.
	///
	/// # Returns
	/// The newly created ParsingNumberTable object.
	///
	pub fn new(grid: Vec<Vec<i32>>) -> NumberTable {
		NumberTable { grid }
	}

	/// Creates a new table and parses the string
	///
	/// # Expected input format
	/// "153\n586\n907"
	///
	pub fn from(content: String) -> NumberTable {
		let grid = content.lines()
			.map(|line| line.chars()
				.filter_map(|c| c.to_digit(10).map(|d| d as i32))
				.collect()
			).collect();

		Self::new(grid)
	}

	/// Creates a new table and parses the string
	///
	/// # Expected input format
	/// "1 5 3 \n5 8 6 \n9 0 7 "
	///
	pub fn from_whitespace(content: String) -> NumberTable {
		let grid = content.lines()
			.map(|line| line.split_whitespace()
				.filter_map(|number| number.parse::<i32>().ok())
				.collect()
			).collect();

		Self::new(grid)
	}


	/// Gets a value out of a specified cell.
	///
	/// # Arguments
	///
	/// * `i` - The row of the cell.
	/// * `j` - The column of the cell.
	///
	/// # Returns
	/// An Option<i32> either containing Some(value), or None, if the cell is not found.
	///
	pub fn get_cell(&self, i: i32, j: i32) -> Option<i32> {
		if self.grid.is_empty() ||
			self.get_height() < i + 1 ||
			self.get_width() < j + 1 {
			return None
		}

		Some(self.grid[i as usize][j as usize])
	}

	/// Sets a value in a specified cell.
	///
	/// # Arguments
	///
	/// * `i` - The row of the cell.
	/// * `j` - The column of the cell.
	/// * `value` - The new value.
	///
	pub fn set_cell(&mut self, i: usize, j: usize, value: i32) {
		self.grid[i][j] = value;
	}

	/// Displays the table in the console.
	///
	pub fn display(&self) {
		for line in &self.grid {
			for c in line {
				print!("{c}");
			}
			println!();
		}
	}

	/// Counts how often a specific digit occurs in the table.
	///
	/// # Arguments
	///
	/// * `n` - the digit to count
	///
	/// # Returns
	/// An Option<i32> either containing Some(value), or None, if the digit does not occur.
	///
	pub fn count_occurrences(&self, n: i32) -> Option<i32> {
		let mut count: i32 = 0;

		for line in &self.grid {
			count += line.iter().filter(|&&x| x == n).count() as i32;
		}

		if count <= 0 {
			return None
		}

		Some(count)
	}

	/// Retrieves the width of the table
	///
	/// # Returns
	/// An i32 of the width of the table.
	///
	pub fn get_width(&self) -> i32 {
		if self.grid.is_empty() || self.grid[0].is_empty() {
			return 0
		}

		self.grid[0].len() as i32
	}

	/// Retrieves the height of the table
	///
	/// # Returns
	/// An i32 of the height of the table.
	pub fn get_height(&self) -> i32 {
		if self.grid.is_empty() {
			return 0
		}

		self.grid.len() as i32
	}
}