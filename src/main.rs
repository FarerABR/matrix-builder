trait Matrix<T> {
    #[doc = r"default method for making a empty array"]
    fn default() -> Self;
    #[doc = r"this makes a new array by type T"]
    ///
    /// pass the argument by a vec
    ///
    fn new(slice: &[Vec<T>]) -> Self;
    #[allow(rustdoc::broken_intra_doc_links)]
    #[doc = r"get method for returning the element in arr[i][j] position"]
    fn get(&self, i: usize, j: usize) -> Result<T, MatrixError>;
}

#[derive(Debug)]
#[doc = "This is the enum for indexing"]
enum MatrixError {
    /**
    this means that the given position is out of bounds of the Matrix
    */
    OutOfBoundIndexing(String),
    /**
    this means that the given position is not present in the matrix (aka the len is smaller than either i or j)
    */
    ForbiddenIndexing(String),
}

#[derive(Debug)]
struct MatrixRowMajor<T> {
    arr: Vec<T>,
    len: usize,
}
impl<T> Matrix<T> for MatrixRowMajor<T>
where
    T: Copy,
{
    #[allow(dead_code)]
    fn default() -> Self {
        Self {
            arr: Vec::new(),
            len: 0,
        }
    }

    #[allow(dead_code)]
    fn new(slice: &[Vec<T>]) -> Self {
        Self {
            arr: slice
                .iter()
                .flat_map(|inner| inner.iter())
                .cloned()
                .collect(),
            len: slice[0].len(),
        }
    }
    #[allow(dead_code)]
    fn get(&self, i: usize, j: usize) -> Result<T, MatrixError> {
        if j > self.len {
            return Err(MatrixError::ForbiddenIndexing(format!(
                "Forbidden indexing: the len of matrix is {} but the index is {}",
                self.len, j
            )));
        }
        if i * self.len + j > self.arr.len() {
            return Err(MatrixError::OutOfBoundIndexing(format!(
                "Out of bound indexing: len is {} but the index is {}",
                self.len,
                i * self.len + j > self.arr.len()
            )));
        }
        return Ok(self.arr[i * self.len + j]);
    }
}

struct MatrixColMajor<T> {
    arr: Vec<T>,
    len: usize,
}
impl<T> Matrix<T> for MatrixColMajor<T>
where
    T: Copy,
{
    #[allow(dead_code)]
    fn default() -> Self {
        Self {
            arr: Vec::new(),
            len: 0,
        }
    }

    #[allow(dead_code)]
    fn new(slice: &[Vec<T>]) -> Self {
        Self {
            arr: (0..slice[0].len())
                .flat_map(|j| slice.iter().map(move |inner| inner[j]))
                .collect(),
            len: slice.len(),
        }
    }

    #[allow(dead_code)]
    fn get(&self, i: usize, j: usize) -> Result<T, MatrixError> {
        if i > self.len {
            return Err(MatrixError::ForbiddenIndexing(format!(
                "Forbidden indexing: the len of matrix is {} but the index is {}",
                i, self.len
            )));
        }
        if j * self.len + i > self.arr.len() {
            return Err(MatrixError::OutOfBoundIndexing(format!(
                "Out of bound indexing: len is {} but the index is {}",
                self.len,
                j * self.len + i
            )));
        }
        return Ok(self.arr[j * self.len + i]);
    }
}

