use num::complex::Complex;

fn calculate_mandelbrot( 
    //max value for the set
    max_iters:f64,

    //params to look for members of set within the space

    x_min:f64,
    x_max:f64,
    y_min:f64,
    y_max:f64,

    // params representing size of output in pixels

    width:usize,
    height:usize,
    ) -> Vec<Vec<usize>>{
        let mut rows =Vec::with_capacity(width);
        for img_y in 0..height{
            let mut row: Vec<usize> = Vec::with_capacity(height);
            for img_x in 0..width{
                let x_percent = (img_x as f64 / width as f64);
                let y_percent = (img_y as f64 / height as f64);
                let cx = x_min + (x_max - x_min) * x_percent; 
                let cy = y_min + (y_max - y_min) * y_percent; 
                let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
                row.push(escaped_at);
            }

            
        }
    }
fn main() {
    println!("Hello, world!");
}
