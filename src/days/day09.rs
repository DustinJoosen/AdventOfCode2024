
pub fn run(content: String) -> i32 {
	// Get the id's
	let mut id_array = parse_to_id_array(content);

	let part_1 = false;
	if part_1 {
		compact(&mut id_array);
	} else {
		compact_part_2(&mut id_array);
	}

	// Get the checksum.
	let checksum = calculate_checksum(&mut id_array);

	// Number is to big to fit in i32 again.
	println!("Result: {checksum}");
	-1
}

fn calculate_checksum(id_array: &mut Vec<String>) -> usize {
	// Calculate the checksum.
	let mut count = 0;
	for i in 0..id_array.len() {
		if id_array[i] == "." {
			continue;
		}

		count += i * &id_array[i].parse().unwrap();
	}
	count
}

fn compact_part_2(mut id_array: &mut Vec<String>) {
	let initial_max = get_max_id(id_array);
	let mut i = initial_max;

	'main: loop {
		compact_once_part_2(&mut id_array, i);

		i -= 1;
		if i <= 0 {
			break 'main;
		}

		println!("ID: {i} calculate");
	}

}


fn compact(mut id_array: &mut Vec<String>) {

	'main: loop {
		let could_compact = compact_once(&mut id_array);
		if !could_compact {
			break 'main;
		}
	}
}

fn compact_once_part_2(id_array: &mut Vec<String>, id: i32) -> bool {
	// Get all indexes where this id appears.
	let mut id_indexes: Vec<usize> = vec![];
	for (i, ida ) in id_array.iter().enumerate() {
		if ida == &id.to_string() {
			id_indexes.push(i);
		}
	}

	let (dot_seq, start_dot_idx) = match get_consecutive_dots_index(id_array, id_indexes.len()) {
		Ok(value) => value,
		Err(value) => return false,
	};

	// Don't wanna move to the right.
	if start_dot_idx > *id_indexes.iter().min().unwrap() {
		return false
	}

	// Replace 1: Turn all of the id into a dot.
	for id_idx in &id_indexes {
		if id_idx > &id_array.len() {
			println!("woops");
		}
		id_array[*id_idx] = String::from(".");
	}

	// Replace 2: Turn n dots at start_dot_idx into id
	for i in start_dot_idx..(start_dot_idx + dot_seq.len()) {
		id_array[i] = id.to_string();
	}

	true
}

fn get_consecutive_dots_index(id_array: &mut Vec<String>, len: usize) -> Result<(String, usize), bool> {
	// This part checks where the dots fit.
	let dot_seq = ".".repeat(len);

	let mut consecutive_dots = 0;
	let mut op_start_dot_idx = None;

	for (idx, elem) in id_array.iter().enumerate() {
		if &elem != &"." {
			consecutive_dots = 0;
			continue;
		}

		consecutive_dots += 1;

		if consecutive_dots == dot_seq.len() {
			op_start_dot_idx = Some(idx - dot_seq.len() + 1);
			break;
		}
	}

	if op_start_dot_idx.is_none() { return Err(false); }
	let start_dot_idx = op_start_dot_idx.unwrap();

	Ok((dot_seq, start_dot_idx))
}

fn compact_once(id_array: &mut Vec<String>) -> bool {
	let first_dot_idx = id_array.iter().position(|i| i == ".").unwrap();
	let last_id_idx = id_array.len() - 1 - id_array
		.iter()
		.rev()
		.position(|i| i != ".")
		.unwrap();

	// If the last_id is further left then the first dot, it's done :)
	if last_id_idx < first_dot_idx {
		return false
	}

	// Swap the last id, and the first dot
	id_array.swap(first_dot_idx, last_id_idx);
	true
}

fn get_max_id(id_array: &mut Vec<String>) -> i32 {
	let max_id = id_array.iter()
		.filter_map(|s| s.parse::<i32>().ok())
		.max();

	max_id.unwrap()
}

fn parse_to_id_array(diskmap: String) -> Vec<String> {
	let mut id_array: Vec<String> = vec![];
	let mut id = 0;

	let mut flag_free_space = false;
	for file in diskmap.chars() {
		let file_base_10 = file.to_digit(10).unwrap();

		if flag_free_space {
			for _ in 0..file_base_10 { id_array.push(".".to_string()); }
		}
		else {
			for _ in 0..file_base_10 { id_array.push(id.to_string()); }
			id += 1;
		}

		flag_free_space = !flag_free_space;
	}
	id_array
}