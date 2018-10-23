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
 pub mod integral_image {
     pub fn integral_image_computing(image: Vec<Vec<u8>>) -> Vec<Vec<u16>> {
         let height: usize = image.len();
         let width: usize = image[0].len();
         let mut integral_image = vec![vec![0u16; width]; height];
         let mut sum_in_lines = vec![vec![0u16; width]; height];
         for i in 0..height {
             for j in 0..width {
                 sum_in_lines[i][j] = image[i][j] as u16;
                 if j > 0 {
                     sum_in_lines[i][j] += sum_in_lines[i][j - 1];
                 }
                 integral_image[i][j] = sum_in_lines[i][j];
                 if i > 0 {
                     integral_image[i][j] += integral_image[i - 1][j];
                 }
             }
         }
         println!("Integral image:");
         for row in &integral_image {
             println!("{:?}", row);
         }
         integral_image
     }

     pub fn window_sum(integral_image: &Vec<Vec<u16>>, window: Vec<usize>) -> u16 {
         println!("Window: {:?}", window);
         let x = window[0];
         let y = window[1];
         let width = window[2];
         let height = window[3];
         let image_height = integral_image.len();
         let image_width = integral_image[0].len();
         let mut result: u16 = 0;
         if y + height - 1< image_height && x + width - 1 < image_width {
             result += integral_image[y + height - 1][x + width - 1];
         }
         if y + height - 1 < image_height && x > 0 {
             result -= integral_image[y + height - 1][x - 1];
         }
         if y > 0 && x + width - 1 < image_width {
             result -= integral_image[y - 1][x + width - 1];
         }
         if y > 0 && x > 0 {
             result += integral_image[y - 1][x - 1];
         }
        println!("Result: {}", result);
         result
     }
 }
