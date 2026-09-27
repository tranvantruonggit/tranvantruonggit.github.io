//! Integer core of the programmer calculator at /tools/hexcalc.
//! The FIN (decimal, fixed-point) mode lives in `fin`.
//!
//! All values are carried as u64 (JS sees them as BigInt through the i64 ABI).
//! `w` is the active word size (8/16/32/64); every result is masked to it and
//! `signed != 0` selects the two's complement interpretation of the same bits.

#![no_std]

use core::ptr::addr_of_mut;

mod fin;

#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

pub const OP_ADD: u32 = 0;
pub const OP_SUB: u32 = 1;
pub const OP_MUL: u32 = 2;
pub const OP_DIV: u32 = 3;
pub const OP_MOD: u32 = 4;
pub const OP_AND: u32 = 5;
pub const OP_OR: u32 = 6;
pub const OP_XOR: u32 = 7;
pub const OP_SHL: u32 = 8;
pub const OP_SHR: u32 = 9;
pub const OP_NOT: u32 = 10; // unary, operates on `a`
pub const OP_NEG: u32 = 11; // unary, operates on `a`

pub const ERR_OK: u32 = 0;
pub const ERR_DIV0: u32 = 1;
pub const ERR_BAD_OP: u32 = 2;
pub const ERR_OVERFLOW: u32 = 3;
pub const ERR_BAD_DIGIT: u32 = 4;

/// 64 binary digits + sign fits comfortably.
pub(crate) const BUF_LEN: usize = 72;

static mut ERR: u32 = ERR_OK;
pub(crate) static mut BUF: [u8; BUF_LEN] = [0; BUF_LEN];

#[inline(always)]
fn width(w: u32) -> u32 {
    match w {
        1..=64 => w,
        _ => 64,
    }
}

#[inline(always)]
fn mask(w: u32) -> u64 {
    let w = width(w);
    if w == 64 {
        u64::MAX
    } else {
        (1u64 << w) - 1
    }
}

/// Sign-extend the low `w` bits of `v`.
#[inline(always)]
fn sext(v: u64, w: u32) -> i64 {
    let s = 64 - width(w);
    ((v << s) as i64) >> s
}

#[inline(always)]
pub(crate) fn set_err(e: u32) {
    unsafe { ERR = e };
}

/// Returns and clears the last error code.
#[no_mangle]
pub extern "C" fn hc_err() -> u32 {
    unsafe {
        let e = ERR;
        ERR = ERR_OK;
        e
    }
}

/// Address of the formatting buffer in linear memory.
#[no_mangle]
pub extern "C" fn hc_buf() -> *const u8 {
    addr_of_mut!(BUF) as *const u8
}

#[no_mangle]
pub extern "C" fn hc_mask(v: u64, w: u32) -> u64 {
    v & mask(w)
}

/// Decimal entry in signed mode works on the magnitude so that "-12" + "3" = "-123".
#[inline(always)]
fn signed_entry(radix: u32, signed: u32) -> bool {
    signed != 0 && radix == 10
}

/// Appends digit `d` to `v`. On overflow or invalid digit, `v` is returned unchanged
/// and the error is latched.
#[no_mangle]
pub extern "C" fn hc_push_digit(v: u64, d: u32, radix: u32, w: u32, signed: u32) -> u64 {
    if d >= radix || !(2..=16).contains(&radix) {
        set_err(ERR_BAD_DIGIT);
        return v;
    }
    let m = mask(w);
    let v = v & m;
    let (r, d) = (radix as u64, d as u64);

    if signed_entry(radix, signed) {
        let s = sext(v, w);
        let neg = s < 0;
        let limit = if neg { (m >> 1) + 1 } else { m >> 1 };
        let mag = s.unsigned_abs();
        match mag.checked_mul(r).and_then(|x| x.checked_add(d)) {
            Some(n) if n <= limit => {
                if neg {
                    n.wrapping_neg() & m
                } else {
                    n
                }
            }
            _ => {
                set_err(ERR_OVERFLOW);
                v
            }
        }
    } else {
        match v.checked_mul(r).and_then(|x| x.checked_add(d)) {
            Some(n) if n <= m => n,
            _ => {
                set_err(ERR_OVERFLOW);
                v
            }
        }
    }
}

/// Removes the last digit of `v` as displayed in `radix`.
#[no_mangle]
pub extern "C" fn hc_pop_digit(v: u64, radix: u32, w: u32, signed: u32) -> u64 {
    if !(2..=16).contains(&radix) {
        set_err(ERR_BAD_DIGIT);
        return v;
    }
    let m = mask(w);
    let v = v & m;
    if signed_entry(radix, signed) {
        let s = sext(v, w);
        let q = s.unsigned_abs() / radix as u64;
        if s < 0 {
            q.wrapping_neg() & m
        } else {
            q
        }
    } else {
        v / radix as u64
    }
}

/// Evaluates `a op b` at word size `w`. Unary ops ignore `b`.
/// On error (division by zero, unknown op) returns 0 and latches the error.
#[no_mangle]
pub extern "C" fn hc_calc(op: u32, a: u64, b: u64, w: u32, signed: u32) -> u64 {
    let w = width(w);
    let m = mask(w);
    let (a, b) = (a & m, b & m);
    let (sa, sb) = (sext(a, w), sext(b, w));
    let sgn = signed != 0;

    let r = match op {
        OP_ADD => a.wrapping_add(b),
        OP_SUB => a.wrapping_sub(b),
        OP_MUL => a.wrapping_mul(b),
        OP_DIV | OP_MOD => {
            if b == 0 {
                set_err(ERR_DIV0);
                return 0;
            }
            match (op, sgn) {
                (OP_DIV, false) => a / b,
                (OP_DIV, true) => sa.wrapping_div(sb) as u64,
                (_, false) => a % b,
                (_, true) => sa.wrapping_rem(sb) as u64,
            }
        }
        OP_AND => a & b,
        OP_OR => a | b,
        OP_XOR => a ^ b,
        OP_SHL => {
            if b >= w as u64 {
                0
            } else {
                a << b
            }
        }
        OP_SHR => {
            if sgn {
                // Arithmetic shift: saturates to all sign bits.
                (sa >> b.min(63)) as u64
            } else if b >= w as u64 {
                0
            } else {
                a >> b
            }
        }
        OP_NOT => !a,
        OP_NEG => a.wrapping_neg(),
        _ => {
            set_err(ERR_BAD_OP);
            return 0;
        }
    };
    r & m
}

/// Formats `v` in `radix` (2..=16, upper-case) into the shared buffer, returns
/// the length. Decimal honours `signed`; other radices show the raw bits.
#[no_mangle]
pub extern "C" fn hc_format(v: u64, radix: u32, w: u32, signed: u32) -> u32 {
    const DIGITS: &[u8; 16] = b"0123456789ABCDEF";
    let radix = if (2..=16).contains(&radix) { radix } else { 16 } as u64;
    let v = v & mask(w);

    let (neg, mut mag) = if signed != 0 && radix == 10 {
        let s = sext(v, w);
        (s < 0, s.unsigned_abs())
    } else {
        (false, v)
    };

    // Fill from the end, then move to the front.
    let buf = unsafe { &mut *addr_of_mut!(BUF) };
    let mut i = BUF_LEN;
    loop {
        i -= 1;
        buf[i] = DIGITS[(mag % radix) as usize];
        mag /= radix;
        if mag == 0 {
            break;
        }
    }
    if neg {
        i -= 1;
        buf[i] = b'-';
    }
    let len = BUF_LEN - i;
    buf.copy_within(i.., 0);
    len as u32
}
