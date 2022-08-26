use ndarray::prelude::*;

fn main() {
    let arr1 = array![1., 2., 3., 4., 5., 6.];
    println!("1D array: {}", arr1);
    // 1D array VS 1D array VS 1D Vec
    let arr1 = array![1., 2., 3., 4., 5., 6.];
    println!("1D array: \t{}", arr1);

    let ls1 = [1., 2., 3., 4., 5., 6.];
    println!("1D list: \t{:?}", ls1);

    let vec1 = vec![1., 2., 3., 4., 5., 6.];
    println!("1D vector: \t{:?}", vec1);
}
