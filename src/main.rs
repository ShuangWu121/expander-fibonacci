
use expander_rs::{
    circuit, BN254Config, Circuit, CircuitLayer, Config, GKRConfig, GKRScheme, GateAdd, GateMul, GateUni, M31ExtConfig, Prover, Verifier,
};
use halo2curves::bn256::{self, Fr};
use sha2::Digest;


use rand::Rng;
mod fibnacciSequence;


///build fibonacci circuit with "layernumber" layers and "copynumber" of copies
/// example circuit of 2 layers and 2 copies
/// 
/// 
/// 
/// layer 2 outputs:[00]   [01]    [10]    [11]
///                  |      |        |      |
///                 Add    Add      Add    Add
///                    \  / |          \  / | 
///                     \/  |           \/  |
///                     /\  |           /\  |
///                    /  \ |          /  \ |
/// layer 1 inputs: [00]   [01]    [10]    [11]
///                  |      |        |      |
///                 Add    Add      Add    Add
///                    \  / |          \  / | 
///                     \/  |           \/  |
///                     /\  |           /\  |
///                    /  \ |          /  \ |
/// layer 0 inputs: [00]  [01]     [10]   [11]
/// 
/// 
/// 

fn build_fibonacci(layernumber: usize,copynumber: usize)->Circuit<BN254Config>{
    let mut circuit=Circuit::default();
    let mut l0=CircuitLayer::<BN254Config>::default();
    
    l0.input_var_num=((2 * copynumber) as f64).log2().ceil() as usize;
    println!("input_var_num {:?}",l0.input_var_num);
    l0.output_var_num=((2 * copynumber) as f64).log2().ceil() as usize;

    for i in 0..copynumber{
        l0.add.push(GateAdd{
            i_ids:[2*i],
            o_id:2*i+1,
            coef:halo2curves::bn256::Fr::from(1 as u32),
            is_random:false,
            gate_type:1,
        });
        l0.add.push(GateAdd{
            i_ids:[2 * i + 1],
            o_id:2 * i + 1,
            coef:halo2curves::bn256::Fr::from(1 as u32),
            is_random:false,
            gate_type:1,
        });

        l0.add.push(GateAdd{
            i_ids:[2*i+1],
            o_id:2*i,
            coef:halo2curves::bn256::Fr::from(1 as u32),
            is_random:false,
            gate_type:1,
        });
        
    }
    
    std::iter::repeat(()).take(layernumber).for_each(|_| {
        circuit.layers.push(l0.clone());
    });
  
    circuit
   
}



///build fibonacci circuit with roundnumber of rounds
/// example circuit of 3 rounds
/// 
/// 
/// 
/// layer 2 outputs:            [00]   [01]           [10][11]
///                              |       |             |    |
///                             Add     Add           Add  Add
///                             /  \    /             / \  /     
///                            /    \  /             /   \/   
///                           /      \/             /    /\ 
///                          /       /\            /    /  \
///                         /       /  neg        /    /    neg
///                        /       /    \        /    /      \
/// layer 1 inputs:     [000]    [001]  [010][011][100]       [101]
///                       |        |      |    |    |           |
///                      Add      Add    Add  Add  Add         Add
///                       /\      /\      \  / |    /\          |
///                      /  \    /  \      \/  |   /  \         |
///                     /    \  neg  \     /\  |  neg  \        |
///                    /      \/      \   /  \ | /      \       |
/// layer 0 inputs: [000]    [001]    [010]  [011]       [100] [101]   
/// inputs           x0       y0       x1     y1          x2     y2
/// 
/// 

