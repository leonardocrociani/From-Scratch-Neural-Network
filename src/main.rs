mod data {
    pub mod loader;
}

mod utils {
    pub mod fs;
}

use data::loader;

fn main() {
    let file_path = "assets/ML-CUP24-TR.csv";
    let num_features = 11;
    let features: Vec<String> = (1..=num_features).map(|x| format!("FEATURE_{:?}", x)).collect();
    let targets: Vec<String> = vec!["x", "y", "z"].into_iter().map(|x| format!("TARGET_{}", x)).collect();

    println!("Loading data from file: {}...", file_path);
    println!("Features: {:?}", features);
    println!("Targets: {:?}", targets);

    let (X_train, Y_train) = loader::load_from_csv(file_path, features, targets, true, None).unwrap(); 

    println!("X_train: {:?}", X_train);
    println!("Y_train: {:?}", Y_train);
}
