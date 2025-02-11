use std::collections::HashMap;
use sparsity::tensor::load_tensor;

fn main() {
    let mut args = std::env::args();
    let _ = args.next().expect("missing program name");
    let tensor_path = args.next().expect("missing tensor path argument");

    let (nmodes, mut tensor) = load_tensor(&tensor_path).expect("failed to load tensor");
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

    // Check for empty slices
    for m in 0..nmodes {
        // TODO
    }
}
