pub mod input {
    extern crate rand;
    use rand::Rng;

    pub fn generate_input_image() -> (Vec<Vec<u32>>, Vec<u32>, Vec<u32>) {
        let mut filled_rows = vec![0; ::HEIGHT];
        let mut filled_columns = vec![0; ::WIDTH];
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
