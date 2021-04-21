use num::float::PrimitiveFloat;
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Clone, Copy)]
pub struct NiceFloat<T: PrimitiveFloat>(pub T);

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum FloatType {
    NegativeInfinity,
    NegativeFinite,
    NegativeZero,
    NaN,
    PositiveZero,
    PositiveFinite,
    PositiveInfinity,
}

impl<T: PrimitiveFloat> NiceFloat<T> {
    #[inline]
    pub fn unwrap(self) -> T {
        self.0
    }

    fn float_type(self) -> FloatType {
        if self.0.is_nan() {
            FloatType::NaN
        } else if self.0.is_sign_positive() {
            if self.0 == T::ZERO {
                FloatType::PositiveZero
            } else if self.0.is_finite() {
                FloatType::PositiveFinite
            } else {
                FloatType::PositiveInfinity
            }
        } else if self.0 == T::ZERO {
            FloatType::NegativeZero
        } else if self.0.is_finite() {
            FloatType::NegativeFinite
        } else {
            FloatType::NegativeInfinity
        }
    }
}

impl<T: PrimitiveFloat> PartialEq<NiceFloat<T>> for NiceFloat<T> {
    #[inline]
    fn eq(&self, other: &NiceFloat<T>) -> bool {
        if self.0 == T::ZERO {
            other.0 == T::ZERO && self.0.is_sign_positive() == other.0.is_sign_positive()
        } else {
            self.0 == other.0 || self.0.is_nan() && other.0.is_nan()
        }
    }
}

impl<T: PrimitiveFloat> Eq for NiceFloat<T> {}

impl<T: PrimitiveFloat> Hash for NiceFloat<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.0.is_nan() {
            "NaN".hash(state)
        } else {
            self.0.to_bits().hash(state)
        }
    }
}

impl<T: PrimitiveFloat> Ord for NiceFloat<T> {
    fn cmp(&self, other: &NiceFloat<T>) -> Ordering {
        let self_type = self.float_type();
        let other_type = other.float_type();
        self_type.cmp(&other_type).then_with(|| {
            if self_type == FloatType::PositiveFinite || self_type == FloatType::NegativeFinite {
                self.0.partial_cmp(&other.0).unwrap()
            } else {
                Ordering::Equal
            }
        })
    }
}

impl<T: PrimitiveFloat> PartialOrd<NiceFloat<T>> for NiceFloat<T> {
    #[inline]
    fn partial_cmp(&self, other: &NiceFloat<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub trait FmtRyuString: Copy {
    fn fmt_ryu_string(self, f: &mut Formatter<'_>) -> fmt::Result;
}

macro_rules! impl_fmt_ryu_string {
    ($f: ident) => {
        impl FmtRyuString for $f {
            #[inline]
            fn fmt_ryu_string(self, f: &mut Formatter<'_>) -> fmt::Result {
                let mut buffer = ryu::Buffer::new();
                let printed = buffer.format_finite(self);
                // Convert e.g. "1e100" to "1.0e100".
                // `printed` is ASCII, so we can manipulate bytes rather than chars.
                let mut e_index = None;
                let mut found_dot = false;
                for (i, &b) in printed.as_bytes().iter().enumerate() {
                    match b {
                        b'.' => {
                            found_dot = true;
                            break;
                        }
                        b'e' => {
                            e_index = Some(i);
                            break;
                        }
                        _ => {}
                    }
                }
                if !found_dot {
                    if let Some(e_index) = e_index {
                        let mut out_bytes = vec![0; printed.len() + 2];
                        let (in_bytes_lo, in_bytes_hi) = printed.as_bytes().split_at(e_index);
                        let (out_bytes_lo, out_bytes_hi) = out_bytes.split_at_mut(e_index);
                        out_bytes_lo.copy_from_slice(in_bytes_lo);
                        out_bytes_hi[0] = b'.';
                        out_bytes_hi[1] = b'0';
                        out_bytes_hi[2..].copy_from_slice(in_bytes_hi);
                        f.write_str(std::str::from_utf8(&out_bytes).unwrap())
                    } else {
                        panic!("Unexpected Ryu string: {}", printed);
                    }
                } else {
                    f.write_str(printed)
                }
            }
        }
    };
}
impl_fmt_ryu_string!(f32);
impl_fmt_ryu_string!(f64);

impl<T: PrimitiveFloat> Display for NiceFloat<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.0.is_nan() {
            f.write_str("NaN")
        } else if self.0.is_infinite() {
            if self.0.is_sign_positive() {
                f.write_str("Infinity")
            } else {
                f.write_str("-Infinity")
            }
        } else {
            self.0.fmt_ryu_string(f)
        }
    }
}

impl<T: PrimitiveFloat> Debug for NiceFloat<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl<T: PrimitiveFloat> FromStr for NiceFloat<T> {
    type Err = <T as FromStr>::Err;

    #[inline]
    fn from_str(src: &str) -> Result<NiceFloat<T>, <T as FromStr>::Err> {
        match src {
            "NaN" => Ok(T::NAN),
            "Infinity" => Ok(T::POSITIVE_INFINITY),
            "-Infinity" => Ok(T::NEGATIVE_INFINITY),
            src => T::from_str(src),
        }
        .map(NiceFloat)
    }
}
