extern crate mrslac;


#[test]
fn sparsecs_init() {
    use mrslac::vector::sparse_cs::SparseCS;
    use mrslac::vector::BasicReadableVector;

    let a = SparseCS::new(3);

    for i in 0..a.get_size() {
        assert_eq!(a.get_element(i), 0.0);
    }
}


#[test]
fn sparsecs_sparse_vector_interface() {
    use mrslac::vector::sparse_cs::SparseCS;
    use mrslac::vector::SparseVector;

    let a = SparseCS::new(3);

    assert_eq!(a.nnz(), 0);
}


#[test]
fn sparsecs_given_empty_vector_when_set_ellement_then_nnz_is_one() {
    use mrslac::vector::sparse_cs::SparseCS;
    use mrslac::vector::BasicWriteableVector;
    use mrslac::vector::SparseVector;

    // Given
    let mut a = SparseCS::new(3);

    // When
    a.set_element(0, 1.0);

    // Then
    assert_eq!(a.nnz(), 1);
}

#[test]
fn sparsecs_basic_matrix_interface() {
    use mrslac::vector::sparse_cs::SparseCS;
    use mrslac::vector::BasicReadableVector;
    use mrslac::vector::BasicWriteableVector;

    let mut a = SparseCS::new(3);

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


#[test]
fn test_sparsecs_set_zero() {
    use mrslac::vector::sparse_cs::SparseCS;
    use mrslac::vector::BasicReadableVector;
    use mrslac::vector::BasicWriteableVector;

    let mut a = SparseCS::new(3);

    // test setters
    a.set_zero();

    // test all getters
    for i in 0..a.get_size() {
        assert_eq!(a.get_element(i), 0.0);
    }
}


