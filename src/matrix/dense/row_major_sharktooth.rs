use matrix::BasicReadableMatrix;
use matrix::BasicWriteableMatrix;


/// A row-major shark-tooth dense matrix data structure
/// The matrix data structure is implemented as a block matrix comprised of 2x2 blocks
pub struct DenseRowMajorSharktooth {
	rows: usize,
	columns: usize,
	element: Vec<f32>
}


impl DenseRowMajorSharktooth {

	/// creates a new matrix
	pub fn new(row: usize, col: usize) -> DenseRowMajorSharktooth {
		let mut v = DenseRowMajorSharktooth{rows: row, columns: col, element: vec![0.0; row*col]};
		v.resize(row, col);
		return v;
	}

	/// resizes the matrix
	pub fn resize(&mut self, row: usize, col: usize) {
		let sharktooth_row = ( (row + 1) / 2 )*2;
		let sharktooth_col = col;
		self.element.resize(sharktooth_row*sharktooth_col, 0.0);

		self.rows = row;
		self.columns = col;
	}

    /// converts two-dimensional index to a row-major sharktooth index
	fn get_linear_index(&self, i: usize, j: usize) -> usize {

		let block_i = (i/2)*2;
		let block_idx = block_i*self.columns + j*2;

		let local_idx = i % 2;

		let idx = block_idx + local_idx;
		return idx;
	}

}


impl BasicReadableMatrix for DenseRowMajorSharktooth {

	/// returns the matrix' rows
	fn get_rows(&self) -> usize {
		return self.rows;
	}

	/// returns the matrix' columns
	fn get_columns(&self) -> usize {
		return self.columns;
	}

	/// returns the (i,j)-th element of the block matrix
	fn get_element(&self, i: usize, j: usize)-> f32 {

		if i >= self.get_rows() || j >= self.get_columns() {
			panic!("assignment out of bounds");
		}
			
		let n = self.get_linear_index(i,j);

		return self.element[n];
	}

}


impl BasicWriteableMatrix for DenseRowMajorSharktooth {

	/// sets the (i,j)-th element of the block row vector
	fn set_element(&mut self, i: usize, j: usize, value: f32) {

		if i >= self.get_rows() || j >= self.get_columns() {
			panic!("assignment out of bounds");
		}
		
		let n = self.get_linear_index(i,j);
		self.element[n] = value;
	}

    /// sets all elements to zero
    fn set_zero(&mut self) {

        for x in &mut self.element {
            *x = 0.0
        }
    }

}

#[cfg(test)]
mod tests {

	#[test]
	fn row_major_sharktooth_linear_index() {
    	use matrix::dense::row_major_sharktooth::DenseRowMajorSharktooth;
		let a = DenseRowMajorSharktooth::new(3,3);

		assert_eq!(a.get_linear_index(0, 0), 0);
		assert_eq!(a.get_linear_index(1, 0), 1);
		assert_eq!(a.get_linear_index(0, 1), 2);
		assert_eq!(a.get_linear_index(1, 1), 3);
		assert_eq!(a.get_linear_index(0, 2), 4);
		assert_eq!(a.get_linear_index(1, 2), 5);
		assert_eq!(a.get_linear_index(2, 0), 6);
		assert_eq!(a.get_linear_index(2, 1), 8);
		assert_eq!(a.get_linear_index(2, 2), 10);
	}

}