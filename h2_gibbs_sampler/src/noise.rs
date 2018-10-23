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
pub mod noise {
    extern crate rand;
    use rand::Rng;

    pub fn noise_image(image: Vec<Vec<u32>>, epsilon: f64) -> Vec<Vec<u32>> {
        let mut noised_image = vec![vec![0u32; ::WIDTH]; ::HEIGHT];
        for i in 0..::HEIGHT {
            for j in 0..::WIDTH {
                if rand::thread_rng().gen_range(0., 1.) < epsilon {
                    if image[i][j] == 0 {
                        noised_image[i][j] = 1;
                    } else {
                        noised_image[i][j] = 0;
                    }
                } else {
                    if image[i][j] == 1 {
                        noised_image[i][j] = 1;
                    }
                }
            }
        }
        println!("Noised image:");
        for row in &noised_image {
            println!("{:?}", row);
        }
        return noised_image;
    }
}
