#[allow(non_upper_case_globals)]

fn foo(s: &str) -> bool {
    // Make sure only valid modifiers appear, and only in allowed quantities.
    const MOD_u: usize = 1 << 0;
    const MOD_k: usize = 1 << 1;
    const MOD_h: usize = 1 << 2;
    const MOD_I: usize = 1 << 3;
    const MOD_G: usize = 1 << 4;
    const MOD_H: usize = 1 << 5;
    const MOD_P: usize = 1 << 6;
    const MOD_S: usize = 1 << 7;
    const MOD_D: usize = 1 << 8;
    const MOD_W: usize = 1 << 9;
    const MOD_e: usize = 1 << 10;
    let mut bitset = 0;
    let mut ps = 0;
    for c in s.bytes() {
        match c {
            b'u' if bitset & MOD_u == 0 => bitset |= MOD_u,
            b'k' if bitset & MOD_k == 0 => bitset |= MOD_k,
            b'h' if bitset & MOD_h == 0 => bitset |= MOD_h,
            b'I' if bitset & MOD_I == 0 => bitset |= MOD_I,
            b'G' if bitset & MOD_G == 0 => bitset |= MOD_G,
            b'H' if bitset & MOD_H == 0 => bitset |= MOD_H,
            b'P' if bitset & MOD_P == 0 => bitset |= MOD_P,
            b'S' if bitset & MOD_S == 0 => bitset |= MOD_S,
            b'D' if bitset & MOD_D == 0 => bitset |= MOD_D,
            b'W' if bitset & MOD_W == 0 => bitset |= MOD_W,
            b'e' if bitset & MOD_e == 0 => bitset |= MOD_e,
            b'p' if ps < 3 => ps += 1,
            _ => {
                return false;
            }
        }
    }
    println!("{:#010b}", &bitset);
    true
}

fn main() {
    let s: &str = "WGePppp";
    let bar = foo(&s);
    println!("{bar}");
}
