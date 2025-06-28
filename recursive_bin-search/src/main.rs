#[allow(dead_code)]
// Recursive list's items
pub fn rec_sum(arr: &[i32]) -> i32 {
    if arr.len() == 1  { return arr[0] };

    arr[0] + rec_sum(&arr[1..])
}

#[allow(dead_code)]
// Recursive list's length
pub fn rec_len(arr: &[i32]) -> usize {
    if arr == [] { return 0 };

    1 + rec_len(&arr[1..])
}

#[allow(dead_code)]
// Recursive list's highest item 
pub fn rec_high(arr: &[i32]) -> i32 {
    if arr.len() == 1 { return arr[0] };

    let next = rec_high(&arr[1..]);

    if next > arr[0] { next } else { arr[0] }
}

#[allow(dead_code)]
// Recursive Binary search
pub fn rec_bin_search(arr: &[i32], target: i32) -> Option<usize> {
    if arr.len() == 1 && arr[0] != target { return None };
    
    todo!()
}