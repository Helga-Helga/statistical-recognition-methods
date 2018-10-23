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
pub mod file_reader {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub fn file_reader(path: String) -> (Vec<Vec<u8>>, Vec<Vec<usize>>){
        let mut f = BufReader::new(File::open(path).unwrap());
        let mut num_line = String::new();
        f.read_line(&mut num_line).unwrap();
        let sizes: Vec<usize> = num_line
            .trim()
            .split(char::is_whitespace)
            .map(|number| number.parse().unwrap())
            .collect();
        println!("Image size: {:?}", sizes);
        let width: usize = sizes[0];
        let height: usize = sizes[1];

        let lines: Vec<String> = f.lines().map(|l| l.unwrap().trim().to_string()).collect();
        let array: Vec<u8> = lines
            .join(" ")
            .trim()
            .split(char::is_whitespace)
            .map(|number| number.parse().unwrap())
            .collect();

        let mut image = vec![vec![0u8; width]; height];
        for i in 0..height {
            for j in 0..width {
                image[i][j] = array[i * width + j];
            }
        }

        let mut windows = vec![vec![0usize; 4]; lines.len() - height];
        for i in 0..lines.len() - height {
            for j in 0..4 {
                windows[i][j] = array[width * height + i * 4 + j] as usize;
            }
        }

        println!("Image:");
        for row in &image {
            println!("{:?}", row);
        }

        println!("Windows:");
        for row in &windows {
            println!("{:?}", row);
        }
        (image, windows)
    }
}
