use regex::Regex;

const AREA_WIDTH: usize = 101;
const AREA_HEIGHT: usize = 103;
const SECONDS: i32 = 100;

pub fn run(content: String) -> i32 {
	// part1(&content)
	part2(&content)
}


fn part2(content: &str) -> i32 {
	// Parse robots.
	let mut robots = parse(content);
	let mut second = 0;

	loop {
		// Create grid.
		let mut grid: Vec<Vec<i32>> = vec![vec![0; AREA_WIDTH]; AREA_HEIGHT];

		for mut robot in &mut robots {
			robot.pos_x = (((robot.pos_x + robot.vel_x) % AREA_WIDTH as i32) + AREA_WIDTH as i32) % AREA_WIDTH as i32;
			robot.pos_y = (((robot.pos_y + robot.vel_y) % AREA_HEIGHT as i32) + AREA_HEIGHT as i32) % AREA_HEIGHT as i32;

			grid[robot.pos_y as usize][robot.pos_x as usize] = 1;
		};

		// Check for every second
		println!("Seconds: {second}");
		for line in &grid {

			if line.iter()
				.map(|&num| num.to_string())
				.collect::<String>()
				.contains("11111111111111111") {
				println!("Hit it!!! {second}");
				display_grid(&grid, false);
				return second + 1;
			}
		}

		second += 1;
	}
}


fn part1(content: &str) -> i32 {
	// Parse robots.
	let mut robots = parse(content);

	// Create grid.
	let mut grid: Vec<Vec<i32>> = vec![vec![0; AREA_WIDTH]; AREA_HEIGHT];

	// Loop through the robots, and calculate the new position for each of them.
	for mut robot in robots {
		robot.pos_x = (((robot.pos_x + robot.vel_x * SECONDS) % AREA_WIDTH as i32) + AREA_WIDTH as i32) % AREA_WIDTH as i32;
		robot.pos_y = (((robot.pos_y + robot.vel_y * SECONDS) % AREA_HEIGHT as i32) + AREA_HEIGHT as i32) % AREA_HEIGHT as i32;

		grid[robot.pos_y as usize][robot.pos_x as usize] += 1;
	}

	display_grid(&grid, true);
	calculate_quad_values(&grid)
}

fn calculate_quad_values(grid: &Vec<Vec<i32>>) -> i32 {
	// Calculate quadrants.
	let middle_row = (AREA_HEIGHT - 1) / 2;
	let middle_col = (AREA_WIDTH - 1) / 2;

	let top_half: Vec<Vec<i32>> = grid.iter().take(middle_row).map(|row| row.to_vec()).collect();
	let bot_half: Vec<Vec<i32>> = grid.iter().skip(middle_row + 1).map(|row| row.to_vec()).collect();

	let top_left: i32 = top_half.iter()
		.map(|row| row.iter().take(middle_col).sum::<i32>()).sum();

	let top_right: i32 = top_half.iter()
		.map(|row| row.iter().skip(middle_col + 1).sum::<i32>()).sum();

	let bot_left: i32 = bot_half.iter()
		.map(|row| row.iter().take(middle_col).sum::<i32>()).sum();

	let bot_right: i32 = bot_half.iter()
		.map(|row| row.iter().skip(middle_col + 1).sum::<i32>()).sum();

	top_left * top_right * bot_left * bot_right
}

fn display_grid(grid: &Vec<Vec<i32>>, part1: bool ) {
	for line in grid {
		for dig in line {
			if *dig == 0 {
				print!(".");
			} else {
				if part1 {
					print!("{}", dig);
				} else {
					print!("#");
				}
			}
		}
		println!();
	}
}

#[derive(Debug)]
struct Robot {
	pos_x: i32,
	pos_y: i32,
	vel_x: i32,
	vel_y: i32
}

fn parse(content: &str) -> Vec<Robot> {
	let pattern = r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)";
	let regex: Regex = Regex::new(pattern).unwrap();

	let mut robots: Vec<Robot> = vec![];

	for line in content.lines() {

		if let Some(captures) = regex.captures(line) {

			robots.push(Robot {
				pos_x: captures.get(1).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap(),
				pos_y: captures.get(2).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap(),
				vel_x: captures.get(3).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap(),
				vel_y: captures.get(4).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap(),
			});
		}
	}

	robots
}
