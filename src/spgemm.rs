pub struct MatrixCSR {
    pub ptr: Vec<i64>,
    pub col: Vec<i64>,
    pub values: Vec<f64>,
}

impl MatrixCSR {
    pub fn break_in_half<'a>(&'a self) -> MatrixCSRRef<'a> {
        todo!("not implemented yet")
    }
}

pub struct MatrixCSRRef<'a> {
    pub ptr: &'a [i64],
    pub col: &'a [i64],
    pub values: &'a [f64],
}

pub fn spgemm(a: &MatrixCSR, b: &MatrixCSR, result: &mut Vec<(i64, i64, f64)>) {
    // TODO: Run a parallel spgemm
}

#[cfg(test)]
mod test {}
