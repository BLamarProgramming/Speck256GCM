pub fn concatenate(counter: &Vec<u32>) -> (u64, u64) {
    let concatenated_counter_0 = (counter[0] as u64) << 32 | (counter[1] as u64);
    let concatenated_counter_1 = (counter[2] as u64) << 32 | (counter[3] as u64);
    return (concatenated_counter_0, concatenated_counter_1)
}
