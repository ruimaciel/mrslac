pub mod dense_row_major;
pub mod sparse_crs;


pub trait BasicReadableMatrix {

    /// returns the number of rows
	fn get_rows(&self) -> usize;

    /// returns the number of columns
    fn get_columns(&self) -> usize;

    /// returns the (i,j)-th element of the matrix
    fn get_element(&self, i: usize, j: usize)-> f32;

}


pub trait BasicWriteableMatrix {

    /// sets the (i,j)-th element
	fn set_element(&mut self, i: usize, j: usize, value: f32);

    /// sets all elements to zero
    fn set_zero(&mut self);

}


pub trait SparseMatrix {

    /// returns the number of non-null matrix elements
    fn nnz(&self) -> usize;

}
