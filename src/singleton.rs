use std::mem::transmute;
use std::sync::{Once, ONCE_INIT};

pub type VectorOfPairs = Vec<(String, String)>;

pub fn get<'a>() -> &'a mut VectorOfPairs {
    static mut _data: *const VectorOfPairs = 0 as *const VectorOfPairs;
    static ONCE: Once = ONCE_INIT;
    unsafe {
        ONCE.call_once(|| {
            let vec: VectorOfPairs = Vec::with_capacity(10);
            _data = transmute(Box::new(vec));

        });
        transmute(_data)
    }
}
