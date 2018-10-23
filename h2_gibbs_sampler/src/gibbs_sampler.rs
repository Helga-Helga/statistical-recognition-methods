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
pub mod gibbs_sampler {
    extern crate rand;
    use rand::Rng;

    pub fn gibbs_sampler(noised_image: Vec<Vec<u32>>, epsilon: f64) -> (Vec<u32>, Vec<u32>){
        // Generate filled columns
        let mut filled_rows = vec![0u32; ::HEIGHT];
        let mut filled_columns = vec![0u32; ::WIDTH];
        for j in 0..::WIDTH {
            if rand::thread_rng().gen_range(0., 1.) > 0.5 {
                filled_columns[j] = 1;
            }
        }
        for _i in 0..2000 {
            filled_rows = fill_rows(&noised_image, &filled_columns, epsilon);
            filled_columns = fill_columns(&noised_image, &filled_rows, epsilon);
        }
        // Extract each 30 result
        let mut filled_rows_result = vec![vec![0u32; ::HEIGHT]; 267];
        let mut filled_columns_result = vec![vec![0u32; ::WIDTH]; 267];
        for i in 0..8000 {
            filled_rows = fill_rows(&noised_image, &filled_columns, epsilon);
            filled_columns = fill_columns(&noised_image, &filled_rows, epsilon);
            if i%30 == 0 {
                filled_rows_result[i / 30] = filled_rows.to_vec();
                filled_columns_result[i / 30] = filled_columns.to_vec();
            }
        }
        // Count frequencies of filling lines
        let mut recognized_rows = vec![0u32; ::HEIGHT];
        let mut recognized_columns = vec![0u32; ::WIDTH];
        for i in 0..::HEIGHT {
            for j in 0..267 {
                if filled_rows_result[j][i] == 1 {
                    recognized_rows[i] += 1;
                }
            }
            recognized_rows[i] /= 267;
        }

        for i in 0..::WIDTH {
            for j in 0..267 {
                if filled_columns_result[j][i] == 1 {
                    recognized_columns[i] += 1;
                }
            }
            recognized_columns[i] /= 267;
        }
        println!("Recognized rows: {:?}", recognized_rows);
        println!("Recognized columns: {:?}", recognized_columns);
        return(recognized_rows, recognized_columns);
    }

    fn fill_rows(noised_image: &Vec<Vec<u32>>, filled_columns: &Vec<u32>, epsilon: f64) ->
                            Vec<u32> {
        let mut filled_rows = vec![0u32; ::HEIGHT];
        for i in 0..::HEIGHT {
            let mut prod1 = 1.;
            let mut prod2 = 1.;
            for j in 0..::WIDTH {
                prod1 *=
                    (epsilon / (1. - epsilon))
                    .powf((noised_image[i][j] as i32- filled_columns[j] as i32).abs() as f64);
                prod2 *=
                    (epsilon / (1. - epsilon)).powf((noised_image[i][j] as i32 - 1).abs() as f64);
            }
            let prob_r0 = prod1 / (prod1 + prod2);
            let prob_r1 = prod2 / (prod1 + prod2);
            if prob_r1 > prob_r0 {
                filled_rows[i] = 1;
            }
        }
        return filled_rows;
    }

    fn fill_columns(noised_image: &Vec<Vec<u32>>, filled_rows: &Vec<u32>, epsilon: f64) ->
                                Vec<u32> {
        let mut filled_columns = vec![0u32; ::WIDTH];
        for j in 0..::WIDTH {
            let mut prod1 = 1.;
            let mut prod2 = 1.;
            for i in 0..::HEIGHT {
                prod1 *=
                    (epsilon / (1. - epsilon))
                    .powf((noised_image[i][j] as i32- filled_rows[i] as i32).abs() as f64);
                prod2 *=
                    (epsilon / (1. - epsilon)).powf((noised_image[i][j] as i32 - 1).abs() as f64);
            }
            let prob_c0 = prod1 / (prod1 + prod2);
            let prob_c1 = prod2 / (prod1 + prod2);
            if prob_c1 > prob_c0 {
                filled_columns[j] = 1;
            }
        }
        return filled_columns;
    }
}
