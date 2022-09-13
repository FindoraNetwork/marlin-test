use std::env;
use marlin_test::marlin;

#[tokio::main]
async fn main() {
    let args:Vec<String> = env::args().collect();
    let test_case = &args[1];

    match test_case.as_str() {
        "tall-matrix-big" => marlin::prove_and_verify_with_tall_matrix_big(),
        "tall-matrix-small" => marlin::prove_and_verify_with_tall_matrix_small(),
        "squat-matrix-big" => marlin::prove_and_verify_with_squat_matrix_big(),
        "squat-matrix-small" => marlin::prove_and_verify_with_squat_matrix_small(),
        "square-matrix" => marlin::prove_and_verify_with_square_matrix(),
        "outlining" => marlin::prove_and_test_outlining(),
        "test-contract" => marlin::test_verification_contract().await,
        "all" => {
            marlin::prove_and_verify_with_tall_matrix_big();
            marlin::prove_and_verify_with_tall_matrix_small();
            marlin::prove_and_verify_with_squat_matrix_big();
            marlin::prove_and_verify_with_squat_matrix_small();
            marlin::prove_and_verify_with_square_matrix();
            marlin::prove_and_test_outlining();
        }
        &_ => {
            println!("Not a valid test case.")
        }
    }
}
