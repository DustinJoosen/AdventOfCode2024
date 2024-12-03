
pub fn run(contents: String) -> i32 {
	let reports = parse(contents);
	//part1(reports)
	 part2(reports)
}


fn part2(reports: Vec<Vec<i32>>) -> i32 {
	let mut safe_count = 0;

	'reportLoop: for report in reports {
		let mut flag = false;

		// Determine if this report increases or decreases.
		let increases = report[0] < report[1];

		for i in 0..report.len() - 1 {
			let curr = report[i];
			let next = report[i + 1];

			let diff = (curr - next).abs();

			// Diff is too big. Skip this report.
			if diff < 1 || diff > 3 || (increases && curr > next) || (!increases && next > curr) {
				if !flag {
					flag = true;
				} else {
					continue 'reportLoop;
				}
			}
		}

		safe_count += 1
	}
	safe_count
}

fn part1(reports: Vec<Vec<i32>>) -> i32 {
	let mut safe_count = 0;

	'reportLoop: for report in reports {
		// Determine if this report increases or decreases.
		let increases = report[0] < report[1];

		for i in 0..report.len() - 1 {
			let curr = report[i];
			let next = report[i + 1];

			let diff = (curr - next).abs();

			if diff < 1 || diff > 3 || (increases && curr > next) || (!increases && next > curr) {
				continue 'reportLoop;
			}
		}
		safe_count += 1
	}
	safe_count
}

fn parse(contents: String) -> Vec<Vec<i32>> {
	let mut result: Vec<Vec<i32>> = Vec::new();

	for line in contents.lines() {
		let parsed: Vec<i32> = line
			.split_whitespace()
			.map(|p| p.parse::<i32>().unwrap())
			.collect();

		result.push(parsed);
	}

	result
}