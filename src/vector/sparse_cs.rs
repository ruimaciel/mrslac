use vector::BasicReadableVector;
use vector::BasicWriteableVector;
use vector::SparseVector;


pub struct SparseCS {
	element: Vec<f32>,
	col_ind: Vec<usize>,
    size: usize
}


impl SparseCS {

	/// creates a new vector
	pub fn new(col: usize) -> SparseCS {
		let v = SparseCS{element: vec![], col_ind:vec![0], size: col};
		return v;
	}

	/// resizes the vector
	pub fn set_size(&mut self, new_size: usize) {
		if self.size < new_size {
			self.size = new_size;
			return;
		}

		//TODO trim vector
		let index = self.col_ind.iter().position(|&idx| idx >= new_size).unwrap();
		self.col_ind.truncate(index);
		self.element.truncate(index);
	}

}


impl BasicReadableVector for SparseCS {

	/// returns the vector's size
	fn get_size(&self) -> usize {
		return self.size;
	}


	/// returns the i-th element of the sparse vector
	fn get_element(&self, i: usize)-> f32 {

		if i >= self.get_size() {
			panic!("assignment out of bounds");
		}

		if self.nnz() == 0 {
			return 0.0
		}
			
		let index = match self.col_ind.binary_search(&i) {
			Err(_) => return 0.0,
			Ok(index) => index,
		};

		let idx = self.col_ind[index];

		return self.element[idx];
	}

}


impl BasicWriteableVector for SparseCS {

	/// sets the i-th element of the sparse vector
	fn set_element(&mut self, i: usize, value: f32) {

		if i >= self.get_size() {
			panic!("assignment out of bounds");
		}

		if i >= self.element.len() {
			self.element.push(value);
			self.col_ind.push(i);
			return;
		}
			
		let index = match self.col_ind.binary_search(&i) {
			Err(_) => panic!(),
			Ok(index) => index,
		};

		self.col_ind.insert(index, i);
		self.element.insert(index, value);
	}


    /// sets all elements to zero
    fn set_zero(&mut self) {
		self.element = vec![];
		self.col_ind = vec![0];
    }

}

impl SparseVector for SparseCS {

    /// returns the number of non-null vector elements
    fn nnz(&self) -> usize {
        return self.element.len();
    }

}

