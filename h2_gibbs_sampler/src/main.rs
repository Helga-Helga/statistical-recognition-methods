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
extern crate rand;
use rand::Rng;

mod input;
mod noise;
mod gibbs_sampler;

use input::input::generate_input_image;
use noise::noise::noise_image;
use gibbs_sampler::gibbs_sampler::gibbs_sampler;

const HEIGHT: usize = 10;
const  WIDTH: usize = 10;

fn main() {
    let (image, real_rows, real_columns) = generate_input_image();

    // Probability of changing color in pixel
    let epsilon: f64 = rand::thread_rng().gen_range(0., 1.);
    println!("Epsilon: {}", epsilon);

    let noised_image = noise_image(image, epsilon);

    let (recognized_rows, recognized_columns) = gibbs_sampler(noised_image, epsilon);
    println!("Errors in rows: {}", count_errors(real_rows, recognized_rows));
    println!("Errors in columns: {}", count_errors(real_columns, recognized_columns));
}

fn count_errors(real: Vec<u32>, recogized: Vec<u32>) -> u32{
    let mut errors = 0;
    for i in 0..real.len() {
        if real[i] != recogized[i] {
            errors += 1;
        }
    }
    return errors;
}
