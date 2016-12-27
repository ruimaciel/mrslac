extern crate mrslac;


#[test]
fn sparse_crs_init() {
    use mrslac::matrix::sparse_crs::SparseCRS;
    use mrslac::matrix::BasicReadableMatrix;

    let a = SparseCRS::new(3,3);

    for i in 0..a.get_rows() {
        for j in 0..a.get_columns() {
            assert_eq!(a.get_element(i, j), 0.0);
        }
    }
}


#[test]
fn sparse_crs_basic_matrix_interface() {
    use mrslac::matrix::sparse_crs::SparseCRS;
    use mrslac::matrix::BasicReadableMatrix;
    use mrslac::matrix::BasicWriteableMatrix;

    let mut a = SparseCRS::new(3,3);

    // test setters
    a.set_element(0, 0, 1.0);
    assert_eq!(a.get_element(0, 0), 1.0);

    a.set_element(1, 1, 1.0);
    assert_eq!(a.get_element(1, 1), 1.0);

    a.set_element(2, 2, 1.0);
    assert_eq!(a.get_element(2, 2), 1.0);

    // test all getters
    for i in 0..a.get_rows() {
        for j in 0..a.get_columns() {
            if i==j {
                assert_eq!(a.get_element(i, j), 1.0);
            }
            else {
                assert_eq!(a.get_element(i, j), 0.0);
            }
        }
    }
}
