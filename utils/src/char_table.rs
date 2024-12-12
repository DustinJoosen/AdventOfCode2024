use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct CharTable {
	pub grid: Vec<Vec<char>>,
}

impl CharTable {

	/// Creates a new CharTable.
	///
	/// # Arguments
	///
	/// * `grid` - the grid to base the table on.
	///
	/// # Returns
	/// The newly created CharTable object.
	///
	pub fn new(grid: Vec<Vec<char>>) -> CharTable {
		CharTable { grid }
	}

	/// Creates a new table and parses the string
	///
	/// # Expected input format
	/// "ABC\nDEF\nGHI"
	///
	pub fn from(content: String) -> CharTable {
		let grid = content.lines()
			.map(|line| line.chars().collect())
			.collect();

		Self::new(grid)
	}

	// /// Creates a new table and parses the string
	// ///
	// /// # Expected input format
	// /// "A B C \nD E F \nG H I "
	// ///
	// pub fn from_whitespace(content: String) -> CharTable {
	// 	let grid = content.lines()
	// 		.map(|line| {
	// 			let split = line.split_whitespace();
	// 			split.map(|line| line.chars())
	// 		})
	// 		.collect::<Vec<Vec<char>>>();
	//
	// 	Self::new(grid)
	// }


	/// Gets a value out of a specified cell.
	///
	/// # Arguments
	///
	/// * `i` - The row of the cell.
	/// * `j` - The column of the cell.
	///
	/// # Returns
	/// An Option<char> either containing Some(value), or None, if the cell is not found.
	///
	pub fn get_cell(&self, i: i32, j: i32) -> Option<char> {
		if i < 0 || j < 0 ||
			self.grid.is_empty() ||
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
	pub fn set_cell(&mut self, i: usize, j: usize, value: char) {
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

	/// Counts how often a specific char occurs in the table.
	///
	/// # Arguments
	///
	/// * `c` - the char to count
	///
	/// # Returns
	/// An Option<i32> either containing Some(value), or None, if the digit does not occur.
	///
	pub fn count_occurrences(&self, c: char) -> Option<i32> {
		let mut count: i32 = 0;

		for line in &self.grid {
			count += line.iter().filter(|&&x| x == c).count() as i32;
		}

		if count <= 0 {
			return None
		}

		Some(count)
	}

	/// Finds the first occurence of a character in the table.
	///
	pub fn first_occurrence(&self, c: char) -> Option<(i32, i32)> {
		for i in 0..self.grid.len() {
			for j in 0..self.grid[i].len() {
				if &self.grid[i][j] != &c {
					continue;
				}

				return Some((i as i32, j as i32))
			}
		}

		None
	}

	/// Retrieves all distinct values in the table.
	///
	pub fn get_distinct_values(&self) -> Vec<char> {
		let mut distinct_values = vec![];

		let mut distinct_set: HashSet<char> = HashSet::new();

		for line in &self.grid {
			let hashset: HashSet<char> = line.iter().cloned().collect();
			distinct_set.extend(hashset);
		}

		distinct_values.extend(distinct_set);
		distinct_values
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