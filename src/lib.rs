//! Matrix library
use std::ops::{Add, Mul};

/// Generic matrix data type
/// It stores any type of 2D matrix data of a numeric type 
/// (e.g. `i32`, `f64`, etc.)
/// The following operations are supported:
/// - `new`: create a new matrix of a given size
/// - `len`: get the number of elements in the matrix
/// - `shape`: get the shape of the matrix as a tuple
/// - `max`: get the maximum value in the matrix
/// - `argmax`: get the location (tuple) of the maximum value in the matrix
/// - `transpose`: get the transpose of the matrix
/// - indexing of the matrix using the `[(row, col)]` syntax
/// - addition of two matrices
/// - multiplication of two matrices
#[derive(Debug, Clone)]
pub struct Matrix<T> {
    // TODO: define your fields here
}

impl<T> Matrix<T>
where
    T: Copy // TODO: add additional trait bounds here, if needed
{
    /// Create a new matrix of a given size, initialized with default values
    pub fn new(n_rows: usize, n_cols: usize) -> Self {
        // TODO: implement this function
    }

    /// Get the number of elements in the matrix
    pub fn len(&self) -> usize {
        // TODO: implement this function
    }

    /// Get the shape of the matrix as a tuple
    pub fn shape(&self) -> (usize, usize) {
        // TODO: implement this function
    }

    /// Get the maximum value in the matrix
    pub fn max(&self) -> T {
        // TODO: implement this function
    }

    /// Get the location (tuple) of the maximum value in the matrix
    pub fn argmax(&self) -> (usize, usize) {
        // TODO: implement this function
    }

    /// Get the transpose of the matrix
    pub fn transpose(&self) -> Self {
        // TODO: implement this function
    }
}

// Indexing of the matrix using the `[(row, col)]` syntax
// e.g. let value = matrix[(0, 0)];
impl<T> std::ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    // TODO: implement the index function of the Index trait
}

// Mutable indexing of the matrix using the `[(row, col)]` syntax
// e.g. matrix[(0, 0)] = 1;
impl<T> std::ops::IndexMut<(usize, usize)> for Matrix<T> {
    // TODO: implement the index_mut function of the IndexMut trait
}

// Addition of two matrices (using reference types)
// e.g. let result = &matrix1 + &matrix2;
// Note: matrix addition is only defined for matrices of the same size
// See: https://en.wikipedia.org/wiki/Matrix_addition
impl<T> std::ops::Add for &Matrix<T>
where
    T: Copy // TODO: add additional trait bounds here, if needed
{
    type Output = Matrix<T>;

    // TODO: implement the add function of the Add trait 
    // see: https://doc.rust-lang.org/std/ops/trait.Add.html
}

// Multiplication of two matrices (using reference types)
// e.g. let result = &matrix1 * &matrix2;
// Note: matrix multiplication is only defined for matrices of compatible sizes
// See: https://en.wikipedia.org/wiki/Matrix_multiplication
impl<T> std::ops::Mul for &Matrix<T>
where
    T: Copy // TODO: add additional trait bounds here, if needed
{
    type Output = Matrix<T>;

    // TODO: implement the mul function of the Mul trait
    // see: https://doc.rust-lang.org/std/ops/trait.Mul.html
}

//////////////////////////////////////////////////////////////////////////////
// DO NOT EDIT BELOW THIS LINE
#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn test_new() {
        let _m1 = Matrix::<i32>::new(2, 3);
        let _m2 = Matrix::<f64>::new(4, 4);
    }

    #[test]
    fn test_size() {
        let m = Matrix::<i32>::new(2, 3);
        assert_eq!(m.len(), 6);
        assert_eq!(m.shape(), (2, 3));
    }

    #[test]
    fn test_indexing() {
        let mut m = Matrix::new(2, 3);

        assert_eq!(m[(0, 0)], 0);
        assert_eq!(m[(0, 1)], 0);
        assert_eq!(m[(0, 2)], 0);
        assert_eq!(m[(1, 0)], 0);
        assert_eq!(m[(1, 1)], 0);
        assert_eq!(m[(1, 2)], 0);

        m[(0, 0)] = 1;
        m[(0, 1)] = 2;
        m[(0, 2)] = 3;
        m[(1, 0)] = 4;
        m[(1, 1)] = 5;
        m[(1, 2)] = 6;

        assert_eq!(m[(0, 0)], 1);
        assert_eq!(m[(0, 1)], 2);
        assert_eq!(m[(0, 2)], 3);
        assert_eq!(m[(1, 0)], 4);
        assert_eq!(m[(1, 1)], 5);
        assert_eq!(m[(1, 2)], 6);
    }

    fn sample_matrix(n_rows: usize, n_cols: usize) -> Matrix<f64> {
        let mut m = Matrix::new(n_rows, n_cols);

        let mut v = 0.0;
        for i in 0..m.shape().0 {
            for j in 0..m.shape().1 {
                v += 1.0;
                m[(i, j)] = v;
            }
        }

        m
    }

    #[test]
    fn test_max() {
        let mut m = sample_matrix(2, 3);

        assert_eq!(m.max(), 6.0);

        m[(1, 1)] = 100.0;
        assert_eq!(m.max(), 100.0);
    }

    #[test]
    fn test_argmax() {
        let mut m = sample_matrix(2, 3);

        assert_eq!(m.argmax(), (1, 2));

        m[(1, 1)] = 100.0;
        assert_eq!(m.argmax(), (1, 1));
    }

    #[test]
    fn test_transpose() {
        let m = sample_matrix(2, 3);

        let mt = m.transpose();
        assert_eq!(mt.shape(), (3, 2));

        for i in 0..m.shape().0 {
            for j in 0..m.shape().1 {
                assert_eq!(m[(i, j)], mt[(j, i)]);
            }
        }
    }

    #[test]
    fn test_add() {
        let m1 = sample_matrix(2, 3);
        let m2 = sample_matrix(2, 3);

        let m3 = &m1 + &m2;

        assert_eq!(m3.shape(), (2, 3));

        for i in 0..m3.shape().0 {
            for j in 0..m3.shape().1 {
                assert_eq!(m3[(i, j)], m1[(i, j)] + m2[(i, j)]);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_add_panic() {
        let m1 = sample_matrix(2, 3);
        let m2 = sample_matrix(3, 2);

        let _m3 = &m1 + &m2;
    }

    #[test]
    fn test_mul() {
        let m1 = sample_matrix(2, 3);
        let m2 = sample_matrix(3, 2);

        let m3 = &m1 * &m2;

        assert_eq!(m3.shape(), (2, 2));

        assert_eq!(m3[(0, 0)], 22.0);
        assert_eq!(m3[(0, 1)], 28.0);
        assert_eq!(m3[(1, 0)], 49.0);
        assert_eq!(m3[(1, 1)], 64.0);
    }

    #[test]
    #[should_panic]
    fn test_mul_panic() {
        let m1 = sample_matrix(2, 3);
        let m2 = sample_matrix(2, 3);

        let _m3 = &m1 * &m2;
    }
}
