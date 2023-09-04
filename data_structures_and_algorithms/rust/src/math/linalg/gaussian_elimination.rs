//! # Resources
//!
//! - [**MathTheBeautiful's Video Series**](https://www.youtube.com/watch?v=l6B0w9juOcY&list=PLlXfTHzgMRUKXD88IdzS14F4NxAZudSmv&index=81)
//!     - [Linear Algebra 9a: Introduction to Gaussian Elimination](https://www.youtube.com/watch?v=l6B0w9juOcY)
//!     - [Linear Algebra 9b: The Operations of Gaussian Elimination](https://www.youtube.com/watch?v=Qq03PW55NQA)
//!     - [Linear Algebra 9c-: Review - Column Space Versus Null Space](https://www.youtube.com/watch?v=KT3NOmOOcmE)
//!     - [Linear Algebra 9c: Why Gaussian Elimination Works](https://www.youtube.com/watch?v=OApLVXbiWb4)
//!     - [Linear Algebra 9c1: Gaussian Elimination Cannot Be Used to Determine the Column Space](https://www.youtube.com/watch?v=ZP9SJGHFQ6k)
//!     - [Linear Algebra 9d: First Gaussian Elimination Example](https://www.youtube.com/watch?v=QX8A9np9VRQ)
//!     - [Linear Algebra 9e: Gaussian Elimination and Systems Without Solutions](https://www.youtube.com/watch?v=DlRcSZd0SIQ)
//!     - [Linear Algebra 9f: Row Switching in Gaussian Elimination](https://www.youtube.com/watch?v=E-y8XFuCssI)

use super::{LinearSystemSolver, Matrix};
use super::{Solution, Solutions};

pub struct GaussJordanElimination;

impl GaussJordanElimination {
    #[allow(dead_code)]
    fn solve_single(mut coefficients: Matrix, mut rhs: Vec<f64>) -> Solution {
        assert!(coefficients.is_square_matrix());
        let dim = coefficients.nrows();
        assert_eq!(dim, rhs.len());
        // from top to bottom (from left to right)
        for i in 0..dim {
            // if `matrix[i][i]` (which will become a pivot) is zero,
            // swap row `i` with a row where `matrix[i][i]` is not zero.
            if let Some(idx) = (i..dim).find(|&idx| coefficients[[idx, i]] != 0.) {
                if idx != i {
                    coefficients.swap_row(idx, i);
                    rhs.swap(idx, i);
                }
            } else {
                continue;
            };

            let pivot = coefficients[[i, i]];
            // scale the row by 1/pivot, so that the pivot becomes 1
            for coef in coefficients.row_mut(i).iter_mut().skip(i) {
                *coef /= pivot;
            }
            rhs[i] /= pivot;
            if i < dim {
                // subtract `row[i]` * `matrix[i][j]` from `row[j]` for each row below row `i`
                // to make `row[i]` zero
                for curr_i in i + 1..dim {
                    let factor = coefficients[[curr_i, i]];
                    for j in i..dim {
                        coefficients[[curr_i, j]] -= factor * coefficients[[i, j]];
                    }
                    rhs[curr_i] -= factor * rhs[i];
                }
            }
        }

        // from right to left
        let mut null_space_cols = Vec::new();
        for i in (1..dim).rev() {
            if coefficients[[i, i]] == 0.0 {
                if rhs[i] != 0. {
                    return Solution::None;
                } else {
                    null_space_cols.push(i);
                    continue;
                }
            }

            for curr_i in 0..i {
                let factor = coefficients[[curr_i, i]];
                for j in i..dim {
                    coefficients[[curr_i, j]] -= factor * coefficients[[i, j]];
                }
                rhs[curr_i] -= factor * rhs[i];
            }
        }
        if null_space_cols.is_empty() {
            Solution::Unique(rhs)
        } else {
            let null_space = null_space_cols
                .into_iter()
                .rev()
                .map(|j_| {
                    let mut ns_el = coefficients.column(j_).collect::<Vec<_>>();
                    ns_el[j_] = -1.;
                    ns_el
                })
                .collect();
            Solution::Infinite(rhs, null_space)
        }
    }

