pub mod crs;
pub mod dok;


pub trait SparseMatrix {

    /// returns the number of non-null matrix elements
    fn nnz(&self) -> usize;

}
