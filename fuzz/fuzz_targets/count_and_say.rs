#![no_main]
use leetcode;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() > 0 && data[0] > 0 && data[0] <= 30 {
        let _ = leetcode::p38_count_and_say::Solution::count_and_say(data[0] as i32);
    }
});
