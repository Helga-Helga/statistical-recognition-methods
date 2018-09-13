// Number of input images (objects)
const N: usize = 20;
// Number of possible values of each hidden parameter (digits from 0 to 9)
const D: usize = 9;

extern crate rand;
use rand::Rng;

fn main() {
    /*
    Matrix of probabilities P(k_i | x_j)
    k_i are numbers from 0 to 9
    x_j are numbers of images
    */
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
    }

    let mut cache = Cache {
        matrix: [[0.0; (N*D+1)]; N],
    };


    for index in 0..N {
        for d in 0..(D*N+1) {
            cache.fill_matrix(index, probabilities, d as f64);
        }
    }
}

struct Cache {
    // Array of matrices: f1, f2, ..., f_N
    matrix: [[f64; (N*D+1)]; N],
}

impl Cache {
    /*
    index: image number: from 0 to N
    d:     value d = k1 + k2 + ... + k_N
    */
    fn fill_matrix(&mut self, index: usize, probabilities: [[f64; D]; N], d: f64) {
        // Fill the first row f1
        // f(0, d) = P(k0 = d | x0)
        // f(0, d) = self.matrix[0]
        if index == 0 {
            for i in 0..(N*D+1) { // for each possible value of d
                if d <= i as f64 && i < D {
                    self.matrix[index][i] = probabilities[index][i];
                } else {
                    self.matrix[index][i] = 0.0;
                }
            }
        } else {
            /*
            Fill another rows: f1, f2, ..., f_{N-1}
            f(index, d) = sum_{i=0}^N P(k_{index = i} | x_{index}) * f(index - 1, d - i)
            f(index, d) = self.matrix[index]
            */
            self.matrix[index] = [0.0; (N*D+1)];
            for i in 0..(N*D+1) { // for each possible value of d
                for j in 0..N {
                    if d >= j as f64 && j < N && index < D {
                        self.matrix[index][i as usize] +=
                            probabilities[j][index] * self.matrix[index-1][d as usize - j];
                    }
                }
            }
        }
    }
}
