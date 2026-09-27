//! FIN mode: signed decimal fixed-point, i64 scaled by 10^4.
//!
//! Range is +/-922,337,203,685,477.5807, i.e. ~922 trillion VND with 4 decimals.
//! Rounding (x, /, fin_round) is half away from zero, like a shop calculator.
//! Number entry works on the magnitude; JS keeps the sign and the "." state.

use crate::{
    set_err, BUF, BUF_LEN, ERR_BAD_DIGIT, ERR_BAD_OP, ERR_DIV0, ERR_OVERFLOW, OP_ADD, OP_DIV,
    OP_MUL, OP_NEG, OP_SUB,
};
use core::ptr::addr_of_mut;

const FRAC_DIGITS: usize = 4;
const SCALE: i64 = 10_000;
const POW10: [i64; FRAC_DIGITS + 2] = [1, 10, 100, 1_000, 10_000, 100_000];

#[inline(always)]
fn overflow() -> i64 {
    set_err(ERR_OVERFLOW);
    0
}

/// n / d rounded half away from zero. `d` must be non-zero.
#[inline(always)]
fn div_round(n: i128, d: i128) -> i128 {
    let q = n / d;
    let r = n % d;
    if 2 * r.abs() >= d.abs() {
        if (n < 0) != (d < 0) {
            q - 1
        } else {
            q + 1
        }
    } else {
        q
    }
}

#[inline(always)]
fn narrow(v: i128) -> i64 {
    if v > i64::MAX as i128 || v < i64::MIN as i128 {
        overflow()
    } else {
        v as i64
    }
}

/// Appends digit `d` to the magnitude `mag`.
/// `frac < 0`: still in the integer part; `frac = n`: n decimals typed after ".".
#[no_mangle]
pub extern "C" fn fin_push_digit(mag: i64, d: u32, frac: i32) -> i64 {
    if d > 9 || mag < 0 {
        set_err(ERR_BAD_DIGIT);
        return mag;
    }
    let d = d as i64;
    if frac < 0 {
        match mag.checked_mul(10).and_then(|m| m.checked_add(d * SCALE)) {
            Some(n) => n,
            None => {
                set_err(ERR_OVERFLOW);
                mag
            }
        }
    } else if (frac as usize) < FRAC_DIGITS {
        mag + d * POW10[FRAC_DIGITS - 1 - frac as usize]
    } else {
        set_err(ERR_BAD_DIGIT); // no room for more decimals
        mag
    }
}

/// Removes the last typed digit (same `frac` convention as `fin_push_digit`).
#[no_mangle]
pub extern "C" fn fin_pop_digit(mag: i64, frac: i32) -> i64 {
    if frac < 0 {
        (mag / (10 * SCALE)) * SCALE
    } else if frac == 0 || frac as usize > FRAC_DIGITS {
        mag
    } else {
        mag - mag % POW10[FRAC_DIGITS + 1 - frac as usize]
    }
}

/// `a op b` for + - x / and unary NEG (on `a`). Errors return 0 and latch.
#[no_mangle]
pub extern "C" fn fin_calc(op: u32, a: i64, b: i64) -> i64 {
    match op {
        OP_ADD => a.checked_add(b).unwrap_or_else(overflow),
        OP_SUB => a.checked_sub(b).unwrap_or_else(overflow),
        OP_MUL => narrow(div_round(a as i128 * b as i128, SCALE as i128)),
        OP_DIV => {
            if b == 0 {
                set_err(ERR_DIV0);
                0
            } else {
                narrow(div_round(a as i128 * SCALE as i128, b as i128))
            }
        }
        OP_NEG => a.checked_neg().unwrap_or_else(overflow),
        _ => {
            set_err(ERR_BAD_OP);
            0
        }
    }
}

/// Rounds to `decimals` (0 for VND, 2 for USD).
#[no_mangle]
pub extern "C" fn fin_round(v: i64, decimals: u32) -> i64 {
    if decimals as usize >= FRAC_DIGITS {
        return v;
    }
    let unit = POW10[FRAC_DIGITS - decimals as usize] as i128;
    narrow(div_round(v as i128, unit) * unit)
}

/// Formats with "," thousands and "." decimals into the shared buffer; returns length.
/// `frac < 0`: trim trailing zeros; `frac = n`: exactly n decimals (truncated, round first).
#[no_mangle]
pub extern "C" fn fin_format(v: i64, frac: i32) -> u32 {
    let mag = v.unsigned_abs();
    let mut int = mag / SCALE as u64;
    let raw = mag % SCALE as u64;

    let (mut f, nd) = if frac < 0 {
        let (mut f, mut nd) = (raw, FRAC_DIGITS);
        while nd > 0 && f % 10 == 0 {
            f /= 10;
            nd -= 1;
        }
        (f, nd)
    } else {
        let nd = (frac as usize).min(FRAC_DIGITS);
        (raw / POW10[FRAC_DIGITS - nd] as u64, nd)
    };
    let nonzero = int != 0 || f != 0;

    let buf = unsafe { &mut *addr_of_mut!(BUF) };
    let mut i = BUF_LEN;
    let mut put = |c: u8| {
        i -= 1;
        buf[i] = c;
    };

    if nd > 0 {
        for _ in 0..nd {
            put(b'0' + (f % 10) as u8);
            f /= 10;
        }
        put(b'.');
    }
    let mut n = 0;
    loop {
        if n > 0 && n % 3 == 0 {
            put(b',');
        }
        put(b'0' + (int % 10) as u8);
        int /= 10;
        n += 1;
        if int == 0 {
            break;
        }
    }
    if v < 0 && nonzero {
        put(b'-');
    }

    let len = BUF_LEN - i;
    buf.copy_within(i.., 0);
    len as u32
}

/// Parses `len` bytes from the shared buffer, e.g. "25,790.00" or "-1234.5".
/// Commas and spaces are ignored; decimals beyond 4 are rounded.
#[no_mangle]
pub extern "C" fn fin_parse(len: u32) -> i64 {
    let buf = unsafe { &*addr_of_mut!(BUF) };
    let s = &buf[..(len as usize).min(BUF_LEN)];

    let mut neg = false;
    let mut int: i128 = 0;
    let mut frac: i128 = 0;
    let mut nd = 0usize;
    let mut round_up = false;
    let mut seen_dot = false;
    let mut digits = 0;

    for (k, &c) in s.iter().enumerate() {
        match c {
            b'-' if k == 0 => neg = true,
            b',' | b' ' if !seen_dot => {}
            b'.' if !seen_dot => seen_dot = true,
            b'0'..=b'9' => {
                let d = (c - b'0') as i128;
                digits += 1;
                if !seen_dot {
                    int = int * 10 + d;
                    if int > i64::MAX as i128 {
                        return overflow();
                    }
                } else if nd < FRAC_DIGITS {
                    frac = frac * 10 + d;
                    nd += 1;
                } else if nd == FRAC_DIGITS {
                    round_up = d >= 5;
                    nd += 1;
                }
            }
            _ => {
                set_err(ERR_BAD_DIGIT);
                return 0;
            }
        }
    }
    if digits == 0 {
        set_err(ERR_BAD_DIGIT);
        return 0;
    }

    let nd = nd.min(FRAC_DIGITS);
    let mag = int * SCALE as i128 + frac * POW10[FRAC_DIGITS - nd] as i128 + round_up as i128;
    narrow(if neg { -mag } else { mag })
}
