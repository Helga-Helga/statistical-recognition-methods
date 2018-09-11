// Number of images
const N: usize = 20;
// Number of hidden parameters (digits)
const D: usize = 10;

extern crate rand;
use rand::Rng;

fn main() {
    // Matrix of probabilities P(k_i / x_j)
    // k_i are numbers from 0 to 9
    // x_j are numbers of images
    let mut probabilities = [[0.0; D] ; N]; // 2D array: N rows x D columns

    for i in 0..N {
        let mut sum = 0.0;
        for j in 0..D {
            probabilities[i][j] = rand::thread_rng().gen_range(1, 101) as f64;
            sum += probabilities[i][j];
        }
        for j in 0..D {
            probabilities[i][j] = probabilities[i][j] as f64 / sum as f64;
        }
        println!("{:?}", probabilities[i]);
    }
}

struct Cache {
    // Array of matrices: f1, f2, ..., f_N
    matrices: Vec<Vec<Vec<f64>>>,
}

impl Cache {
    /*
    index: i in formulas
    d:     value d = k1 + k2 + ... + k_N
    I:     I_p = k_1 + k_2 + ... + k_{p-1}
    */
    fn fill_matrix(&mut self, index: usize, probabilities: <Vec<Vec<f64>>>,
                   d: f64, I: f64) -> &mut self {
        // Fill the first matrix f1
        if index == 0 {
            for i in 0..(D*N) { // for each possible value of I
                for j in 0..(D*N) { // for each possible value of d
                    if d > D - 1 {
                        self.matrices[index][i][j] = 0;
                    } else {
                        // f1(d, 0) = P(k1=d / x1)
                        self.matrices[index][i][j] = probabilities[i][d];
                    }
                }
            }
        } else {
            // Fill another matrices: f2, f3, ..., f_N
            // f_i(s, I_{N-i}) = sum_{j=0}^D P(k_i=j) * f_{i-1}(s, I_{N-i}+j)
            // f_i = self.matrices[i]
            for i in 0..(D*N) {
                for j in 0..(D*N) {
                    self.matrices[index][i][j] = 0;
                }
                for j in 0..(D*N) {
                    for sum_index in 0..D {
                        self.matrices[index][i][j] +=
                        probabilities[i][j] *
                        fill_matrix(&mut self, index-1, probabilities, d, I+i);
                    }
                }
            }
        }
        self.matrices
    }
}
