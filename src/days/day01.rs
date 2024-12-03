
pub fn run(contents: String) -> i32 {
	let (arr1, arr2) = match parse(contents) {
		Some(v) => v,
		None => {
			return -1
		}
	};

	// part1(arr1, arr2);
	part2(arr1, arr2)
}

fn part2(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
	let mut sum = 0;
	for location_id in arr1 {
		let count = arr2.iter().filter(|&&x| x == location_id).count() as i32;
		sum += location_id * count;
	}

	sum
}

fn part1(mut arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
	// Sort the arrays.
	arr1.sort();
	arr2.sort();

	// Calculate everything.
	let mut sum = 0;
	for i in 0..arr1.len() {
		let diff = (arr1[i] - arr2[i]).abs();
		sum += diff;
	}

	sum
}

fn parse(contents: String) -> Option<(Vec<i32>, Vec<i32>)> {
	let mut arr1: Vec<i32> = Vec::new();
	let mut arr2: Vec<i32> = Vec::new();

	for line in contents.lines() {
		let line_numbers: Vec<&str> = line.split_whitespace().collect();

		if line_numbers.len() != 2 {
			println!("Parsing Error: There were more then 2 numbers found on line {line}");
			return None;
		}

		arr1.push(line_numbers[0].parse().unwrap_or_else(|_| 0));
		arr2.push(line_numbers[1].parse().unwrap_or_else(|_| 0));
	}

	Some((arr1, arr2))

}
