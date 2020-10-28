#![no_main]
use leetcode;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut nums: Vec<i32> = data.to_vec().iter().map(|b| *b as i32).collect();
    let mut target: i32 = match nums.pop() {
        Some(i) => i,
        None => 0,
    };
    let _ = leetcode::p35_search_insert_position::Solution::search_insert(nums, target);
});
