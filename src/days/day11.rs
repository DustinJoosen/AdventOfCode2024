
pub fn run(content: String) -> i32 {
	let mut stones = parse(content);

	// part1(&mut stones)
	part2(&mut stones)
}

fn part2(stones: &mut Vec<i64>) -> i32 {
	let blink_count = 75;

	let mut counted_stones: Vec<(i64, i64)> = stones.iter()
		.map(|&stone| (stone, 1))
		.collect();

	for _ in 0..blink_count {
		blink_part_2(&mut counted_stones);
		sort_and_remove(&mut counted_stones);
	}

	let count: i64 = counted_stones.iter().map(|&(_, count)| count).sum();

	// Number is too big to fit in i32 again.
	println!("Result: {:?}", count);
	-1
}

fn part1(stones: &mut Vec<i64>) -> i32 {
	let blink_count = 25;

	for _ in 0..blink_count {
		blink(stones);
	}

	stones.len() as i32
}

fn blink_part_2(mut stones: &mut Vec<(i64, i64)>) {
	let mut new_stones = vec![];

	// Step 1: Do the blink actions.
	for i in 0..stones.len() {
		let mut stone = &mut stones[i];

		// Rule 1: 0->1
		if stone.0 == 0 {
			new_stones.push((1, stone.1));
			continue;
		}

		// Rule 2: Split even-digit-numbers
		if stone.0.to_string().len() % 2 == 0 {

			let stone_chars: Vec<char> = stone.0.to_string().chars().collect();
			let midpoint = stone_chars.len() / 2;

			let fi_half: String = stone_chars[..midpoint].iter().collect();
			let se_half: String = stone_chars[midpoint..].iter().collect();

			new_stones.push((fi_half.parse().unwrap(), stone.1));
			new_stones.push((se_half.parse().unwrap(), stone.1));

			continue;
		}

		// Rule 3: If all else fails, multiply by 2024.
		new_stones.push((stone.0 * 2024, stone.1));
	};

	*stones = new_stones
}

// Step 2: Sort the stones, and merge duplicates.
fn sort_and_remove(mut stones: &mut Vec<(i64, i64)>) {
	let mut new_stones: Vec<(i64, i64)> = vec![];

	for i in 0..stones.len() {

		// Stone is not in new_stones. Just add and continue.
		if !new_stones.iter().any(|&(x, _)| x == stones[i].0) {
			new_stones.push(stones[i]);
			continue;
		}

		// This stone is already in new_stones. Add the value.
		if let Some(pos) = new_stones.iter().position(|&(x, _)| x == stones[i].0) {
			new_stones[pos].1 += stones[i].1
		}
		else {
			println!("Stone is already in newstones, but not found by position(). Big logic issue");
		}
	}

	new_stones.sort_by_key(|stone| stone.0);
	*stones = new_stones
}

fn blink(stones: &mut Vec<i64>) {
	let mut new_stones: Vec<i64> = vec![];

	stones.iter_mut().for_each(|stone| {

		// Rule 1: 0->1
		if *stone == 0 {
			new_stones.push(1);
			return;
		}

		// Rule 2: Split even-digit-numbers
		if stone.to_string().len() % 2 == 0 {

			let stone_chars: Vec<char> = stone.to_string().chars().collect();
			let midpoint = stone_chars.len() / 2;

			let fi_half: String = stone_chars[..midpoint].iter().collect();
			let se_half: String = stone_chars[midpoint..].iter().collect();

			new_stones.push(fi_half.parse().unwrap());
			new_stones.push(se_half.parse().unwrap());

			return;
		}

		// Rule 3: If all else fails, multiply by 2024.
		new_stones.push(*stone * 2024);
	});

	*stones = new_stones;
}


fn parse(content: String) -> Vec<i64> {
	content.split_whitespace()
		.map(|c| c.parse().unwrap())
		.collect::<Vec<i64>>()
}
