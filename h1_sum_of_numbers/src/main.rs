/*
 * MIT License
 *
 * Copyright (c) 2018 Olga Laviagina
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
// Number of input images (objects)
const N: usize = 20;
// Number of possible values of each hidden parameter (digits from 0 to 9)
const D: usize = 9;

extern crate rand;
#[macro_use]
extern crate approx;
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
        cache.fill_matrix(index, probabilities);
    }

    let mut max_probability = 0.0; // the most probable value of d
    let mut max_index = 0; // index of the most probable value of d
    for d in 0..(N*D+1) { // for each possible value of d when there are N images
        if cache.matrix[N-1][d] > max_probability {
            max_probability = cache.matrix[N-1][d];
            max_index = d;
        }
    }
    println!("The most probable sum is: {}", max_index);
}

struct Cache {
    // Array of matrices: f1, f2, ..., f_N
    matrix: [[f64; (N*D+1)]; N],
}

impl Cache {
    // index: image number: from 0 to N
    fn fill_matrix(&mut self, index: usize, probabilities: [[f64; D]; N]) {
        // Fill the first row f1
        // f(0, d) = P(k0 = d | x0)
        // f(0, d) = self.matrix[0]
        if index == 0 {
            for d in 0..(N*D+1) { // for each possible value of d
                if d < D {
                    self.matrix[index][d] = probabilities[index][d];
                } else {
                    self.matrix[index][d] = 0.0;
                }
            }
        } else {
            /*
            Fill another rows: f1, f2, ..., f_{N-1}
            f(index, d) = sum_{j=0}^(N*D+1) P(k_{index = j} | x_{index}) * f(index - 1, d - j)
            f(index, d) = self.matrix[index]
            */
            self.matrix[index] = [0.0; (N*D+1)];
            for d in 0..(N*D+1) { // for each possible value of d
                for j in 0..D {
                    if d >= j as usize && index < N && j < N {
                        self.matrix[index][d as usize] +=
                            probabilities[index][j] * self.matrix[index-1][d as usize - j];
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Importing names from outer (for mod tests) scope
    use super::*;

    #[test]
    fn sum_of_probabilities_equals_one() {
        // Test if sum of probabilities in each row of cache.matrix is equal to 1
        let probabilities = [[0.1; D] ; N];

        let mut cache = Cache {
            matrix: [[0.0; (N*D+1)]; N],
        };


        for index in 0..N {
            cache.fill_matrix(index, probabilities);
            for d in 0..(N*D+1) {
                print!("{} ", cache.matrix[index][d]);
            }
        }

        let mut sums =  [0.0; N];
        for i in 0..N {
            for j in 0..(N*D+1) {
                sums[i] += cache.matrix[i][j];
            }
            relative_eq!(sums[i], 1.0, epsilon=1E-9);
        }
    }
}
