use std::collections::HashMap;

pub fn run(content: String) -> i32 {
	// part1(content)
	part2(content)
}

fn part2(content: String) -> i32 {
	let parsed = parse(content);
	let mut grid = Grid::new(parsed.len(), parsed[0].len());

	let mut frequencies: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

	// Fill grid.
	for y in 0..parsed.len() {
		for x in 0..parsed[y].len() {
			let freq: Option<char> = if parsed[y][x] != '.' { Some(parsed[y][x]) } else { None };
			let grid_cell = GridCell::new(x, y, freq, false);

			if freq.is_some() {
				frequencies.entry(freq.unwrap())
					.or_insert_with(Vec::new)
					.push((y, x));
			}

			grid.add_grid_cell(grid_cell);
		}
	}

	// Check grid for frequencies.
	for frequency in frequencies {
		for antenna_a in &frequency.1 {
			for antenna_b in &frequency.1 {
				if antenna_a == antenna_b {
					continue;
				}

				let antinodes = calculate_antinodes_part_2(&grid, *antenna_a, *antenna_b);

				for antinode in antinodes {
					if antinode.0 >= 0 && antinode.0 < grid.height &&
						antinode.1 >= 0 && antinode.1 < grid.width {
						let mut grid_cell: GridCell = grid._grid[antinode.0][antinode.1].unwrap();
						grid_cell.contains_antinode = true;
						grid._grid[antinode.0][antinode.1] = Some(grid_cell);
					}
				}
			}
		}
	}

	grid.count_antinodes()
}

fn part1(content: String) -> i32 {
	let parsed = parse(content);
	let mut grid = Grid::new(parsed.len(), parsed[0].len());

	let mut frequencies: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

	// Fill grid.
	for y in 0..parsed.len() {
		for x in 0..parsed[y].len() {
			let freq: Option<char> = if parsed[y][x] != '.' { Some(parsed[y][x]) } else { None };
			let grid_cell = GridCell::new(x, y, freq, false);

			if freq.is_some() {
				frequencies.entry(freq.unwrap())
					.or_insert_with(Vec::new)
					.push((y, x));
			}

			grid.add_grid_cell(grid_cell);
		}
	}

	// Check grid for frequencies.
	for frequency in frequencies {
		for antenna_a in &frequency.1 {
			for antenna_b in &frequency.1 {
				if antenna_a == antenna_b {
					continue;
				}

				let antinodes = calculate_antinodes(*antenna_a, *antenna_b);

				// If OOB, continue.
				if antinodes.0.0 >= 0 && antinodes.0.0 < grid.height &&
					antinodes.0.1 >= 0 && antinodes.0.1 < grid.width {
					let mut grid_cell_a: GridCell = grid._grid[antinodes.0.0][antinodes.0.1].unwrap();
					grid_cell_a.contains_antinode = true;
					grid._grid[antinodes.0.0][antinodes.0.1] = Some(grid_cell_a);
				}

				if antinodes.1.0 >= 0 && antinodes.1.0 < grid.height &&
					antinodes.1.1 >= 0 && antinodes.1.1 < grid.width {
					let mut grid_cell_b: GridCell = grid._grid[antinodes.1.0][antinodes.1.1].unwrap();
					grid_cell_b.contains_antinode = true;
					grid._grid[antinodes.1.0][antinodes.1.1] = Some(grid_cell_b);
				}
			}
		}
	}

	grid.count_antinodes()
}

fn calculate_antinodes_part_2(grid: &Grid, antenna_a: (usize, usize), antenna_b: (usize, usize))
	-> Vec<(usize, usize)> {

	let mut antinodes: Vec<(usize, usize)> = vec![];

	// Antenna a.
	let x_diff: isize = antenna_a.1 as isize - antenna_b.1 as isize;
	let y_diff: isize = antenna_a.0 as isize - antenna_b.0 as isize;

	let mut current_antinode = antenna_a;
	loop {
		let next_antinode_a = (
			(current_antinode.0 as isize + y_diff) as usize,
			(current_antinode.1 as isize + x_diff) as usize,
		);

		if next_antinode_a.0 < 0
			|| next_antinode_a.1 < 0
			|| next_antinode_a.0 >= grid.height
			|| next_antinode_a.1 >= grid.width {
			break;
		}

		current_antinode = (next_antinode_a.0, next_antinode_a.1);

		if current_antinode.0 >= 0 && current_antinode.0 < grid.height &&
			current_antinode.1 >= 0 && current_antinode.1 < grid.width {
			antinodes.push(current_antinode);
			println!("{:?}", current_antinode);
		}
	}

	let x_diff: isize = antenna_b.1 as isize - antenna_a.1 as isize;
	let y_diff: isize = antenna_b.0 as isize - antenna_a.0 as isize;
	loop {
		let next_antinode_b = (
			(current_antinode.0 as isize + y_diff) as usize,
			(current_antinode.1 as isize + x_diff) as usize
		);

		if next_antinode_b.0 < 0
			|| next_antinode_b.1 < 0
			|| next_antinode_b.0 >= grid.height
			|| next_antinode_b.1 >= grid.width {
			break;
		}

		current_antinode = (next_antinode_b.0, next_antinode_b.1);

		if current_antinode.0 >= 0 && current_antinode.0 < grid.height &&
			current_antinode.1 >= 0 && current_antinode.1 < grid.width {
			antinodes.push(current_antinode);
			println!("{:?}", current_antinode);
		}
	}


	antinodes
}


fn calculate_antinodes(antenna_a: (usize, usize), antenna_b: (usize, usize))
   -> ((usize, usize), (usize, usize)) {

	// Antenna a.
	let x_diff: isize = antenna_a.1 as isize - antenna_b.1 as isize;
	let y_diff: isize = antenna_a.0 as isize - antenna_b.0 as isize;
	let antinode_a = (
		(antenna_a.0 as isize + y_diff) as usize,
		(antenna_a.1 as isize + x_diff) as usize,
	 );

	// Antenna b.
	let x_diff: isize = antenna_b.1 as isize - antenna_a.1 as isize;
	let y_diff: isize = antenna_b.0 as isize - antenna_a.0 as isize;
	let antinode_b = (
		(antenna_b.0 as isize + y_diff) as usize,
		(antenna_b.1 as isize + x_diff) as usize
	);

	(antinode_a, antinode_b)
}

#[derive(Clone, Copy, Debug)]
struct GridCell {
	x: usize,
	y: usize,
	frequency: Option<char>,
	contains_antinode: bool
}
impl GridCell {
	fn new(x: usize, y: usize, frequency: Option<char>, contains_antinode: bool) -> GridCell {
		GridCell {x, y, frequency, contains_antinode}
	}
}


#[derive(Debug)]
struct Grid {
	_grid: Vec<Vec<Option<GridCell>>>,
	width: usize,
	height: usize
}

impl Grid {
	fn new(height: usize, width: usize) -> Grid {
		let grid = vec![vec![None; width]; height];
		Grid { _grid: grid, width, height }
	}

	fn add_grid_cell(&mut self, grid_cell: GridCell) {
		let (x, y) = (grid_cell.x, grid_cell.y);
		self._grid[y][x] = Some(grid_cell);
	}

	fn count_antinodes(&self) -> i32 {
		let mut count = 0;
		for line in &self._grid {
			for char in line {
				if char.unwrap().contains_antinode {
					count += 1
				}
			}
		}

		count
	}

}

fn parse(content: String) -> Vec<Vec<char>> {
	let lines: Vec<&str> = content.split("\n").collect();
	lines.iter()
		.map(|line| line.chars().collect())
		.take(lines.len() - 1)
		.collect()
}