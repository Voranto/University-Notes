

fn main() {
    println!("Please input the matrix size: ");
    let mut input_line : String= String::new();
    std::io::stdin().read_line(&mut input_line).expect("Failed to read line");
    
    let n: usize = input_line.trim().parse().expect("Input not an integer");
    let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; n + 1]; n];
    println!("Input the values for each row of the matrix, separated by spaces");
    for i in 0..n{
        let mut row_input = String::new();
        std::io::stdin().read_line(&mut row_input).expect("Failed to read line");
        let parts : Vec<&str> = row_input.split_whitespace().collect();
        
        if parts.len() != n + 1{
            println!("ERROR");
            return;
        }
        let mut j = 0;
        for part in parts{
           matrix[i][j] = part.parse::<f64>().expect("Not a valid number") as f64; 
            j+=1;
        }
    }
    println!("{:?}",matrix);
    let mut gaussian_elimination_matrix = gaussian_elimination(matrix);

    let mut solutions = get_solutions(gaussian_elimination_matrix);
}

fn get_solutions(gaussian_elimination_matrix : Vec<Vec<f64>>) -> Vec<f64>{
    let mut x = vec![0.0;gaussian_elimination_matrix.len()];
    let n = gaussian_elimination_matrix.len();
    
    println!("{:?}", gaussian_elimination_matrix);
    for i in (0..n).rev(){
        x[i] = gaussian_elimination_matrix[i][n];
        for j in i+1..n {
            x[i] -= gaussian_elimination_matrix[i][j]*x[j];
        }
        x[i] /= gaussian_elimination_matrix[i][i];
    }
    println!("{:?}",x);
    return x;
}


fn gaussian_elimination(matrix : Vec<Vec<f64>>) -> Vec<Vec<f64>>{
    let n = matrix.len();
    let mut l = vec![vec![0.0;n];n];
    let mut next_matrix = matrix.clone();
    let mut prev_matrix = matrix;
    for j in 0..n - 1{
        for i in j+1..n{
            l[i][j] = prev_matrix[i][j] / prev_matrix[j][j];
            for k in j..n{
                next_matrix[i][k] = prev_matrix[i][k] - l[i][j]*prev_matrix[j][k];
            }
            next_matrix[i][n] = prev_matrix[i][n] - l[i][j]*prev_matrix[j][n];
        }
        prev_matrix = next_matrix.clone();
        println!("{:?}", next_matrix);
    }

    return next_matrix;

}
