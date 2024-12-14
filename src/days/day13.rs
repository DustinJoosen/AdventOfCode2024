use regex::Regex;

pub fn run(content: String) -> i32 {
	let games = parse(content);
	let mut tickets_total = 0;

	for game in games {
		let mut combos: Vec<(i32, i32)> = vec![];

		for a_presses in 0..100 {
			for b_presses in 0..100 {
				let x = (game[0].0 * a_presses) + (game[1].0 * b_presses);
				let y = (game[0].1 * a_presses) + (game[1].1 * b_presses);

				if game[2].0 == x && game[2].1 == y {
					combos.push((a_presses, b_presses));
				}
			}
		}

		if !combos.is_empty() {
			tickets_total += combos[0].0 * 3 + combos[0].1
		}
	}

	tickets_total
}

fn parse(content: String) -> Vec<Vec<(i32, i32)>> {
	let pattern = r"\d+";
	let regex: Regex = Regex::new(pattern).unwrap();

	let result: Vec<Vec<(i32, i32)>> = content.lines().collect::<Vec<&str>>()
		.chunks(4)
		.map(|chunk| {
			let lines: Vec<&str> = chunk.iter().take(3).cloned().collect();
			let mut st: Vec<(i32, i32)> = vec![];

			for line in lines {

				let numbers: Vec<i32> = regex.find_iter(line)
					.map(|mat| mat.as_str().parse::<i32>().unwrap())
					.collect();

				st.push((numbers[0], numbers[1]));
			}
			st
		}).collect();
	result
}