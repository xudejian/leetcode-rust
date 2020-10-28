#![no_main]
use leetcode;
use libfuzzer_sys::fuzz_target;
use rand::Rng;

fuzz_target!(|data: &[u8]| {
    if let Ok(mut s) = std::string::String::from_utf8(data.to_vec()) {
        let mut rng = rand::thread_rng();
        let mid = rng.gen_range(0, s.len() + 1);
        let pat = s.split_off(mid);
        let _ = leetcode::p28_implement_str_str::Solution::str_str(s, pat);
    }
});
