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
use std::fs::File;
use std::io::prelude::*;

mod file_reader;
mod integral_image;

fn main() -> std::io::Result<()> {
    let (image, windows) = file_reader::file_reader::file_reader("./input.txt".to_string());
    let integral_image = integral_image::integral_image::integral_image_computing(&image);
    let mut results = Vec::new();
    for window in windows {
        let result = integral_image::integral_image::window_sum(&integral_image, window);
        results.push(result);
    }
    let mut file = File::create("out.txt").expect("Unable to create file")  ;
    for result in &results {
        writeln!(file, "{}", result).expect("Unable to write data");
    }
    Ok(())
}
