
pub fn run(content: String) -> i32 {
	let map = parse(content);

	let mut total_trailhead_score_part_1 = 0;
	let mut total_trailhead_score_part_2 = 0;


	// Loop through all the chars.
	for i in 0..map.len() {
		for j in 0..map[i].len() {

			// Only need to do something when you hit a trailhead, so early exit if it's not the case.
			if map[i][j] != 0 {
				continue;
			}

			// A mutable array of all the locations where the trail leads (for part 1).
			let mut nines = vec![];

			let trailhead_score = check_if_bordering_next_path(&map, (i, j), &mut nines);
			println!("Trailhead ({i}, {j}) count: {trailhead_score}");

			total_trailhead_score_part_1 += trailhead_score;
			total_trailhead_score_part_2 += nines.len();
		}
	}

	println!("Part 1: {total_trailhead_score_part_1}");
	println!("Part 2: {total_trailhead_score_part_2}");

	total_trailhead_score_part_1
}


// Recursively checks if the next tile is next on the path.
fn check_if_bordering_next_path(
	map: &Vec<Vec<i32>>,
	position: (usize, usize),
	nines: &mut Vec<(usize, usize)>) -> i32 {

	let mut total = 0;

	// Current height.
	let current_height = map[position.0][position.1];

	// Exit the recursion.
	if current_height == 9 {
		if !nines.contains(&position) {
			nines.push(position);
		}
		return 1;
	}

	// Right.
	if position.1 + 1 < map[0].len() && map[position.0][position.1 + 1] == (current_height + 1) {
		total += check_if_bordering_next_path(&map, (position.0, position.1 + 1), nines);
	}

	// Down.
	if position.0 + 1 < map.len() && map[position.0 + 1][position.1] == (current_height + 1) {
		total += check_if_bordering_next_path(&map, (position.0 + 1, position.1), nines);
	}

	// Left.
	if position.1 > 0 && map[position.0][position.1 - 1] == (current_height + 1) {
		total += check_if_bordering_next_path(&map, (position.0, position.1 - 1), nines);
	}

	// Up.
	if position.0 > 0 && map[position.0 - 1][position.1] == (current_height + 1) {
		total += check_if_bordering_next_path(&map, (position.0 - 1, position.1), nines);
	}

	total
}

// TODO: Make a Parsing Utility Struct, cause this is something that can easily be automated.
fn parse(content: String) -> Vec<Vec<i32>> {
	let mut result: Vec<Vec<i32>> = Vec::new();

	for line in content.lines() {
		let parsed: Vec<i32> = line
			.chars()
			.map(|p| p.to_digit(10).unwrap() as i32)
			.collect();

		result.push(parsed);
	}

	result

}


