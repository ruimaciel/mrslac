pub mod dense;
pub mod sparse_cs;


pub trait BasicReadableVector {

	fn get_size(&self) -> usize;

    fn get_element(&self, i: usize)-> f32;

}

pub trait BasicWriteableVector {

	fn set_element(&mut self, i: usize, value: f32);

    /// sets all elements to zero
    fn set_zero(&mut self);

}

pub trait SparseVector {

    /// the number of non-zero elements in the vector
    fn nnz(&self) -> usize;

}
