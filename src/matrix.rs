
/// Reference-based CSR version.
pub struct CSR<'a> {
    pub ptr: &'a [i64],
    pub col: &'a [i64],
    pub values: &'a [f64],
}

impl<'a> CSR<'a> {}

pub struct CSROwned {
    /// Row extents
    pub ptr: Vec<i64>,
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
        let mut col = vec![];
        let mut values = vec![];

        let mut last_row = 0;
        let mut last_col = 0;
        for (i, j, value) in entries {
            assert!(*i >= last_row);
            if last_row != *i {
                ptr.push(*i);
            } else {
                assert!(*j >= last_col);
            }
            col.push(*j);
            values.push(*value);
            last_row = *i;
            last_col = *j;
        }
        ptr.push(values.len() as i64);

        // TODO
        CSROwned {
            ptr,
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
        assert_eq!(csr.col.len(), 0);
        assert_eq!(csr.values.len(), 0);
    }

    #[test]
    fn basic_matrix() {
        // TODO
        assert!(false);
    }
}
