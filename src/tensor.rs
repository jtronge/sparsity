//! Sparse tensor data structures.
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;
use std::cmp::Ordering;
use std::path::Path;

#[derive(Debug)]
pub enum TensorError {
    IOError(std::io::Error),
    BadTensorEntry,
    BadTensorCoord,
    InvalidValue,
}

/// Load and validate a tensor from a file.
pub fn load_tensor<P: AsRef<Path>>(path: P) -> Result<(usize, Vec<(Vec<usize>, f64)>), TensorError> {
    let file = File::open(path).map_err(|err| TensorError::IOError(err))?;
    let mut reader = BufReader::new(file);
    let mut tensor_data: Vec<(Vec<usize>, f64)> = vec![];
    let mut line = String::new();
    let mut nmodes = None;
    let mut lno = 1;
    while reader.read_line(&mut line).map_err(|err| TensorError::IOError(err))? != 0 {
        let trimmed_line = line.trim();
        // Check if this is a comment line
        if trimmed_line.starts_with('#') || trimmed_line.len() == 0 {
            continue;
        }
        let parts: Vec<String> = trimmed_line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        if nmodes.is_none() {
            let _ = nmodes.insert(parts.len()-1);
        } else {
            let count = nmodes.expect("missing dimension count");
            if count != parts.len()-1 {
                return Err(TensorError::BadTensorEntry);
            }
        }

        let mut co = vec![];
        for idx in &parts[..parts.len()-1] {
            match usize::from_str_radix(idx, 10) {
                Ok(num) => co.push(num),
                Err(_) => return Err(TensorError::BadTensorCoord),
            }
        }

        let value = match f64::from_str(&parts[parts.len()-1]) {
            Ok(num) => num,
            Err(_) => return Err(TensorError::InvalidValue),
        };

        tensor_data.push((co, value));

        line.clear();
        lno += 1;
    }

    tensor_data.sort_by(|a, b| {
        let (akey, _) = &a;
        let (bkey, _) = &b;

        assert_eq!(akey.len(), bkey.len());
        for (i, j) in akey.iter().zip(bkey.iter()) {
            if i < j {
                return Ordering::Less;
            } else if i > j {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    });

    Ok((nmodes.expect("missing nmodes (the tensor is probably empty"), tensor_data))
}

/// Compressed Sparse Fiber data structure.
///
/// Based on http://shaden.io/pub-files/smith2017knl.pdf
struct CSF {
    fptr: Vec<Vec<i64>>,
    fids: Vec<Vec<i64>>,
    values: Vec<f64>,
}

impl CSF {
    fn new(tensor: Vec<(Vec<i64>, f64)>) -> CSF {
        // First double check that the tensor is sorted
        for i in 0..tensor.len()-2 {
            let a = &tensor[i];
            let b = &tensor[i+1];
            let (akey, _) = a;
            let (bkey, _) = b;
            for (i, j) in akey.iter().zip(bkey.iter()) {
                if i < j {
                    break;
                } else {
                    assert!(i <= j);
                }
            }
        }

        let mut fptr = vec![];
        let mut fids = vec![];
        let values = tensor.iter().map(|(_, value)| *value).collect();

        // Build the trie from the top down
        let count = tensor[0].0.len();
        for i in 0..count-1 {
            let mut last: Vec<i64> = tensor[0].0[..i+2].iter().map(|x| *x).collect();
            let mut k = 0;
            let mut ptr = vec![0];
            let mut ids = vec![tensor[0].0[i]];
            for entry in &tensor {
                if !entry.0.starts_with(&last) {
                    k += 1;
                    if !entry.0.starts_with(&last[..i+1]) {
                        ptr.push(k as i64);
                        ids.push(entry.0[i]);
                    }
                }
                last.copy_from_slice(&entry.0[..i+2]);
            }
            ptr.push(k+1 as i64);
            fptr.push(ptr);
            fids.push(ids);
        }
        fids.push(tensor.iter().map(|(key, _)| key[key.len()-1]).collect());

        CSF {
            fptr,
            fids,
            values,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_tensor() {
        // Test example from http://shaden.io/pub-files/smith2017knl.pdf
        let tensor = vec![
            (vec![1, 1, 1, 2], 1.0),
            (vec![1, 1, 1, 3], 2.0),
            (vec![1, 2, 1, 1], 3.0),
            (vec![1, 2, 1, 3], 4.0),
            (vec![1, 2, 2, 1], 5.0),
            (vec![2, 2, 2, 1], 6.0),
            (vec![2, 2, 2, 2], 7.0),
            (vec![2, 2, 2, 3], 8.0),
        ];
        let csf = CSF::new(tensor);

        assert_eq!(csf.fptr, vec![vec![0, 2, 3], vec![0, 1, 3, 4], vec![0, 2, 4, 5, 8]]);
        assert_eq!(csf.fids, vec![vec![1, 2], vec![1, 2, 2], vec![1, 1, 2, 2], vec![2, 3, 1, 3, 1, 1, 2, 3]]);
        assert_eq!(csf.values, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    }
}
