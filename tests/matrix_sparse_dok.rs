extern crate mrslac;


#[test]
fn sparse_dok_init() {
    use mrslac::matrix::sparse_dok::SparseDOK;
    use mrslac::matrix::BasicReadableMatrix;

    let a = SparseDOK::new(3,3);

    for i in 0..a.get_rows() {
        for j in 0..a.get_columns() {
            assert_eq!(a.get_element(i, j), 0.0);
        }
    }
}


#[test]
fn sparse_dok_basic_matrix_interface() {
    use mrslac::matrix::sparse_dok::SparseDOK;
    use mrslac::matrix::BasicReadableMatrix;
    use mrslac::matrix::BasicWriteableMatrix;

    let mut a = SparseDOK::new(3,3);

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
fn sparse_dok_basic_sparse_interface() {
    use mrslac::matrix::sparse_dok::SparseDOK;
    use mrslac::matrix::BasicWriteableMatrix;
    use mrslac::matrix::SparseMatrix;

    let mut a = SparseDOK::new(3,3);

    assert_eq!(a.nnz(), 0);

    a.set_element(0, 0, 1.0);
    assert_eq!(a.nnz(), 1);

    a.set_element(1, 0, 2.0);
    assert_eq!(a.nnz(), 2);

    a.set_element(1, 1, 3.0);
    assert_eq!(a.nnz(), 3);

    a.set_element(2, 2, 4.0);
    assert_eq!(a.nnz(), 4);
}


#[test]
fn test_sparse_dok_set_zero() {
    use mrslac::matrix::sparse_dok::SparseDOK;
    use mrslac::matrix::BasicReadableMatrix;
    use mrslac::matrix::BasicWriteableMatrix;

    let mut a = SparseDOK::new(3,3);

    // test setters
    a.set_zero();

    // test all getters
    for i in 0..a.get_rows() {
        for j in 0..a.get_columns() {
            assert_eq!(a.get_element(i,j), 0.0);
        }
    }
}


