// Copyright 2023-2023 the slutils-rs authors.

use std::ops;

/// Runtime fixed-size 2D matrix
pub struct Matrix2D<T> {
    /// Number of rows.
    rows: usize,
    /// Number of columns.
    columns: usize,
    /// Matrix values. Layout is array-of-structs (AOS).
    data: Vec<T>,
}

impl<T> Matrix2D<T> {
    /// Create a new matrix.
    ///
    /// # Arguments
    ///
    /// * `rows` - Number of rows.
    /// * `cols` - Number of columns.
    /// * `data` - Data values.
    ///
    /// # Returns
    ///
    /// New matrix.
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        Self {
            rows,
            columns: cols,
            data,
        }
    }

    /// Create a new matrix with all values set to zero.
    ///
    /// # Arguments
    ///
    /// * `rows` - Number of rows.
    /// * `cols` - Number of columns.
    ///
    /// # Returns
    ///
    /// New matrix.
    pub fn zeros(rows: usize, cols: usize) -> Self
    where
        T: Default + Copy,
    {
        Self {
            rows,
            columns: cols,
            data: vec![T::default(); cols * rows],
        }
    }

    /// Get the matrix dimensions.
    pub fn get_size(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }
}

impl<T> ops::Index<(usize, usize)> for Matrix2D<T> {
    type Output = T;

    /// Get a reference to a matrix element.
    ///
    /// # Arguments
    ///
    /// * `index` - Index of the element.
    ///
    /// # Returns
    ///
    /// Element value at the given index.
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.columns + index.1]
    }
}

impl<T> ops::IndexMut<(usize, usize)> for Matrix2D<T> {
    /// Get a mutable reference to a matrix element.
    ///
    /// # Arguments
    ///
    /// * `index` - Index of the element.
    ///
    /// # Returns
    ///
    /// Element value at the given index.
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * self.columns + index.1]
    }
}

#[cfg(test)]
mod tests {
    /// Test matrix creation.
    #[test]
    fn create_matrix2d() {
        // Create matrix:
        // 1 2 3
        // 4 5 6
        let matrix = crate::util::matrix::Matrix2D::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(matrix.get_size(), (2, 3));

        assert_eq!(matrix[(0, 0)], 1);
        assert_eq!(matrix[(0, 1)], 2);
        assert_eq!(matrix[(0, 2)], 3);
        assert_eq!(matrix[(1, 0)], 4);
        assert_eq!(matrix[(1, 1)], 5);
        assert_eq!(matrix[(1, 2)], 6);
    }

    /// Test matrix creation with default values.
    #[test]
    fn create_matrix2d_zeros() {
        // Check NxN matrix
        let matrix0 = crate::util::matrix::Matrix2D::<u8>::zeros(3, 3);
        assert_eq!(matrix0.get_size(), (3, 3));
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(matrix0[(i, j)], 0);
            }
        }

        // Check MxN matrices
        let matrix1 = crate::util::matrix::Matrix2D::<u8>::zeros(2, 4);
        assert_eq!(matrix1.get_size(), (2, 4));
        let matrix2 = crate::util::matrix::Matrix2D::<u8>::zeros(4, 2);
        assert_eq!(matrix2.get_size(), (4, 2));
    }
}
