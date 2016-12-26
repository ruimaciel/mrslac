pub mod dense;


pub trait BasicReadableMatrix {

	fn get_rows(&self) -> usize;
    fn get_columns(&self) -> usize;
    fn get_element(&self, i: usize, j: usize)-> f32;

}


pub trait BasicWriteableMatrix {

	fn set_element(&mut self, i: usize, j: usize, value: f32);

}