struct MatrixBlocky<T> {
    arr: Vec<MatrixRowMajor<T>>,
    len: usize,
}
impl<T> Matrix<T> for MatrixBlocky<T>
where
    T: Copy,
{
    #[allow(dead_code)]
    fn default() -> Self {
        Self {
            arr: Vec::new(),
            len: 0,
        }
    }

    #[allow(dead_code)]
    fn new(slice: &[Vec<T>]) -> Self {
        Self {
            arr: vec![
                MatrixRowMajor::new(
                    &slice
                        .iter()
                        .take(slice.len() / 2)
                        .map(|row| row.iter().take(slice.len() / 2).cloned().collect())
                        .collect::<Vec<Vec<T>>>(),
                ),
                MatrixRowMajor::new(
                    &slice
                        .iter()
                        .take(slice.len() / 2)
                        .map(|row| {
                            row.iter()
                                .skip(slice.len() / 2)
                                .take(slice.len() / 2)
                                .cloned()
                                .collect()
                        })
                        .collect::<Vec<Vec<T>>>(),
                ),
                MatrixRowMajor::new(
                    &slice
                        .iter()
                        .skip(slice.len() / 2)
                        .take(slice.len() / 2)
                        .map(|row| row.iter().take(slice.len() / 2).cloned().collect())
                        .collect::<Vec<Vec<T>>>(),
                ),
                MatrixRowMajor::new(
                    &slice
                        .iter()
                        .skip(slice.len() / 2)
                        .take(slice.len() / 2)
                        .map(|row| {
                            row.iter()
                                .skip(slice.len() / 2)
                                .take(slice.len() / 2)
                                .cloned()
                                .collect()
                        })
                        .collect::<Vec<Vec<T>>>(),
                ),
            ],
            len: slice.len(),
        }
    }

    #[allow(dead_code)]
    fn get(&self, i: usize, j: usize) -> Result<T, MatrixError> {
        let block = &self.arr[(i / (self.len / 2)) * 2 + (j / (self.len / 2))];
        block.get(i % (self.len / 2), j % (self.len / 2))
    }
}

fn main() {
    println!("{}", 5 / 5);
    println!(
        "Please run `cargo test` to see the test results or open ./doc/matrix_builder/index.html"
    )
}

#[cfg(test)]
mod matrix_test {
    use super::*;

    #[test]
    fn test_3x4_row_matrix() {
        let arr = vec![
            vec![1, 2, 3, 4], //
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ];
        let row = MatrixColMajor::new(&arr);

        assert_eq!(6, row.get(1, 1).unwrap());
        assert_eq!(1, row.get(0, 0).unwrap());
        assert_eq!(12, row.get(2, 3).unwrap());
        assert_ne!(14, row.get(5, 9).unwrap_or_default())
    }

    #[test]
    fn test_3x4_col_matrix() {
        let arr = vec![
            vec![1, 2, 3, 4], //
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ];
        let col = MatrixColMajor::new(&arr);

        assert_eq!(6, col.get(1, 1).unwrap());
        assert_eq!(1, col.get(0, 0).unwrap());
        assert_eq!(12, col.get(2, 3).unwrap());
        assert_ne!(14, col.get(5, 9).unwrap_or_default())
    }

    #[test]
    fn test_8x12_row_matrix() {
        let arr = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            vec![13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24],
            vec![25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36],
            vec![37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48],
            vec![49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60],
            vec![61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72],
            vec![73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84],
            vec![85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96],
        ];
        let row = MatrixRowMajor::new(&arr);

        assert_eq!(28, row.get(2, 3).unwrap());
        assert_eq!(68, row.get(5, 7).unwrap());
        assert_eq!(96, row.get(7, 11).unwrap());
        assert_ne!(14, row.get(5, 9).unwrap_or_default())
    }

    #[test]
    fn test_8x12_col_matrix() {
        let arr = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            vec![13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24],
            vec![25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36],
            vec![37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48],
            vec![49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60],
            vec![61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72],
            vec![73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84],
            vec![85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96],
        ];
        let col = MatrixColMajor::new(&arr);

        assert_eq!(28, col.get(2, 3).unwrap());
        assert_eq!(68, col.get(5, 7).unwrap());
        assert_eq!(96, col.get(7, 11).unwrap());
        assert_ne!(14, col.get(5, 9).unwrap_or_default())
    }

    #[test]
    fn test_6x6_block_matrix() {
        let arr = vec![
            vec![1, 2, 3, 4, 5, 6],
            vec![7, 8, 9, 10, 11, 12],
            vec![13, 14, 15, 16, 17, 18],
            vec![19, 20, 21, 22, 23, 24],
            vec![25, 26, 27, 28, 29, 30],
            vec![31, 32, 33, 34, 35, 36],
        ];
        let block = MatrixBlocky::new(&arr);

        assert_eq!(21, block.get(3, 2).unwrap());
        assert_eq!(28, block.get(4, 3).unwrap());
        assert_eq!(11, block.get(1, 4).unwrap());
        assert_ne!(14, block.get(3, 5).unwrap_or_default())
    }
}
