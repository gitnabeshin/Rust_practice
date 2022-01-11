pub fn sum(data: &[u8]) -> u8 {
    let mut ret = 0;
    for x in data {
        ret += x;
    }
    ret
}