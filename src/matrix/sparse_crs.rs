use matrix::BasicReadableMatrix;
use matrix::BasicWriteableMatrix;
use matrix::SparseMatrix;


pub struct SparseCRS {
	val: Vec<f32>,
	col_ind: Vec<usize>,
	row_ptr: Vec<usize>,
    columns: usize
}


impl SparseCRS {

	/// creates a new matrix
	pub fn new(rows: usize, columns: usize) -> SparseCRS {
		let m = SparseCRS{val: vec![], row_ptr: vec![0; rows+1], col_ind: vec![0], columns: columns};
		return m;
	}

	/// resizes the vector
	pub fn resize(&mut self, rows: usize, columns: usize) {
		self.val = vec![];
		self.row_ptr = vec![0; rows+1];
		self.col_ind = vec![0];
        self.columns = columns;
	}

}



impl BasicReadableMatrix for SparseCRS {

	/// returns the number of rows in the matrix
	fn get_rows(&self) -> usize {
		return self.row_ptr.len()-1;
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
		
        for idx in self.row_ptr[i]..self.row_ptr[i+1]{
            if self.col_ind[idx] == j {
                return self.val[idx]
            }
        }
        
        // this bit is only reached if no element was found
		return 0.0
	}
}



impl BasicWriteableMatrix for SparseCRS {

	/// sets the (i,j)-th element of the sparse matrix 
	fn set_element(&mut self, i: usize, j: usize, new_value: f32) {

		if i >= self.get_rows() || j >= self.get_columns() {
			panic!("assignment out of bounds");
		}
		
        
        let idx: usize = self.row_ptr[i];

        for idx in self.row_ptr[i]..self.row_ptr[i+1]{
            if self.col_ind[idx] == j {
                self.val[idx] = new_value;
                return
            } else if self.col_ind[idx] > j {
                break;
            }
        }

        // non-null element doesn't exist, so let's add one
        self.col_ind.insert(idx, j);
        self.val.insert(idx, new_value);

        for row_id in i+1..self.row_ptr.len() {
            self.row_ptr[row_id] = self.row_ptr[row_id] + 1;
        }
	}

}


impl SparseMatrix for SparseCRS {

    /// returns the number of non-null matrix elements
    fn nnz(&self) -> usize {
        return self.val.len();
    }

}

