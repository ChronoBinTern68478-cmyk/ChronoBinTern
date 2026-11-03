// examples/bitnet_demo.rs

use mspn::{mspn_dot_product, Micro};

fn main() {
    println!("=== BitNet b1.58 neuron via MSPN ===\n");

    // Example neuron
    let inputs: [u8; 8] = [1, 0, 1, 1, 0, 1, 0, 1];
    let weights: [i8; 8] = [1, -1, 0, 1, -1, 1, 0, -1];

    // Convert weights to microsteps
    let steps: Vec<Micro> = weights.iter().map(|&w| match w {
        1 => Micro::F,
        -1 => Micro::R,
        0 => Micro::P,
        _ => panic!("invalid weight"),
    }).collect();

    println!("inputs:  {:?}", inputs);
    println!("weights: {:?}", weights);
    println!("steps:   {:?}\n", steps);

    // Method 1: standard sum
    let standard: i64 = inputs.iter()
        .zip(weights.iter())
        .map(|(&x, &w)| x as i64 * w as i64)
        .sum();
    println!("standard:    {}", standard);

    // Method 2: MSPN dot product
    let mspn = mspn_dot_product(&inputs, &steps);
    println!("MSPN:        {}", mspn);

    assert_eq!(standard, mspn);
    println!("\n✓ Both methods agree: {}", standard);

    // === Layer example ===
    println!("\n=== BitNet layer (4 neurons) ===\n");

    let layer_weights: [[i8; 8]; 4] = [
        [1, -1, 0, 1, -1, 1, 0, -1],
        [0, 0, 1, 1, -1, 0, 1, -1],
        [1, 1, 1, 0, 0, -1, -1, 0],
        [-1, -1, 0, 0, 1, 1, 0, 1],
    ];

    for (i, w) in layer_weights.iter().enumerate() {
        let steps: Vec<Micro> = w.iter().map(|&x| match x {
            1 => Micro::F,
            -1 => Micro::R,
            _ => Micro::P,
        }).collect();
        let out = mspn_dot_product(&inputs, &steps);
        println!("neuron {}: output = {}", i, out);
    }
}