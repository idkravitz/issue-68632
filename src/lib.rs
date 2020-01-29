pub fn max_subarray_bad(arr: &[i32]) -> (usize, usize, i32)
{
    let prefixes = arr
        .iter()
        .enumerate()
        .scan((0, 0), |s, (i, v)| {
            if s.1 > 0 {
                s.1 = s.1 + *v;
            } else {
                *s = (i, *v);
            }
            Some(*s)
        });
    let (right_idx, (left_idx, sum)) = prefixes
        .enumerate()
        .max_by_key(|&(_, (_, sum))| sum)
        .unwrap();
    
    (left_idx, right_idx + 1, sum)
}

pub fn max_subarray_good(arr: &[i32]) -> (usize, usize, i32)
{
    let zro = 0;
    let prefixes = arr
        .iter()
        .enumerate()
        .scan((0, 0), |s, (i, v)| {
            if s.1 > zro {
                s.1 = s.1 + *v;
            } else {
                *s = (i, *v);
            }
            Some(*s)
        });
    let (right_idx, (left_idx, sum)) = prefixes
        .enumerate()
        .max_by_key(|&(_, (_, sum))| sum)
        .unwrap();
    
    (left_idx, right_idx + 1, sum)
}