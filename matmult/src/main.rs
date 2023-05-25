use rand::{thread_rng, Rng};

use std::{fs::{self, File}, io::{BufReader, BufRead}};

use nalgebra::{SMatrix};

fn file_to_mat(file_path: &str) -> Vec<Vec<f64>> {
    let open_file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(open_file);

    let mut matrix: Vec<Vec<f64>> = vec![];
    for line in reader.lines() {
        let one_line: Vec<f64> = line.unwrap()
            .split(",")
            .into_iter()
            .map(|n| n.trim().parse::<f64>().unwrap())
            .collect();
        matrix.push(one_line);
    };
    matrix
}

fn get_rand() -> Vec<Vec<f64>>{
    let mut rng = thread_rng();
    let mut matrix: Vec<Vec<f64>> = vec![];
    for i in 0..100{
        let value: Vec<f64> = (0..100).map(|_| rng.gen()).collect();
        matrix.push(value)
    }
    return matrix; 
}

fn nalgebra_new_dot() {
    type Matrix2x3f = SMatrix<f64, 100, 100>;
    let mat = Matrix2x3f::new_random();
    let aa = mat * mat;
}

fn mat_array(mat: Vec<Vec<f64>>) {
    let mat_dim = mat.len();

    let zero_vec: Vec<f64> = vec![0.; mat_dim];
    let mut product = vec![zero_vec; mat_dim];

    for i in 0..mat_dim {
        for j in 0..mat_dim {

        product[i][j] = (0..mat_dim)
            .into_iter()
            .map(|k| mat[i][k]*mat[i][k])
            .sum::<f64>();

        }
    }
}

fn main() {
    let matrix= file_to_mat("/Users/sox/CODE/minigrep/example/mat.csv");
    //let matrix = get_rand();
    mat_array(matrix);
    // nalgebra_new_dot();
}



