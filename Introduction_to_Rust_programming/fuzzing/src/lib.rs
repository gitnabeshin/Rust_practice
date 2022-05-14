pub fn sum(data: &[u8]) -> u8 {
    let mut ret = 0;
    for x in data {
        ret += x;
    }
    ret
}

pub fn sum_wrap(data: &[u8]) -> u8 {
    let mut ret = 0;
    for x in data {
        // avoid overflow(ex.  wrapping_add, checked_add, overflowing_add, saturating_add)
        ret = x.wrapping_add(ret);
    }
    ret
}