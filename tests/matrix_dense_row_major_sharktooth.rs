extern crate mrslac;


#[test]
fn test_row_major_sharktooth_init() {
    use mrslac::matrix::dense::row_major_sharktooth::DenseRowMajorSharktooth;
    use mrslac::matrix::BasicReadableMatrix;

    let a = DenseRowMajorSharktooth::new(3,3);

    for i in 0..a.get_rows() {
        for j in 0..a.get_columns() {
            assert_eq!(a.get_element(i, j), 0.0);
        }
    }
}


#[test]
fn test_row_major_sharktooth_basic_matrix_interface() {
    use mrslac::matrix::dense::row_major_sharktooth::DenseRowMajorSharktooth;
    use mrslac::matrix::BasicReadableMatrix;
    use mrslac::matrix::BasicWriteableMatrix;

    let mut a = DenseRowMajorSharktooth::new(3,3);

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


#[test]
fn test_row_major_sharktooth_row_major_set_zero() {
    use mrslac::matrix::dense::row_major_sharktooth::DenseRowMajorSharktooth;
    use mrslac::matrix::BasicReadableMatrix;
    use mrslac::matrix::BasicWriteableMatrix;

    let mut a = DenseRowMajorSharktooth::new(3,3);

    // test setters
    a.set_zero();

    // test all getters
    for i in 0..a.get_rows() {
        for j in 0..a.get_columns() {
            assert_eq!(a.get_element(i,j), 0.0);
        }
    }
}

