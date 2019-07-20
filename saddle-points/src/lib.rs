pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    /*
        unimplemented!(
            "find the saddle points of the following matrix: {:?}",
            input
        )
    */
    let mut points = Vec::new();
    let mut row_success = true;
    let mut column_success = true;
   /* println!("Here is the matrix:");
    //display matrix
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            print!(" {} ", input[i][j]);
        }
        println!();
    }*/

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            println!("Current point: {}", input[i][j]);
            for k in 0..input[i].len() {
                println!("Comparng {} and {}.", input[i][j], input[i][k]);
                if input[i][j] < input[i][k] {
                    row_success = false;
                    break;
                }
            }
            for u in 0..input.len() {
                println!("Comparing {} and {}.", input[i][j], input[u][j]);
                if input[i][j] > input[u][j] {
                    column_success = false;
                    break;
                }
            }
            if row_success && column_success {
                points.push((i, j));
            }
            row_success = true;
            column_success = true;
        }
    }
    points
}
