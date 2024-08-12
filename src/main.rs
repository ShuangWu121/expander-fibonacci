
use expander_rs::{
    circuit, BN254Config, Circuit, CircuitLayer, Config, GKRConfig, GKRScheme, GateAdd, GateMul, GateUni, M31ExtConfig, Prover, Verifier
};
use halo2curves::bn256::{self, Fr};
use sha2::Digest;


use rand::Rng;


fn gen_simple_circuit<C: GKRConfig>() -> Circuit<C> {
    let mut circuit = Circuit::default();
    let mut l0 = CircuitLayer::default();
    l0.input_var_num = 2;
    l0.output_var_num = 2;
    l0.add.push(GateAdd {
        i_ids: [0],
        o_id: 0,
        coef: C::CircuitField::from(1),
        is_random: false,
        gate_type: 1,
    });

    l0.add.push(GateAdd {
        i_ids: [0],
        o_id: 0,
        coef: C::CircuitField::from(1),
        is_random: false,
        gate_type: 1,
    });


    l0.add.push(GateAdd {
        i_ids: [0],
        o_id: 1,
        coef: C::CircuitField::from(1),
        is_random: false,
        gate_type: 1,
    });
    l0.add.push(GateAdd {
        i_ids: [1],
        o_id: 1,
        coef: C::CircuitField::from(1),
        is_random: false,
        gate_type: 1,
    });
    l0.mul.push(GateMul {
        i_ids: [0, 2],
        o_id: 2,
        coef: C::CircuitField::from(1),
        is_random: false,
        gate_type: 1,
    });
    circuit.layers.push(l0.clone());
    circuit
}

fn gen_one_layer_fibonacci_circuit<C: GKRConfig>() -> Circuit<BN254Config> {
    let mut circuit=Circuit::default();
    let mut l0=CircuitLayer::default();

    l0.input_var_num=1;
    l0.output_var_num=1;

    l0.add.push(GateAdd{
        i_ids:[0],
        o_id:1,
        coef:halo2curves::bn256::Fr::from(1 as u32),
        is_random:false,
        gate_type:1,
    });

    l0.add.push(GateAdd{
        i_ids:[1],
        o_id:1,
        coef:halo2curves::bn256::Fr::from(1 as u32),
        is_random:false,
        gate_type:1,
    });

    l0.add.push(GateAdd{
        i_ids:[1],
        o_id:0,
        coef:halo2curves::bn256::Fr::from(1 as u32),
        is_random:false,
        gate_type:1,
    });



    circuit.layers.push(l0.clone());
    
    let mut l1=CircuitLayer::<BN254Config>::default();
    l1.input_var_num=1;
    l1.output_var_num=1;

    l1.add.push(GateAdd{
        i_ids:[0],
        o_id:1,
        coef:halo2curves::bn256::Fr::from(1 as u32),
        is_random:false,
        gate_type:1,
    });

    l1.add.push(GateAdd{
        i_ids:[1],
        o_id:1,
        coef:halo2curves::bn256::Fr::from(1 as u32),
        is_random:false,
        gate_type:1,
    });

    l1.add.push(GateAdd{
        i_ids:[1],
        o_id:0,
        coef:halo2curves::bn256::Fr::from(1 as u32),
        is_random:false,
        gate_type:1,
    });

    circuit.layers.push(l1.clone());




    circuit
}


fn main(){

   //choose circuit to try
   //let mut circuit=gen_simple_circuit::<BN254Config>();
   let mut circuit=gen_one_layer_fibonacci_circuit::<BN254Config>();


   println!("circuit generated as \n {:?}",circuit);
   println!("rnd coefs identified {:?}",circuit.rnd_coefs_identified);
   circuit.rnd_coefs_identified=true;

   //circuit.set_random_input_for_test();

   circuit.layers[0].input_vals.evals = (0..(1 << circuit.log_input_size()))
            .map(|_| <BN254Config as GKRConfig>::SimdCircuitField::one())
            .collect();
   println!("inputs of the circuits \n: {:?}",
            circuit.layers[0]
                .input_vals
                .evals
                .iter()
                //.take(2)
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
       //.take(10)
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

   println!("Proof hash: ");
    sha2::Sha256::digest(&proof.bytes)
        .iter()
        .for_each(|b| print!("{} ", b));
    println!();

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

