use utils::{CharTable};

pub fn run(content: String) -> i32 {
	let regions = parse(content);

	let mut total_price_part_1 = 0;
	let mut total_price_part_2 = 0;

	for region in &regions {
		println!("\nRegion: {}", region.crops);
		// println!("Plots: {:?}", region.plots);
		// println!("Perimeter: {:?}", region.get_perimeter());
		// println!("Area: {:?}", region.get_area());
		// println!("Sides: {:?}", region.get_sides());
		println!("Price: {}", region.get_area() * region.get_sides());
		println!("___\n");

		total_price_part_1 += region.get_area() * region.get_perimeter();
		total_price_part_2 += region.get_area() * region.get_sides();
	}

	println!("Part 1: {}", total_price_part_1);
	println!("Part 2* not done yet*: {}", total_price_part_2);

	total_price_part_1
}

#[derive(Debug)]
struct Region {
	field: CharTable,
	crops: char,
	plots: Vec<(i32, i32)>,
}

impl Region {
	pub fn new(field: CharTable, crops: char) -> Region {
		Region { field, crops, plots: vec![] }
	}

	pub fn get_area(&self) -> i32 {
		self.plots.len() as i32
	}

	pub fn get_sides(&self) -> i32 {
		let mut corners = 0;
		let mut inner_corners: Vec<(i32, i32)> = vec![];
		let mut padding: Vec<(i32, i32)> = vec![];

		for plot in &self.plots {

			// let mut plot_exposed_sides = 0;
			let mut plot_exposed_sides: Vec<char> = vec![];

			for _pos in vec![
				&(plot.0 - 1, plot.1, 'u'),
				&(plot.0 + 1, plot.1, 'd'),
				&(plot.0, plot.1 - 1, 'l'),
				&(plot.0, plot.1 + 1, 'r')] {

				if let Some(cell) = self.field.get_cell(_pos.0, _pos.1) {

					if !self.plots.contains(&(_pos.0, _pos.1)) {
						plot_exposed_sides.push(_pos.2);

						// Check if the exposed cell is bordering more of this region.
						let mut bordering_cell_exposed_sides: Vec<char> = vec![];

						if let Some(top) = self.field.get_cell(_pos.0 - 1, _pos.1) {
							if top == self.crops {
								bordering_cell_exposed_sides.push('t');
							}
						}
						if let Some(bot) = self.field.get_cell(_pos.0 + 1, _pos.1) {
							if bot == self.crops {
								bordering_cell_exposed_sides.push('b');
							}
						}
						if let Some(lef) = self.field.get_cell(_pos.0, _pos.1 - 1) {
							if lef == self.crops {
								bordering_cell_exposed_sides.push('l');
							}
						}
						if let Some(rig) = self.field.get_cell(_pos.0, _pos.1 + 1) {
							if rig == self.crops {
								bordering_cell_exposed_sides.push('r');
							}
						}

						let new_ic = if bordering_cell_exposed_sides.len() <= 1 {
							0
						} else if bordering_cell_exposed_sides.len() == 2 {
							if bordering_cell_exposed_sides.contains(&'l') && bordering_cell_exposed_sides.contains(&'r') ||
								bordering_cell_exposed_sides.contains(&'u') && bordering_cell_exposed_sides.contains(&'d') {
								0
							} else {
								1
							}
						} else if bordering_cell_exposed_sides.len() == 3 {
							if !padding.contains(&(_pos.0, _pos.1)) {
								padding.push((_pos.0, _pos.1));
							}
							// println!("({:?})bordering has 2. buffer time -boimler: {:?}", (_pos), bordering_cell_exposed_sides);
							2
						} else {
							println!("??? there are {} sides: {:?} at pos {:?}", bordering_cell_exposed_sides.len(), bordering_cell_exposed_sides, _pos);
							4
						};

						// Determine the cell where the inner corner is.
						if new_ic >= 1 {
							// println!("INNER FOUND: {:?}", (_pos.0, _pos.1));
							if !inner_corners.contains(&((_pos.0, _pos.1))) {
								inner_corners.push((_pos.0, _pos.1));
							}
						}
					}
				} else {
					// The cell isn't found. AKA this cell is at a border. That counts tho.
					plot_exposed_sides.push(_pos.2);
				}
			}

			let mut corner_count = 0;
			if plot_exposed_sides.contains(&'u') && plot_exposed_sides.contains(&'l') {
				corner_count += 1;
			}
			if plot_exposed_sides.contains(&'u') && plot_exposed_sides.contains(&'r') {
				corner_count += 1;
			}
			if plot_exposed_sides.contains(&'d') && plot_exposed_sides.contains(&'l') {
				corner_count += 1;
			}
			if plot_exposed_sides.contains(&'d') && plot_exposed_sides.contains(&'r') {
				corner_count += 1;
			}

			corners += corner_count;
			// println!("Plot exposed sides: {:?}, ic: {:?}, p: {:?}", &(plot, plot_exposed_sides, corner_count), inner_corners, padding.len());

		}

		// println!("inner corners: {:?}", inner_corners);
		(corners + inner_corners.len() + padding.len()) as i32
	}

	pub fn get_perimeter(&self) -> i32 {
		let mut perimeter = 0;
		for plot in &self.plots {

			for _pos in vec![
				plot,
				&(plot.0 - 1, plot.1),
				&(plot.0 + 1, plot.1),
				&(plot.0, plot.1 - 1),
				&(plot.0, plot.1 + 1)] {

				if let Some(cell) = self.field.get_cell(_pos.0, _pos.1) {

					if cell != self.crops {
						perimeter += 1;
					}
				} else {
					// The cell isn't found. AKA this cell is at a border. That counts tho.
					perimeter += 1;
				}
			}
		}

		perimeter
	}

	pub fn find_plots(initial_pos: (i32, i32), field: CharTable) -> Option<Region> {
		if let Some(cell) = field.get_cell(initial_pos.0, initial_pos.1) {
			let mut region = Region::new(field, cell);
			region.find_adjacent_plots(initial_pos);
			return Some(region);
		}

		None
	}

	fn find_adjacent_plots(&mut self, pos: (i32, i32)) {
		// Check if each adjacent plot has the same crops.
		for _pos in vec![
			pos,
			(pos.0 - 1, pos.1),
			(pos.0 + 1, pos.1),
			(pos.0, pos.1 - 1),
			(pos.0, pos.1 + 1)] {

			if let Some(cell) = self.field.get_cell(_pos.0, _pos.1) {

				// Don't want to get in an infinite loop, so only check the plot if
				// it's not yet in the region
				if !self.plots.contains(&_pos) && cell == self.crops {
					self.plots.push(_pos);

					// Recursively also check the this plot.
					self.find_adjacent_plots(_pos);
				}
			}
		}

	}
}


fn parse(content: String) -> Vec<Region> {
	let ct = CharTable::from(content);
	ct.display();

	let mut regions = vec![];
	let mut used_cells: Vec<(i32, i32)> = vec![];

	for i in 0..ct.grid.len() {
		for j in 0..ct.grid[i].len() {

			if used_cells.contains(&(i as i32, j as i32)) {
				continue;
			}

			if let Some(region) = Region::find_plots(
				(i as i32, j as i32),
				ct.clone()
			) {
				used_cells.extend(&region.plots);
				regions.push(region);
			}
		}
	}

	regions
}
