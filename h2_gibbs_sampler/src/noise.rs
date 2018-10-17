pub mod noise {
    extern crate rand;
    use rand::Rng;

    pub fn noise_image(image: Vec<Vec<u32>>, epsilon: f64) -> Vec<Vec<u32>> {
        let mut noised_image = vec![vec![0; ::WIDTH]; ::HEIGHT];
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
