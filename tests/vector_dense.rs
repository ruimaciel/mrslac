extern crate mrslac;


#[test]
fn dense_init() {
    use mrslac::vector::dense::Dense;
    use mrslac::vector::BasicReadableVector;

    let a = Dense::new(3);

    for i in 0..a.get_size() {
        assert_eq!(a.get_element(i), 0.0);
    }
}


#[test]
fn dense_basic_matrix_interface() {
    use mrslac::vector::dense::Dense;
    use mrslac::vector::BasicReadableVector;
    use mrslac::vector::BasicWriteableVector;

    let mut a = Dense::new(3);

    // test setters
    a.set_element(0, 1.0);
    assert_eq!(a.get_element(0), 1.0);

    a.set_element(1, 1.0);
    assert_eq!(a.get_element(1), 1.0);

    a.set_element(2, 1.0);
    assert_eq!(a.get_element(2), 1.0);

    // test all getters
    for i in 0..a.get_size() {
        assert_eq!(a.get_element(i), 1.0);
    }
}


