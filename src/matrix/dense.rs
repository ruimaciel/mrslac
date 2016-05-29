
struct Dense {
	rows: usize,
	columns: usize,
	element: Vec<f32>
}


impl Dense {

	/// creates a new vector
	fn new(row: usize, col: usize) -> Dense {
		let mut v = Dense{rows: row, columns: col, element: vec![0.0; row*col]};
		v.set_size(row, col);
		return v;
	}

	/// resizes the vector
	fn set_size(&mut self, row: usize, col: usize) {
		self.element.resize(row*col,0.0);
		self.rows = row;
		self.columns = col;
	}

	/// returns the vector's rows
	fn get_rows(&self) -> usize {
		return self.rows;
	}

	/// returns the vector's columns
	fn get_columns(&self) -> usize {
		return self.columns;
	}


	fn get_linear_index(&self, i: usize, j: usize) -> usize {
		return (self.get_columns()-1)*self.get_rows() + j;
	}

	/// returns the i-th element of the dense static vector
	fn get_element(&self, i: usize, j: usize)-> f32 {

		if i >= self.get_rows() || j >= self.get_columns() {
			panic!("assignment out of bounds");
		}
			
		let n = self.get_linear_index(i,j);

		return self.element[n];
	}

	/// sets the i-th element of the dense static vector
	fn set_element(&mut self, i: usize, j: usize, value: f32) {

		if i >= self.get_rows() || j >= self.get_columns() {
			panic!("assignment out of bounds");
		}
		
		let n = self.get_linear_index(i,j);
		self.element[n] = value;
	}

}
