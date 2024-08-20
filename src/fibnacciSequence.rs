use expander_rs::{Circuit, GKRConfig};

pub fn generate_fibonacci_sequence<C: GKRConfig> (mut circuit: Circuit<C>)-> Circuit<C>{
    let mut x=<C as GKRConfig>::SimdCircuitField::from(1);
   let mut y=<C as GKRConfig>::SimdCircuitField::from(1);
   //generate fibonacci sequence as inputs
   circuit.layers[0].input_vals.evals = (0..(1 << circuit.log_input_size()))
   .flat_map(|_| {
       let x_next = y;
       let y_next = x + y;

       let result = vec![x, x_next];

    
       x = x_next;
       y = y_next;

       result
   })
   .take(1 << circuit.log_input_size()) // Ensure the output length matches the required size
   .collect();
   circuit
   
}