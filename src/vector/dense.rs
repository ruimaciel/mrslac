
pub struct Dense {
	element: Vec<f32>,
}


impl Dense {

	/// creates a new vector
	fn new(col: usize) -> Dense {
		let mut v = Dense{element: vec![0.0; col]};
		return v;
	}

	/// resizes the vector
	fn set_size(&mut self, size: usize) {
		self.element.resize(size,0.0);
	}


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

	/// sets the i-th element of the dense static vector
	fn set_element(&mut self, i: usize, value: f32) {

		if i >= self.get_size() {
			panic!("assignment out of bounds");
		}
			
		self.element[i] = value;
	}

}


#[test]
fn test_dense_static_new() {
	let vector_size = 20;
	let v = Dense::new(vector_size);
	assert_eq!(v.element.len(), vector_size);
}


#[test]
fn test_dense_static_setter() {
	let vector_size = 20;
	let mut v = Dense::new(vector_size);

	assert_eq!(v.get_size(), vector_size);

	let value: f32 = 1.0;
	v.set_element( 0, value);
	assert_eq!(v.element[0], value);

	v.set_element( vector_size-1, value);
	assert_eq!(v.element[vector_size-1], value);
}


#[test]
fn test_dense_static_getter() {
	let vector_size = 20;
	let mut v = Dense::new(vector_size);

	let set_value: f32 = 1.0;
	v.set_element( 0, set_value);
	let mut get_value = v.get_element(0);
	assert_eq!(set_value, get_value);

	v.set_element( vector_size-1, set_value);
	get_value = v.get_element(vector_size-1);
	assert_eq!(set_value, get_value);
}

