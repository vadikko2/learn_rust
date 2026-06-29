pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32) {
    if slice.len() < indices.len() {
        return;
    }
    for &idx in indices.iter() {
        if idx >= slice.len() {
            continue;
        }
        slice[idx] = value;
    }
}
