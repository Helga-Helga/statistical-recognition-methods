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
pub mod input {
    extern crate rand;
    use rand::Rng;

    pub fn generate_input_image() -> (Vec<Vec<u32>>, Vec<u32>, Vec<u32>) {
        let mut filled_rows = vec![0u32; ::HEIGHT];
        let mut filled_columns = vec![0u32; ::WIDTH];
        // Generate filled rows
        for i in 0..::HEIGHT {
            if rand::thread_rng().gen_range(0., 1.) > 0.5 {
                filled_rows[i] = 1;
            }
        }
        println!("Filled rows: {:?}", filled_rows);
        // Generate filled columns
        for j in 0..::WIDTH {
            if rand::thread_rng().gen_range(0., 1.) > 0.5 {
                filled_columns[j] = 1;
            }
        }
        println!("Filled columns: {:?}", filled_columns);
        // Create image from filled rows and columns
        let mut image = vec![vec![0u32; ::WIDTH]; ::HEIGHT];
        for i in 0..::HEIGHT {
            for j in 0..::WIDTH {
                if filled_rows[i] == 1 || filled_columns[j] == 1 {
                    image[i][j] = 1;
                }
            }
        }
        println!("Input image:");
        for row in &image {
            println!("{:?}", row);
        }

        return (image, filled_rows, filled_columns);
    }
}
