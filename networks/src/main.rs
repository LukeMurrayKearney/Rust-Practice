extern crate nalgebra as na;
use std::ops::{MulAssign, Mul};

use na::{DVector};
use nalgebra_sparse::{CooMatrix, CsrMatrix};

fn main() {
    let mut infecteds_coo = CooMatrix::new(1,10);
    for i in vec![1,2,5,7,8].into_iter() {
        infecteds_coo.push(0,i,1.0);
    }
    let infecteds = CsrMatrix::from(&infecteds_coo);
    let mut rates_coo: CooMatrix<f64> = CooMatrix::new(1, 10);
    for i in 0..10 {
        rates_coo.push(0, i, i as f64);
    }
    let rates = CsrMatrix::from(&rates_coo);
    // println!("{:?}", infecteds.col_indices());

    let mut ds : Vec<f64> = Vec::new();
    for i in infecteds.col_indices().iter() {
        ds.push(rates.values()[*i]);
    }
    // println!("{:?}\n{:?}\n{:?}", infecteds, rates, infecteds.clone()*rates.transpose());
    println!("{:?}, {:?}",rates.col_indices().to_owned(), infecteds.col_indices().to_owned());
    let indices_rates = rates.col_indices().to_owned();
    let common_elements: Vec<&usize> = indices_rates
        .iter()
        .filter(|&x| infecteds.col_indices().to_owned().contains(x))
        .collect();
    println!("{:?}\n{:?}", rates.col_indices(), rates_coo.col_indices());
    // let answer: () = {
        // for i in infecteds.col_indices().iter() {
            // println!("{:?}", i);
        // }
    // };
}
