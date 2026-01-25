use serde::{Serialize, de::DeserializeOwned};
use mpi::traits::*;
use postcard;

pub fn broadcast_data<T: Serialize + DeserializeOwned>(
    process: &impl Root,
    rank: usize,
    data: Option<&T>,
) -> T {
    if rank == 0 {
        let mut buf = postcard::to_allocvec(data.unwrap()).unwrap();
        let mut buf_size = buf.len();
        process.broadcast_into(&mut buf_size);
        process.broadcast_into(&mut buf[..]);
        postcard::from_bytes(&buf).unwrap()
    } else {
        let mut buf_size = 0usize;
        process.broadcast_into(&mut buf_size);
        let mut buf = vec![0u8; buf_size];
        process.broadcast_into(&mut buf[..]);
        postcard::from_bytes(&buf[..]).unwrap()
    }
}

