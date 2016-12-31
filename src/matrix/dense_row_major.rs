use matrix::BasicReadableMatrix;
use matrix::BasicWriteableMatrix;


pub struct DenseRowMajor {
	rows: usize,
	columns: usize,
	element: Vec<f32>
}


impl DenseRowMajor {

	/// creates a new matrix
	pub fn new(row: usize, col: usize) -> DenseRowMajor {
		let mut v = DenseRowMajor{rows: row, columns: col, element: vec![0.0; row*col]};
		v.resize(row, col);
		return v;
	}

	/// resizes the matrix
	pub fn resize(&mut self, row: usize, col: usize) {
		self.element.resize(row*col,0.0);
		self.rows = row;
		self.columns = col;
	}

	fn get_linear_index(&self, i: usize, j: usize) -> usize {
		return (self.get_columns()-1)*i + j;
	}
}



impl BasicReadableMatrix for DenseRowMajor {

	/// returns the matrix' rows
	fn get_rows(&self) -> usize {
		return self.rows;
	}

	/// returns the matrix' columns
	fn get_columns(&self) -> usize {
		return self.columns;
	}

	/// returns the i-th element of the block matrix
	fn get_element(&self, i: usize, j: usize)-> f32 {

		if i >= self.get_rows() || j >= self.get_columns() {
			panic!("assignment out of bounds");
		}
			
		let n = self.get_linear_index(i,j);

		return self.element[n];
	}

}


impl BasicWriteableMatrix for DenseRowMajor {

	/// sets the i-th element of the block row vector
	fn set_element(&mut self, i: usize, j: usize, value: f32) {

		if i >= self.get_rows() || j >= self.get_columns() {
			panic!("assignment out of bounds");
		}
		
		let n = self.get_linear_index(i,j);
		self.element[n] = value;
	}

}

