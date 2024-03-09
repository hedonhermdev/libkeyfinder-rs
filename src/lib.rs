#[cxx::bridge]
pub mod ffi {
    #[repr(i32)]
    #[derive(Debug)]
    enum Key {
        A_MAJOR = 0,
        A_MINOR = 1,
        B_FLAT_MAJOR = 2,
        B_FLAT_MINOR = 3,
        B_MAJOR = 4,
        B_MINOR = 5,
        C_MAJOR = 6,
        C_MINOR = 7,
        D_FLAT_MAJOR = 8,
        D_FLAT_MINOR = 9,
        D_MAJOR = 10,
        D_MINOR = 11,
        E_FLAT_MAJOR = 12,
        E_FLAT_MINOR = 13,
        E_MAJOR = 14,
        E_MINOR = 15,
        F_MAJOR = 16,
        F_MINOR = 17,
        G_FLAT_MAJOR = 18,
        G_FLAT_MINOR = 19,
        G_MAJOR = 20,
        G_MINOR = 21,
        A_FLAT_MAJOR = 22,
        A_FLAT_MINOR = 23,
        SILENCE = 24,
    }

    struct PcmAudioData {
        pub channels: u32,
        pub sample_rate: u32,
        pub length: u32,
        pub samples: Vec<f32>,
    }

    extern "Rust" {}

    unsafe extern "C++" {
        include!("keyfinder/include/key.hpp");

        type Key;

        pub fn find_key(data: &PcmAudioData) -> Key;
    }
}

pub use ffi::*;

#[cfg(test)]
mod tests {

    #[test]
    fn test_silence() {
        use crate::ffi::{find_key, PcmAudioData};
        let data = PcmAudioData {
            channels: 2,
            length: 88200,
            sample_rate: 44100,
            samples: vec![0.0; 88200],
        };
        let key = find_key(&data);
        dbg!(key);
    }

}
