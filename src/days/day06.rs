
pub fn run(content: String) -> i32 {
	let mut map = parse(content);

	// part1(map)
	part2(map)
}

fn part2(mut map: Vec<Vec<char>>) -> i32 {
	let mut guard = GuardState::new(&map);

	// Loop through all possible locations for a new obstacle.
	let mut count = 0;
	for i in 0..map.len() {
		for j in 0..map[0].len() {

			// Make the new map.
			let mut new_map = map.clone();
			new_map[i][j] = '#';

			guard.reset(&new_map);

			if guard.location == (i as i32, j as i32) {
				// Don't wanna overwrite the guard :)
				println!("Overwriting guard!!!!");
				continue;
			}

			// println!("G->{:?}, L->({i}, {j})", guard.location);
			// print_map(&new_map);
			// println!();

			// Move until it's gone, or if it's in an inf loop.
			while guard.move_once(&mut new_map).is_some() && !guard.is_in_inf_loop { }

			if guard.is_in_inf_loop {
				count += 1;
			}

		}
	}

	count
}

fn part1(mut map: Vec<Vec<char>>) -> i32 {
	let mut guard = GuardState::new(&map);

	// Move guard until it's gone.
	while guard.move_once(&mut map).is_some() {}

	// Count how many x's there are.
	map.iter()
		.flat_map(|map_line| map_line.iter())
		.filter(|&map_line_char| map_line_char == &'X' || map_line_char == &'^')
		.count() as i32
}

fn print_map(map: &Vec<Vec<char>>) {
	for line in map {
		for char in line {
			print!("{}", char);
		}
		println!();
	}
}

#[derive(Debug)]
pub struct GuardState {
	turn_count: usize,
	location: (i32, i32),
	previously_visited_turns: Vec<VisitedTurn>,
	is_in_inf_loop: bool,
}

#[derive(Debug)]
pub struct VisitedTurn {
	turn_location: (i32, i32),
	guard_location: (i32, i32),
}

impl GuardState {
	pub fn new(map: &Vec<Vec<char>>) -> GuardState {
		GuardState {
			turn_count: 0,
			location: Self::get_guard_location(map),
			previously_visited_turns: Vec::new(),
			is_in_inf_loop: false,
		}
	}

	fn reset(&mut self, map: &Vec<Vec<char>>) {
		self.turn_count = 0;
		self.location = Self::get_guard_location(map);
		self.previously_visited_turns.clear();
		self.is_in_inf_loop = false;
	}

	fn get_guard_location(map: &Vec<Vec<char>>) -> (i32, i32) {
		for i in 0..map.len() {
			if map[i].contains(&'^') {
				let j = map[i].iter().position(|&c| c == '^').unwrap();
				return (i as i32, j as i32)
			}
		}

		(-1, -1)
	}

	pub fn move_once(&mut self, mut map: &mut Vec<Vec<char>>) -> Option<()> {
		self.is_in_inf_loop = false;
		let mut next_location = self.get_next_location();

		// Check if out of bounds.
		if next_location.0 < 0 ||
			next_location.1 < 0 ||
			next_location.0 >= (map.len()) as i32 ||
			next_location.1 >= (map[0].len()) as i32 {
			return None;
		}

		// Turn.
		if map[next_location.0 as usize][next_location.1 as usize] == '#' {
			// Check if this turn has already been visited (inf loop)

			if self.previously_visited_turns.iter().any(|visited_turn| {
				return visited_turn.turn_location == next_location &&
					visited_turn.guard_location == self.location }) {
				println!("Infinite loop found! At {:?}", next_location);
				self.is_in_inf_loop = true;
			} else {
				// Log the turn for next time.
				self.previously_visited_turns.push(VisitedTurn {
					turn_location: next_location,
					guard_location: self.location
				});
			}

			// Turning 90 degrees and re-retrieving the next location.
				self.turn_count += 1;
			next_location = self.get_next_location();
		}

		self.location = next_location;

		// Update map.
		map[next_location.0 as usize][next_location.1 as usize] = 'X';
		Some(())
	}

	fn get_next_location(&mut self) -> (i32, i32) {
		let directions = ["top", "right", "bottom", "left"];
		let direction = directions[self.turn_count % 4];

		if direction == "top" {
			(self.location.0 - 1, self.location.1)
		} else if direction == "right" {
			(self.location.0, self.location.1 + 1)
		} else if direction == "bottom" {
			(self.location.0 + 1, self.location.1)
		} else if direction == "left" {
			(self.location.0, self.location.1 - 1)
		} else {
			self.location
		}
	}
}


fn parse(content: String) -> Vec<Vec<char>> {
	let mut lines: Vec<&str> = content.split("\n").collect();
	lines.remove(lines.len() - 1);

	lines.into_iter()
		.map(|s| s.chars().collect())
		.collect()
}

