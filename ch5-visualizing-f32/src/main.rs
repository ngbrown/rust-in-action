const BIAS_F32: i32 = 127;
const BIAS_F64: i64 = 1023;
const RADIX_F32: f32 = 2.0;
const RADIX_F64: f64 = 2.0;

fn main() {
    let n: f64 = (1.0 / 100.0);
    let (sign, exp, frac) = to_parts_f64(n);
    let (sign_, exp_, mant) = decode_f64(sign, exp, frac);
    let n_ = from_parts_f64(sign_, exp_, mant);

    println!("{:.20} -> {:.16}", n, n_);
    println!("field     | as bits                                              | as real number");
    println!(
        "sign      |                                                    {:01b} | {}",
        sign, sign_
    );
    println!(
        "exponent  |                                          {:011b} | {}",
        exp, exp_
    );
    println!("mantissa  | {:052b} | {:.20}", frac, mant);

    let n: f32 = n as f32;
    let (sign, exp, frac) = to_parts_f32(n);
    let (sign_, exp_, mant) = decode_f32(sign, exp, frac);
    let n_ = from_parts_f32(sign_, exp_, mant);

    println!("{:.20} -> {:.7}", n, n_);
    println!("field     | as bits                 | as real number");
    println!("sign      |                       {:01b} | {}", sign, sign_);
    println!("exponent  |                {:08b} | {}", exp, exp_);
    println!("mantissa  | {:023b} | {:.20}", frac, mant);
}

fn to_parts_f32(n: f32) -> (u32, u32, u32) {
    let bits: u32 = n.to_bits();

    let sign = (bits >> 31) & 0x01;
    let exponent = (bits >> 23) & 0xff;
    let fraction = bits & 0x7f_ffff;

    (sign, exponent, fraction)
}

fn decode_f32(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let signed_1 = (-1_f32).powf(sign as f32);

    let exponent_ = (exponent as i32) - BIAS_F32;
    let exponent_ = RADIX_F32.powf(exponent_ as f32);

    let mut mantissa: f32 = match exponent {
        0xff => 0.0,
        _ => 1.0,
    };

    // simpler would be to (optionally) 'or' (|) with the leading 1 and divide by 0x80_0000 (8_388_608)
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    (signed_1, exponent_, mantissa)
}

fn from_parts_f32(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}

fn to_parts_f64(n: f64) -> (u64, u64, u64) {
    let bits: u64 = n.to_bits();

    let sign = (bits >> 63) & 0x01;
    let exponent = (bits >> 52) & 0x7_ff;
    let fraction = bits & 0xf_ffff_ffff_ffff;

    (sign, exponent, fraction)
}

fn decode_f64(sign: u64, exponent: u64, fraction: u64) -> (f64, f64, f64) {
    let signed_1 = (-1_f64).powf(sign as f64);

    let exponent_ = (exponent as i64) - BIAS_F64;
    let exponent_ = RADIX_F64.powf(exponent_ as f64);

    let mut mantissa: f64 = match exponent {
        0xff => 0.0,
        _ => 1.0,
    };

    // simpler would be to (optionally) 'or' (|) with the leading 1 and divide by 0x80_0000 (8_388_608)
    for i in 0..52 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f64;
            let weight = 2_f64.powf(i_ - 52.0);
            mantissa += weight;
        }
    }

    (signed_1, exponent_, mantissa)
}

fn from_parts_f64(sign: f64, exponent: f64, mantissa: f64) -> f64 {
    sign * exponent * mantissa
}
