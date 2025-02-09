
/// Reference-based CSR version.
pub struct CSR<'a> {
    pub ptr: &'a [usize],
    pub row: &'a [i64],
    pub col: &'a [i64],
    pub values: &'a [f64],
}

impl<'a> CSR<'a> {}

pub struct CSROwned {
    /// Row extents
    pub ptr: Vec<usize>,
    /// Row indices
    pub row: Vec<i64>,
    /// Column indices
    pub col: Vec<i64>,
    /// Actual values
    pub values: Vec<f64>,
}

impl CSROwned {
    /// Create a new owned Compresses Sparse Row (CSR) matrix.
    ///
    /// entries: a list of matrix entries (i, j, value) -- must be sorted by (i, j)
    pub fn new(entries: &[(i64, i64, f64)]) -> CSROwned {
        let mut ptr = vec![];
        let mut row = vec![];
        let mut col = vec![];
        let mut values = vec![];

        let mut last_row = -1;
        let mut last_col = -1;
        for (i, j, value) in entries {
            assert!(*i >= last_row);
            if last_row != *i {
                ptr.push(values.len());
                row.push(*i);
            } else {
                assert!(*j >= last_col);
            }
            col.push(*j);
            values.push(*value);
            last_row = *i;
            last_col = *j;
        }
        ptr.push(values.len());

        CSROwned {
            ptr,
            row,
            col,
            values,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let csr = CSROwned::new(&[]);
        assert_eq!(csr.ptr.len(), 1);
        assert_eq!(csr.ptr[0], 0);
        assert_eq!(csr.row.len(), 0);
        assert_eq!(csr.col.len(), 0);
        assert_eq!(csr.values.len(), 0);
    }

    #[test]
    fn basic_matrix() {
        let csr = CSROwned::new(&[
            (0, 0, 1.0),
            (1, 0, 2.0),
            (1, 2, 3.0),
            (1, 20, 4.0),
            (7, 1, 5.0),
            (8, 9, 6.0),
            (12, 2, 7.0),
            (16, 4, 8.0),
        ]);
        assert_eq!(csr.ptr.len(), 7);
        assert_eq!(csr.row, vec![0, 1, 7, 8, 12, 16]);
        assert_eq!(csr.col, vec![0, 0, 2, 20, 1, 9, 2, 4]);
        assert_eq!(csr.values, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    }
}
