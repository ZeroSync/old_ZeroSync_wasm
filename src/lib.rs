use air::{ProcessorAir, PublicInputs};
use serde::{Deserialize, Serialize};
use winter_utils::{Deserializable, SliceReader};
use winterfell::StarkProof;

use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use winter_math::StarkField;
use sha3::{Digest, Sha3_256};


// The size of our compiled Bitcoin program
const PROGRAM_LENGTH: usize = 2726; 

static mut CURSOR: usize = PROGRAM_LENGTH;

fn read_felt(pub_inputs: &PublicInputs) -> String{
    unsafe {
        let word = pub_inputs.mem.1[CURSOR].unwrap().word();
        CURSOR += 1;
        return format!("{}", word)
    }
}

fn read_hex(pub_inputs: &PublicInputs) -> String{
    let felt = &read_felt(pub_inputs);
    return format!("0x{}",felt).to_string()
}

fn read_4_bytes(pub_inputs: &PublicInputs) -> String{
    let felt = &read_felt(pub_inputs)[3..];
    format!("{:0>8}", felt ).to_string()
}

use std::u32;

fn read_uint32(pub_inputs: &PublicInputs) -> u32{
    let felt = &read_felt(pub_inputs)[3..];
    let z = u32::from_str_radix(felt, 16);
    z.unwrap()
}

fn read_hash(pub_inputs: &PublicInputs) -> String{
    let mut block_hash: String = "".to_owned();

    for _ in 0..8 {
        block_hash.push_str(&read_4_bytes(&pub_inputs));
    }

    block_hash
}


fn read_utreexo_roots(pub_inputs: &PublicInputs) -> Vec<String>{
    let result = Vec::new()
    for _ in 0..28 {
        result.push( read_felt(pub_inputs) )
    }

    result
}




fn compute_program_hash(pub_inputs: &PublicInputs)->String{
    // Compute program hash
    let mut hasher = Sha3_256::new();
    let mut bytes: Vec<u8> = vec![];
    for i in 0..PROGRAM_LENGTH {
        let felt = pub_inputs.mem.1[i].unwrap().word();
        bytes.extend(felt.as_int().to_le_bytes());
    }
    hasher.update(&bytes);
    let digest = hasher.finalize();
    let program_hash = hex::encode(digest);
    return program_hash
}


#[derive(Serialize, Deserialize)]
pub struct BitcoinState {
    pub block_height: u32,
    pub difficulty: String,
    pub total_work: String,
    pub epoch_start_time: u32,
    pub prev_timestamps: Vec<u32>,
    pub best_block_hash: String,
    pub program_hash: String,
    pub utreexo_roots: Vec<String>,
}




#[wasm_bindgen]
pub fn verify(buffer: &Uint8Array) -> JsValue {
    // Load proof and public inputs
    let b = buffer.to_vec();
    let data: ProofData = bincode::deserialize(&b).unwrap();
    let proof = StarkProof::from_bytes(&data.proof_bytes).unwrap();
    let pub_inputs = PublicInputs::read_from(&mut SliceReader::new(&data.input_bytes[..])).unwrap();
    

    // Compute the program hash
    let program_hash = compute_program_hash(&pub_inputs);

    // Read the Block height 
    let block_height = read_uint32(&pub_inputs);

    // Read the block hash
    let block_hash = read_hash(&pub_inputs);

    // Read total chain work
    let total_work = read_hex(&pub_inputs);
    
    // Read difficulty
    let difficulty = read_hex(&pub_inputs);

    // Read time stamps
    let timestamps = Vec::new();
    for _ in 0..11 {
        timestamps.push( read_uint32(&pub_inputs) );        
    }
    
    let epoch_start_time = read_uint32(&pub_inputs);

    let utreexo_roots = read_utreexo_roots(&pub_inputs);


    // Verify execution
    match winterfell::verify::<ProcessorAir>(proof, pub_inputs) {
        Ok(_) => {

            let bitcoin_state = BitcoinState {
                prev_timestamps: timestamps,
                best_block_hash: block_hash,
                block_height: block_height,
                difficulty: difficulty,
                total_work: total_work,
                epoch_start_time: epoch_start_time,
                program_hash: program_hash,
                utreexo_roots: utreexo_roots,
            };

            serde_wasm_bindgen::to_value(&bitcoin_state).unwrap()
        },
        Err(_err) => serde_wasm_bindgen::to_value("Proof is invalid").unwrap(),
    }

}

#[derive(Serialize, Deserialize)]
struct ProofData {
    input_bytes: Vec<u8>,
    proof_bytes: Vec<u8>,
}