fn build_parallel_fibonacci(roundsnumber: usize)->Circuit<BN254Config>{
    let mut circuit=Circuit::default();
    let mut l0=CircuitLayer::<BN254Config>::default();
    
    l0.input_var_num=((2 * roundsnumber) as f64).log2().ceil() as usize;
    println!("input_var_num {:?}",l0.input_var_num);
    l0.output_var_num=((3 * (roundsnumber-1)) as f64).log2().ceil() as usize;

    //define layer 1

    for i in 0..(roundsnumber-1){
        //gate =x[i]+y[i]
        l0.add.push(GateAdd{
            i_ids:[2*i],
            o_id:3*i,
            coef:halo2curves::bn256::Fr::from(1 as u32),
            is_random:false,
            gate_type:1,
        });
        l0.add.push(GateAdd{
            i_ids:[2 * i + 1],
            o_id:3 * i,
            coef:halo2curves::bn256::Fr::from(1 as u32),
            is_random:false,
            gate_type:1,
        });

        //gate x[i+1]=y[i]
        l0.add.push(GateAdd{
            i_ids:[2*i+1],// y[i]
            o_id:3*i+1,
            coef:halo2curves::bn256::Fr::from(1 as u32).neg(),
            is_random:false,
            gate_type:1,
        });

        
        l0.add.push(GateAdd{
            i_ids:[2*i+2],// x[i+1]
            o_id:3*i+1,
            coef:halo2curves::bn256::Fr::from(1 as u32),
            is_random:false,
            gate_type:1,
        });
     
        

        //gate y[i+1]
        
        l0.add.push(GateAdd{
            i_ids:[2*i+3],
            o_id:3*i+2,
            coef:halo2curves::bn256::Fr::from(1 as u32),
            is_random:false,
            gate_type:1,
        });
        
        
    
    }
    circuit.layers.push(l0.clone());

    let mut l1=CircuitLayer::<BN254Config>::default();

    l1.input_var_num=((3 * (roundsnumber-1)) as f64).log2().ceil() as usize;
    l1.output_var_num=((2 * roundsnumber) as f64).log2().ceil() as usize;
    
    for i in 0..(roundsnumber-1){
        //y[i+1]=x[i]+y[i]
        l1.add.push(GateAdd{
            i_ids:[3 * i ],
            o_id:2* i,
            coef:halo2curves::bn256::Fr::from(1 as u32),
            is_random:false,
            gate_type:1,
        });
    
        //gate x[i+1]=y[i]
        l1.add.push(GateAdd{
            i_ids:[3*i+2],// y[i]
            o_id:2*i,
            coef:halo2curves::bn256::Fr::from(1 as u32).neg(),
            is_random:false,
            gate_type:1,
        });

        l1.add.push(GateAdd{
            i_ids:[3*i+1],// y[i]=x[i]
            o_id:2*i+1,
            coef:halo2curves::bn256::Fr::from(1 as u32),
            is_random:false,
            gate_type:1,
        });
    
    }

    circuit.layers.push(l1.clone());
    circuit
   
}

fn main(){

   //sometime, the same parameter that run successfully can cause overflow problem with bad proof, donnot know why
   //let mut circuit=build_fibonacci(2, 2);
   let mut circuit=build_parallel_fibonacci(2);


   println!("circuit generated as \n {:?}",circuit);
   println!("rnd coefs identified {:?}",circuit.rnd_coefs_identified);
   circuit.rnd_coefs_identified=true;

   //circuit.set_random_input_for_test();

   //circuit.layers[0].input_vals.evals = (0..(1 << circuit.log_input_size()))
     //       .map(|_| <BN254Config as GKRConfig>::SimdCircuitField::one())
       //     .collect();

   

   let mut x=<BN254Config as GKRConfig>::SimdCircuitField::one();
   let mut y=<BN254Config as GKRConfig>::SimdCircuitField::one();
   
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
   

   println!("inputs of the circuits \n: {:?}",
            circuit.layers[0]
                .input_vals
                .evals
                .iter()
                .take(2)
                .collect::<Vec<_>>());
   println!("start evaluating circuit \n");
   circuit.evaluate();

   println!("circuit evaluated as \n {:?}",circuit.layers);

   println!("Output of the circuit: {:?}",
     circuit.layers
            .last()
            .unwrap()
            .output_vals
            .evals
            .iter()
            .take(10)
            .collect::<Vec<_>>());

   let mut prover=Prover::new(&Config::<BN254Config>::new(GKRScheme::Vanilla));
   prover.prepare_mem(&circuit);
   println!("start prove");
   let (claimed_v, proof) = prover.prove(&mut circuit);

   println!("claimed value is {:?}",claimed_v);
   println!("proof is {:?}",proof);

   println!("Proof generated. Size: {} bytes", proof.bytes.len());
    // first and last 16 proof u8
   println!("Proof bytes: ");
   proof.bytes.iter().take(30).for_each(|b| print!("{} ", b));
   print!("... ");
   proof
        .bytes
        .iter()
        .rev()
        .take(16)
        .rev()
        .for_each(|b| print!("{} ", b));
   println!();

   //println!("Proof hash: ");
    //sha2::Sha256::digest(&proof.bytes)
    //    .iter()
    //    .for_each(|b| print!("{} ", b));
    //println!();

    // Verify
    let verifier = Verifier::new(&Config::<BN254Config>::new(GKRScheme::Vanilla));
    println!("Verifier created.");

    assert!(verifier.verify(&mut circuit, &claimed_v, &proof));
    println!("Correct proof verified."); 

    let mut bad_proof = proof.clone();
    let rng = &mut rand::thread_rng();
    let random_idx = rng.gen_range(0..bad_proof.bytes.len());
    let random_change = rng.gen_range(1..256) as u8;
    bad_proof.bytes[random_idx] += random_change;
    assert!(!verifier.verify(&mut circuit, &claimed_v, &bad_proof));
    println!("Bad proof rejected.");

   
   // for fixed input
   for i in 0..(1 << circuit.log_input_size()) {
    circuit.layers.first_mut().unwrap().input_vals.evals[i] = halo2curves::bn256::Fr::from((i % 3 == 1) as u32);
   }

}

