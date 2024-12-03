use regex::Regex;

pub fn run(contents: String) -> i32 {
	//part1(contents)
	part2(contents)
}


fn part2(contents: String) -> i32 {
	let pattern = r"(mul\(\d+,\d+\)|do\(\)|don't\(\))";
	let regex: Regex = Regex::new(pattern).unwrap();

	let mut _do = true;
	let mut total = 0;

	for capture in regex.captures_iter(&contents) {
		let group = &capture[0];
		if group == "do()" {
			_do = true;
			continue;
		} else if  group == "don't()" {
			_do = false;
			continue;
		}

		if !_do {
			continue;
		}

		let num_arr: Vec<i32> = group
			.replace("mul(", "")
			.replace(")", "")
			.split(",")
			.map(|v| v.parse().unwrap())
			.collect();

		total += num_arr[0] * num_arr[1];
	}
	total
}

fn part1(contents: String) -> i32 {
	let pattern = r"(mul\(\d+,\d+\))";
	let regex: Regex = Regex::new(pattern).unwrap();

	let mut total = 0;
	for capture in regex.captures_iter(&contents) {
		let group = &capture[0];
		let num_arr: Vec<i32> = group
			.replace("mul(", "")
			.replace(")", "")
			.split(",")
			.map(|v| v.parse().unwrap())
			.collect();

		total += num_arr[0] * num_arr[1];
	}
	total
}
