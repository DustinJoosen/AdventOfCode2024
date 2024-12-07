
pub fn run(content: String) -> i32 {
	let part_1 = false;

	let mut total = 0;
	let calculations = parse(content);
	for calculation in calculations {
		let calculation_works = try_calculation(&calculation, !part_1);
		if calculation_works {
			total += calculation.0;
			println!("Calculation works!: {:?}", &calculation);
		}
	}

	// Result is too big for i32.
	println!("Result: {total}");
	-1
}

fn try_calculation(calculation: &(i64, Vec<i32>), part2: bool) -> bool {
	let (result, nums) = calculation;

	let op_count = nums.len() - 1;

	// Get all ops.
	let mut ops_results = Vec::new();
	generate_ops(op_count, Vec::new(), &mut ops_results, part2);

	// Go through all the operations.
	for combination in ops_results {
		// Get the tokens.
		let mut tokens: Vec<String> = vec![];
		for i in 0..nums.len() {
			tokens.push(nums[i].to_string());
			if i != nums.len() - 1 {
				tokens.push(combination[i].to_string());
			}
		}

		// Check if it was correct.
		let sum_correct = if part2 {
			check_sum_part_2(tokens, *result)
		} else {
			check_sum(tokens, *result)
		};

		if sum_correct {
			return true
		}
	}

	false
}


fn generate_ops(op_count: usize, current_combination: Vec<String>, result: &mut Vec<Vec<String>>, include_append: bool) {
	if current_combination.len() == op_count {
		result.push(current_combination);
		return;
	}

	// Go the '+' route and recurse.
	let mut new_combo = current_combination.clone();
	new_combo.push("+".to_string());
	generate_ops(op_count, new_combo, result, include_append);

	// Go the '*' route and recurse.
	let mut new_combo = current_combination.clone();
	new_combo.push("x".to_string());
	generate_ops(op_count, new_combo, result, include_append);

	if include_append {
		// Go the '|' route and recurse.
		let mut new_combo = current_combination.clone();
		new_combo.push("|".to_string());
		generate_ops(op_count, new_combo, result, include_append);
	}
}

fn check_sum_part_2(tokens: Vec<String>, expected_result: i64) -> bool {
	let mut buffer_result: i64 = 0;
	let mut buffer_operator = String::from("+");

	for token in tokens {
		match token.as_str() {
			"+" | "x" | "|" => buffer_operator = token,
			_ => {
				buffer_result = match buffer_operator.as_str() {
					"+" => buffer_result + token.parse::<i64>().unwrap(),
					"x" => buffer_result * token.parse::<i64>().unwrap(),
					"|" => format!("{}{}", buffer_result, token).parse::<i64>().unwrap(),
					_ => panic!("Operator {token} not supported")
				};
			}
		}
	}

	buffer_result == expected_result
}


fn check_sum(tokens: Vec<String>, expected_result: i64) -> bool {
	let mut buffer_result: i64 = 0;
	let mut buffer_operator = String::from("+");

	for token in tokens {
		match token.as_str() {
			"+" | "x" => buffer_operator = token,
			_ => {
				buffer_result = match buffer_operator.as_str() {
					"+" => buffer_result + token.parse::<i64>().unwrap(),
					"x" => buffer_result * token.parse::<i64>().unwrap(),
					_ => panic!("Operator {token} not supported")
				};
			}
		}
	}

	buffer_result == expected_result
}

fn parse(content: String) -> Vec<(i64, Vec<i32>)> {
	content.lines()
		.map(|line| {
			let (pre, post) = line.split_once(": ").unwrap();

			let res: i64 = pre.parse().unwrap();
			let nums: Vec<i32> = post.split_whitespace()
				.map(|num| num.parse().unwrap())
				.collect();

			(res, nums)
		})
		.collect()
}