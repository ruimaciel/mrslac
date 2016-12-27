pub mod dense;
pub mod sparse_crs;


pub trait BasicReadableMatrix {

	fn get_rows(&self) -> usize;
    fn get_columns(&self) -> usize;
    fn get_element(&self, i: usize, j: usize)-> f32;

}


pub trait BasicWriteableMatrix {

	fn set_element(&mut self, i: usize, j: usize, value: f32);

}


pub trait SparseMatrix {

    /// returns the number of non-null matrix elements
    fn nnz(&self) -> usize;

}
