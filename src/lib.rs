pub fn get_two_mut<T>(data: &mut [T], mut idx1: usize, mut idx2: usize) -> (&mut T, &mut T) {
    let reversed = if idx1 > idx2 {
        (idx1, idx2) = (idx2, idx1);
        true
    } else {
        false
    };

    let (left, right) = data.split_at_mut(idx1 + 1);
    let idx2 = idx2 - (idx1 + 1);
    let res1 = &mut left[idx1];
    let res2 = &mut right[idx2];

    if reversed {
        (res2, res1)
    } else {
        (res1, res2)
    }
}
