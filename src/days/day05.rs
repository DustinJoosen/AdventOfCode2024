
pub fn run(content: String) -> i32 {
	// part1(content)
	part2(content)
}

fn part2(content: String) -> i32 {
	let (order_rules, updates) = parse(content);
	let mut count = updates.iter().map(|update_line| {

		let valid_update_line = get_valid_update(&order_rules, update_line);
		if &valid_update_line == update_line {
			return 0;
		}

		valid_update_line[valid_update_line.len() / 2]
	}).sum();

	count
}


fn part1(content: String) -> i32 {
	let (order_rules, updates) = parse(content);
	let mut count = updates.iter().map(|update_line| {

		let valid_update_line = get_valid_update(&order_rules, update_line);

		if &valid_update_line != update_line {
			return 0;
		}
		valid_update_line[valid_update_line.len() / 2]
	}).sum();

	count
}

fn get_valid_update(order_rules: &Vec<(i32, i32)>, update_line: &Vec<i32>) -> Vec<i32> {
	// Load the ordering into the rules.
	let mut rules: Vec<ParsingRule> = Vec::new();
	for update in update_line {
		let mut new_rule = crate::days::day05::ParsingRule::new(*update);
		new_rule.load_rules(&order_rules, update_line);
		rules.push(new_rule);
	}

	// Sort the rules according to the ordering rules.
	let mut buffer: Vec<i32> = Vec::new();
	while !rules.is_empty() {
		if let Some(idx) = rules.iter().position(|rule| rule.goes_after.is_empty()) {
			let rule_to_remove = rules[idx].number;

			// Remove rule value from all other rules.
			for rule in &mut rules {
				rule.goes_after.retain(|&x| x != rule_to_remove);
			}

			// Remove rule from rules, and add it to buffer.
			let earliest_rule = rules.remove(idx);
			buffer.push(earliest_rule.number);
		}
	}

	buffer
}

#[derive(Debug)]
pub struct ParsingRule {
	number: i32,
	goes_after: Vec<i32>
}

impl ParsingRule {
	pub fn new(number: i32) -> ParsingRule {
		ParsingRule{ number, goes_after: Vec::new() }
	}

	pub fn load_rules(&mut self, rules: &Vec<(i32, i32)>, relevant_numbers: &Vec<i32>) {
		for rule in rules {
			if rule.1 == self.number && relevant_numbers.contains(&rule.0) {
				self.goes_after.push(rule.0);
			}
		}

	}
}

fn parse(mut contents: String) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
	contents = contents.replace("\r", "");

	let ordering_rules: Vec<(i32, i32)> = contents.split_once("\n\n").unwrap().0
		.lines()
		.map(|line| {
			let parts = line.split_once("|").unwrap();
			(parts.0.parse().unwrap(), parts.1.parse().unwrap())
		})
		.collect();

	let updates: Vec<Vec<i32>> = contents.split_once("\n\n").unwrap().1
		.lines()
		.map(|line| {
			line.split(",")
				.map(|x| x.parse().unwrap())
				.collect()
		})
		.collect();

	(ordering_rules, updates)
}