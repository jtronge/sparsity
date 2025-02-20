use std::collections::HashMap;
use sparsity::tensor::load_tensor;

/// Compute the tensor dimensions.
fn compute_tensor_dims(nmodes: usize, tensor: &[(Vec<usize>, f64)]) -> Vec<usize> {
    let mut min_dims: Vec<usize> = (0..nmodes).map(|m| tensor[0].0[m]).collect();
    let mut max_dims: Vec<usize> = (0..nmodes).map(|m| tensor[0].0[m]).collect();

    for (co, _) in tensor {
        for (m, i) in co.iter().enumerate() {
            if *i > max_dims[m] {
                max_dims[m] = *i;
            }
            if *i < min_dims[m] {
                min_dims[m] = *i;
            }
        }
    }

    max_dims
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next().expect("missing program name");
    let tensor_path = args.next().expect("missing tensor path argument");

    let (nmodes, mut tensor) = load_tensor(&tensor_path).expect("failed to load tensor");

    let tensor_dims = compute_tensor_dims(nmodes, &tensor);
    println!("tensor_dims: {:?}", tensor_dims);

    // TODO: Compute nonzero counts per slice
    // TODO: Check for empty slices
    let mut uniq_co = HashMap::new();
    // Check for duplicated entries
    while tensor.len() > 0 {
        let (co, value) = tensor.pop().expect("pop failed unexpectedly");
        if uniq_co.contains_key(&co) {
            println!("found duplicated coordinate: {:?}", co);
            continue;
        }
        uniq_co.insert(co, value);
    }
}
