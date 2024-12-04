use regex::Regex;

pub fn run(contents: String) -> i32 {
	part1(contents)
	//part2(contents)
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

		if _do {
			total += group
				.replace("mul(", "")
				.replace(")", "")
				.split(",")
				.map(|v| v.parse::<i32>().unwrap())
				.take(2)
				.product::<i32>();
		}

	}
	total
}

fn part1(contents: String) -> i32 {
	let pattern = r"(mul\(\d+,\d+\))";
	let regex: Regex = Regex::new(pattern).unwrap();

	let total: i32 = regex.captures_iter(&contents)
		.map(|capture| {
		capture[0]
			.replace("mul(", "")
			.replace(")", "")
			.split(",")
			.map(|v| v.parse::<i32>().unwrap())
			.take(2)
			.product::<i32>()
	})
	.sum();

	total
}
