use vector::BasicReadableVector;
use vector::BasicWriteableVector;


pub struct Dense {
	element: Vec<f32>,
}


impl Dense {

	/// creates a new vector
	pub fn new(col: usize) -> Dense {
		let mut v = Dense{element: vec![0.0; col]};
		return v;
	}

	/// resizes the vector
	pub fn set_size(&mut self, size: usize) {
		self.element.resize(size,0.0);
	}


}


impl BasicReadableVector for Dense {

	/// returns the vector's size
	fn get_size(&self) -> usize {
		return self.element.len();
	}


	/// returns the i-th element of the dense static vector
	fn get_element(&self, i: usize)-> f32 {

		if i >= self.get_size() {
			panic!("assignment out of bounds");
		}
			
		return self.element[i];
	}

}


impl BasicWriteableVector for Dense {

	/// sets the i-th element of the dense static vector
	fn set_element(&mut self, i: usize, value: f32) {

		if i >= self.get_size() {
			panic!("assignment out of bounds");
		}
			
		self.element[i] = value;
	}

}


