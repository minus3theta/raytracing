use ordered_float::OrderedFloat;

pub fn solve_equation(mut coef: Vec<Vec<f64>>, mut rhs: Vec<f64>) -> Vec<f64> {
    let n = coef.len();
    for step in 0..n {
        let (pivot, _) = coef
            .iter()
            .enumerate()
            .skip(step)
            .map(|(i, row)| (i, row[0]))
            .max_by_key(|(_, v)| OrderedFloat(v.abs()))
            .unwrap();
        coef.swap(step, pivot);
        rhs.swap(step, pivot);

        for r in (step + 1)..n {
            let scale = coef[r][step] / coef[step][step];
            for c in step..n {
                coef[r][c] -= scale * coef[step][c];
            }
            rhs[r] -= scale * rhs[step];
        }
    }
    let mut ans = vec![0.0; n];
    for step in (0..n).rev() {
        let mut right = rhs[step];
        for r in (step + 1)..n {
            right -= coef[step][r] * ans[r];
        }

        ans[step] = right / coef[step][step];
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e1() {
        let coef = vec![
            vec![6.0, 2.0, -1.0],
            vec![-3.0, 5.0, 3.0],
            vec![-2.0, 1.0, 3.0],
        ];
        let rhs = vec![48.0, 49.0, 24.0];
        let ans = solve_equation(coef.clone(), rhs.clone());

        for (r, row) in coef.iter().enumerate() {
            let left: f64 = row.iter().zip(ans.iter()).map(|(x, y)| x * y).sum();
            assert!((left - rhs[r]).abs() < 1e-4);
        }
    }
}
