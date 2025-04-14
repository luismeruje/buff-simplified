use std::env;
use log::{error, info, warn};
// use log4rs;
use buff::compress::buff_slice::{BuffSliceCompress};
use std::time::{SystemTime};
//use buff::segment::{Segment};
use rand::prelude::*;
use std::time::Instant;

//use buff::compress::buff_simd::{run_buff_simd_encoding_decoding, run_buff_encoding_decoding_mybitvec, run_buff_majority_encoding_decoding};
//use buff::compress::buff_slice::{run_buff_slice_encoding_decoding, run_buff_slice_scalar_encoding_decoding};

fn main() {
    let mut rng = rand::thread_rng();
    // log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let args: Vec<String> = env::args().collect();
    info!("input args{:?}",args);
    let input_file = &args[1];
    let compression = &args[2];
    let int_scale = args[3].parse::<usize>().unwrap();
    let pred = args[4].parse::<f64>().unwrap();

    println!("ARGS: {}, {}, {}, {}, ",input_file, compression, int_scale,pred);
    //From: https://stackoverflow.com/questions/48218459/how-do-i-generate-a-vector-of-random-numbers-in-a-range
    let vals: Vec<f64> = (0..1000000).map(|_| rng.gen()).collect();
    //let mut seg = Segment::new_segment(None,SystemTime::now(),0,vals.clone(),None,None);
    let comp = BuffSliceCompress::new_buff_slice_compress(10,10,int_scale);
    let mut start1 = Instant::now();
    //let mut compressed= comp.buff_slice_encode(&mut seg);
    let mut elapsed = start1.elapsed();
    println!("Elapsed time for decompression is {} secs; {} millisecs; {} microsecs", elapsed.as_secs(), elapsed.subsec_millis(),elapsed.subsec_micros());
    start1 = Instant::now();
    //let decompressed_vals = comp.buff_slice_decode(compressed);
    elapsed = start1.elapsed();
    println!("Elapsed time for decompression is {} secs; {} millisecs; {} microsecs", elapsed.as_secs(), elapsed.subsec_millis(),elapsed.subsec_micros());




    
    // match compression.as_str(){
    //     "buff" => {
    //         run_buff_encoding_decoding_mybitvec(input_file,int_scale,pred);
    //     },
    //     "buff-simd" => {
    //         run_buff_simd_encoding_decoding(input_file,int_scale,pred);
    //     },
    //     "buff-slice" => {
    //         run_buff_slice_encoding_decoding(input_file,int_scale,pred);
    //     },
    //     "buff-slice-scalar" => {
    //         run_buff_slice_scalar_encoding_decoding(input_file,int_scale,pred);
    //     },
    //     "buff-major" => {
    //         run_buff_majority_encoding_decoding(input_file,int_scale,pred);
    //     },
    //     _ => {panic!("Compression not supported yet.")}
    // }

}