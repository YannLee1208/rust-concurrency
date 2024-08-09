use core::fmt;
use std::ops::{Add, AddAssign, Mul};

use anyhow::{anyhow, Result};

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

fn dot_product<T>(a: Vec<T>, b: Vec<T>) -> Result<T>
where
    T: Default + Add<Output = T> + Mul<Output = T> + AddAssign + Copy,
{
    if a.len() != b.len() {
        return Err(anyhow!("a.len must equal to b.len"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }

    Ok(sum)
}

#[allow(dead_code)]
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: std::fmt::Debug + Add<Output = T> + Mul<Output = T> + AddAssign + Copy + Default,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix a.col must equal to b.row"));
    }

    let mut data = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            data[i * b.col + j] = dot_product(
                a.data[i * a.col..(i + 1) * a.col].to_vec(),
                b.data[j..].iter().step_by(b.col).cloned().collect(),
            )?;
        }
    }

    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

impl<T: std::fmt::Debug> Matrix<T> {
    #[allow(dead_code)]
    // any data type which can by convert to vec
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T> std::fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }
            if i != self.row - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> std::fmt::Debug for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Matrix(row={}, col={}, data={})",
            self.row, self.col, self
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4], 2, 2);
        let b = Matrix::new([1, 2, 3, 4], 2, 2);
        let c = multiply(&a, &b)?;
        assert_eq!(c.row, 2);
        assert_eq!(c.col, 2);
        assert_eq!(c.data, vec![7, 10, 15, 22]);
        Ok(())
    }

    #[test]
    fn test_matrix_display() {
        let a = Matrix::new([1, 2, 3, 4], 2, 2);
        assert_eq!(format!("{}", a), "{1 2,3 4}");
    }

    #[test]
    fn test_matrix_debug() {
        let a = Matrix::new([1, 2, 3, 4], 2, 2);
        assert_eq!(format!("{:?}", a), "Matrix(row=2, col=2, data={1 2,3 4})");
    }

    #[test]
    fn test_dot_product() -> Result<()> {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        assert_eq!(dot_product(a, b)?, 32);
        Ok(())
    }
}