    #[allow(dead_code)]
    fn solve_augmented(mut augmented: Matrix) -> Solutions {
        let nrows = augmented.nrows();
        let ncols = augmented.ncols();
        // from top to bottom (from left to right)
        println!("{}", augmented);

        for i in 0..nrows {
            // if `matrix[i][i]` (which will become a pivot) is zero,
            // swap row `i` with a row where `matrix[i][i]` is not zero.
            if let Some(idx) = (i..nrows).find(|&idx| (augmented[[idx, i]] - 0.).abs() > 1e-6) {
                if idx != i {
                    augmented.swap_row(idx, i);
                }
            } else {
                continue;
            };

            let pivot = augmented[[i, i]];
            // scale the row by 1/pivot, so that the pivot becomes 1
            for coef in augmented.row_mut(i).iter_mut().skip(i) {
                *coef /= pivot;
            }
            if i < nrows {
                // subtract `row[i]` * `matrix[i][j]` from `row[j]` for each row below row `i`
                // to make `row[i]` zero
                for curr_i in i + 1..nrows {
                    let factor = augmented[[curr_i, i]];
                    for j in i..ncols {
                        augmented[[curr_i, j]] -= factor * augmented[[i, j]];
                    }
                }
            }
            println!("{}", augmented);
        }

        // from right to left
        let mut nullspace_cols = Vec::new();
        for i in (1..nrows).rev() {
            if (augmented[[i, i]] - 0.0).abs() < 1e-6 {
                nullspace_cols.push(i);
                continue;
            }

            for curr_i in 0..i {
                let factor = augmented[[curr_i, i]];
                for j in i..ncols {
                    augmented[[curr_i, j]] -= factor * augmented[[i, j]];
                }
            }
        }
        Solutions {
            augmented,
            nrows,
            ncols,
            nullspace_cols,
        }
    }
}

impl LinearSystemSolver for GaussJordanElimination {
    fn solve(augmented: Matrix) -> Solutions {
        Self::solve_augmented(augmented)
    }
}

impl Matrix {
    pub fn swap_row(&mut self, i0: usize, i1: usize) {
        self.0.swap(i0, i1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;
    #[test]
    #[rustfmt::skip]
    fn simple() {
        let m = matrix![
            1, 2,  3,   1;
            2, 4,  7,   2;
            3, 7, 11,   2;
        ];
        let res = GaussJordanElimination::solve(m).unwrap_first();
        assert_eq!(&res, &[3., -1., 0.]);
    }
    #[test]
    #[rustfmt::skip]
    fn no_solution() {
        let m = matrix![
            1, 2, 3,    3;
            4, 5, 6,    9;
            7, 8, 9,    6;
        ];
        let res = GaussJordanElimination::solve(m).first();
        assert_eq!(res, Solution::None);
    }
    #[test]
    #[rustfmt::skip]
    fn infinite_solutions() {
        let m = matrix![
            1, 2, 3,    3;
            4, 5, 6,    9;
            7, 8, 9,   15;
        ];
        let res = GaussJordanElimination::solve(m).first();
        assert_eq!(
            &res,
            &Solution::Infinite(vec![1.0, 1.0, 0.0], vec![vec![-1.0, 2.0, -1.0]])
        );

        let m = matrix![
            1, 2,  3,  4,  5   ,   -4;
            3, 7, 10, 13, 16   ,  -16;
            0, 0,  0,  0,  0   ,    0;
            0, 0,  0,  0,  0   ,    0;
            0, 0,  0,  0,  0   ,    0;
        ];
        let res = GaussJordanElimination::solve(m).first();
        assert_eq!(
            &res,
            &Solution::Infinite(
                vec![4.0, -4.0, 0.0, 0.0, 0.0],
                vec![
                    vec![1.0, 1.0, -1.0, 0.0, 0.0],
                    vec![2.0, 1.0, 0.0, -1.0, 0.0],
                    vec![3.0, 1.0, 0.0, 0.0, -1.0]
                ]
            )
        );
    }
}
