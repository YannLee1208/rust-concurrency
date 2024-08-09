use core::fmt;
use std::{
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

use anyhow::{anyhow, Result};

const NUM_THREADS: usize = 4;

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vec<T>,
    col: Vec<T>,
}

impl<T> MsgInput<T> {
    fn new(idx: usize, row: Vec<T>, col: Vec<T>) -> Self {
        Self { idx, row, col }
    }
}

pub struct MsgOutput<T> {
    idx: usize,
    data: T,
}

#[allow(dead_code)]
impl<T> MsgOutput<T> {
    fn new(idx: usize, data: T) -> Self {
        Self { idx, data }
    }
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
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

#[allow(dead_code)]
pub fn multiply_concurrency<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: std::fmt::Debug
        + Add<Output = T>
        + Mul<Output = T>
        + AddAssign
        + Copy
        + Default
        + Send
        + 'static,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix a.col must equal to b.row"));
    }

    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let data = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        data,
                    }) {
                        eprintln!("Error: {}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    let data_length = a.row * b.col;
    let mut receivers = Vec::with_capacity(data_length);
    let mut data = vec![T::default(); data_length];
    for i in 0..a.row {
        for j in 0..b.col {
            let row = a.data[i * a.col..(i + 1) * a.col].to_vec();
            let col = b.data[j..].iter().step_by(b.col).cloned().collect();
            let idx = i * b.col + j;
            let input = MsgInput::new(idx, row, col);
            let (tx, rx) = oneshot::channel::<MsgOutput<T>>();
            let msg = Msg { input, sender: tx };
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Error: {}", e);
            }
            receivers.push(rx);
        }
    }

    for rx in receivers {
        let msg = rx.recv()?;
        data[msg.idx] = msg.data;
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

    #[test]
    fn test_multiply_concurrency() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4], 2, 2);
        let b = Matrix::new([1, 2, 3, 4], 2, 2);
        let c = multiply_concurrency(&a, &b)?;
        assert_eq!(c.row, 2);
        assert_eq!(c.col, 2);
        assert_eq!(c.data, vec![7, 10, 15, 22]);
        Ok(())
    }

    #[test]
    fn test_concurrency_time() -> Result<()> {
        // build large matrix with 100 x 100
        let a = Matrix::new(vec![1; 100 * 100], 100, 100);
        let b = Matrix::new(vec![1; 100 * 100], 100, 100);
        let start = std::time::Instant::now();
        multiply(&a, &b)?;
        let duration = start.elapsed();
        eprintln!("multiply: {:?}", duration);

        let start = std::time::Instant::now();
        multiply_concurrency(&a, &b)?;
        let duration = start.elapsed();
        eprintln!("multiply_concurrency: {:?}", duration);
        Ok(())
    }
}
