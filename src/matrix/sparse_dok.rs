use matrix::BasicReadableMatrix;
use matrix::BasicWriteableMatrix;
use matrix::SparseMatrix;

use std::collections::BTreeMap;

pub struct SparseDOK {
    val: BTreeMap<(usize, usize), f32>,
    rows: usize,
    columns: usize
}

impl SparseDOK {
    
	/// creates a new matrix
	pub fn new(rows: usize, columns: usize) -> SparseDOK {
		let m = SparseDOK{val: BTreeMap::new(),  rows: rows, columns: columns};
		return m;
	}

	/// resizes the vector
	pub fn resize(&mut self, rows: usize, columns: usize) {
        self.val = BTreeMap::new();
        self.rows = rows;
        self.columns = columns;
	}
}

impl BasicReadableMatrix for SparseDOK {

	/// returns the number of rows in the matrix
	fn get_rows(&self) -> usize {
		return self.rows;
	}

	/// returns the number of columns in the matrix
	fn get_columns(&self) -> usize {
		return self.columns;
	}

	/// returns the (i,j)-th element of the matrix
	fn get_element(&self, i: usize, j: usize)-> f32 {

		if i >= self.get_rows() || j >= self.get_columns() {
			panic!("access out of bounds");
        }

        let key = (i,j);
        
        match self.val.get(&key) {
            None => return 0.0,
            Some(value) => return *value,
        }
	}
}



impl BasicWriteableMatrix for SparseDOK {

	/// sets the (i,j)-th element of the sparse matrix 
	fn set_element(&mut self, i: usize, j: usize, new_value: f32) {

		if i >= self.get_rows() || j >= self.get_columns() {
			panic!("assignment out of bounds");
        }
        
        let key = (i,j);
        
        self.val.insert(key, new_value);
        
	}

    /// sets all elements to zero
    fn set_zero(&mut self) {

		self.val.clear()
    }

}


impl SparseMatrix for SparseDOK {

    /// returns the number of non-null matrix elements
    fn nnz(&self) -> usize {
        return self.val.len();
    }

}

