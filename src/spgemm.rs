use crate::matrix::{CSR, CSROwned};

pub fn spgemm(a: &CSROwned, b: &CSROwned, result: &mut Vec<(i64, i64, f64)>) {
    // TODO: Need to do some precomputation or look at some existing code

    // Do Gustavson's algorithm
    for p0 in 0..a.ptr.len()-1 {
        let i = a.row[p0];
        let a_start = a.ptr[p0];
        let a_stop = a.ptr[p0+1];
        // Current temporary entries for values in row i
        // let mut tmp_cols = vec![];
        // let mut tmp_values = vec![];
        // TODO

        for p1 in a_start..a_stop {
            let k0 = a.col[p1];
            let a_value = a.values[p1];
            for p2 in 0..b.ptr.len()-1 {
                let k1 = b.row[p2];
                let b_value = b.values[p2];
                // Column of A and row of B need to match
                if k0 != k1 {
                    continue;
                }

                let start_b = b.ptr[p2];
                let stop_b = b.ptr[p2+1];
                // Now iterate over all of B
                for p3 in start_b..stop_b {
                    let j = b.col[p3];

                    let mut found = false;
                    for entry in &mut *result {
                        // TODO: Need to avoid this loop
                        if entry.0 == i && entry.1 == j {
                            found = true;
                            entry.2 += a_value * b_value;
                        }
                    }

                    if !found {
                        result.push((i, j, a_value * b_value));
                    }
                }
            }
        }
    }
    result.sort_by_key(|entry| (entry.0, entry.1));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let mat = CSROwned::new(&[
            (0, 0, 1.0),
            (10, 3, 1.0),
            (10, 8, 1.0),
        ]);

        let mut output = vec![];
        spgemm(&mat, &mat, &mut output);

        assert_eq!(output.len(), 1);
        assert_eq!(output[0], (0, 0, 1.0));
    }

    #[test]
    fn simple2() {
        let mat = CSROwned::new(&[
            (0, 0, 1.0),
            (0, 1, 1.0),
            (1, 0, 1.0),
        ]);

        let mut output = vec![];
        spgemm(&mat, &mat, &mut output);

        assert_eq!(output.len(), 4);
        assert_eq!(output[0], (0, 0, 2.0));
        assert_eq!(output[1], (0, 1, 1.0));
        assert_eq!(output[2], (1, 0, 1.0));
        assert_eq!(output[3], (1, 1, 1.0));
    }
}
