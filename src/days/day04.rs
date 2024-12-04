use std::hash::Hash;

pub fn run(contents: String) -> i32 {
	// part1(contents)
	part2(contents)
}

fn part2(contents: String) -> i32 {
	let lines = parse(&contents);
	let mut count = 0;

	for (i, line) in lines.iter().enumerate() {
		for (j, c) in line.chars().enumerate() {
			if c == 'A' {
				if is_xmas(&lines, (i, j)) { count += 1 }
			}
		}
	}

	count
}
fn is_xmas(lines: &Vec<&str>, a_position: (usize, usize)) -> bool {
	let x = a_position.1;
	let y = a_position.0;

	// Check vertical out of bounds.
	if y <= 0 || y >= lines.len() - 1 {
		return false;
	}

	// Check horizontal out of bounds
	if x <= 0 || x >= lines[0].len() - 1 {
		return false;
	}

	// Extract the values to use.
	let top_left = lines[y - 1].chars().nth(x - 1).unwrap();
	let top_right = lines[y - 1].chars().nth(x + 1).unwrap();
	let bot_left = lines[y + 1].chars().nth(x - 1).unwrap();
	let bot_right = lines[y + 1].chars().nth(x + 1).unwrap();

	// Check if 2 m's and 2 s's are there.
	let mut m_count = 0;
	let mut s_count = 0;

	if top_left == 'M' { m_count += 1;} else if top_left == 'S' { s_count += 1; }
	if top_right == 'M' { m_count += 1;	} else if top_right == 'S' { s_count += 1; }
	if bot_left == 'M' { m_count += 1;} else if bot_left == 'S' { s_count += 1; }
	if bot_right == 'M' { m_count += 1;	} else if bot_right == 'S' { s_count += 1; }

	// Can't have them in corners.
	if top_left == bot_right || top_right == bot_left {
		return false
	}

	// Need at least 2 of the correct ones.
	if m_count == 2 && s_count == 2 {
		return true
	}

	false
}

fn part1(contents: String) -> i32 {
	let lines = parse(&contents);
	let word = ['X', 'M', 'A', 'S'];

	let mut count = 0;

	let directions: [(isize, isize); 8] = [
		(1, 0), // Right.
		(-1, 0), // Left.
		(0, 1), // Down.
		(0, -1), // Top.
		(1, -1), // Top right.
		(-1, -1), // Top left.
		(1, 1), // Down right.
		(-1, 1), // Down left.
	];

	for (dx, dy) in directions {
		for (i, line) in lines.iter().enumerate() {
			for (j, c) in line.chars().enumerate() {
				if c != word[0] {
					continue;
				}

				let mut t_i = i;
				let mut t_j = j;

				for word_char in ['M', 'A', 'S'] {
					t_i = (t_i as isize + dy) as usize;
					t_j = (t_j as isize + dx) as usize;
					if t_i < 0 || t_j < 0 || t_i > line.len() - 1 || t_j > lines.iter().len() - 1 {
						continue;
					}

					let next_cell = lines[t_i].chars().nth(t_j).unwrap();
					if next_cell == word_char {
						if word_char == 'S' {
							count += 1;
						}
					} else {
						break;
					}
				}
			}
		}
	}

	count
}

fn parse(contents: &str) -> Vec<&str> {
	let parsed: Vec<&str> = contents.split("\r\n").collect();
	parsed[0..parsed.len() - 1].to_vec()
}