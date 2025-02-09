use sparsity::tensor::load_tensor;

fn main() {
    let mut args = std::env::args();
    let _ = args.next().expect("missing program name");
    let tensor_path = args.next().expect("missing tensor path argument");

    load_tensor(&tensor_path).expect("failed to load tensor");
}
