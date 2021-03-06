use ark_std::{
    cmp::{Ord, Ordering, PartialOrd},
    fmt::{Display, Formatter, Result as FmtResult},
    io::{Read, Result as IoResult, Write},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};
use num_traits::{One, Zero};
use mpc_trait::MpcWire;

use crate::{
    biginteger::{
        arithmetic as fa, BigInteger as _BigInteger, BigInteger256, BigInteger320, BigInteger384,
        BigInteger64, BigInteger768, BigInteger832,
    },
    bytes::{FromBytes, ToBytes},
    fields::{FftField, Field, FpParameters, LegendreSymbol, PrimeField, SquareRootField},
};
use ark_serialize::*;

impl_Fp!(Fp64, Fp64Parameters, BigInteger64, BigInteger64, 1);
impl_Fp!(Fp256, Fp256Parameters, BigInteger256, BigInteger256, 4);
impl_Fp!(Fp320, Fp320Parameters, BigInteger320, BigInteger320, 5);
impl_Fp!(Fp384, Fp384Parameters, BigInteger384, BigInteger384, 6);
impl_Fp!(Fp768, Fp768Parameters, BigInteger768, BigInteger768, 12);
impl_Fp!(Fp832, Fp832Parameters, BigInteger832, BigInteger832, 13);

pub mod fp2;
pub use self::fp2::*;

pub mod fp3;
pub use self::fp3::*;

pub mod fp4;
pub use self::fp4::*;

pub mod fp6_2over3;

pub mod fp6_3over2;
pub use self::fp6_3over2::*;

pub mod fp12_2over3over2;
pub use self::fp12_2over3over2::*;

pub mod quadratic_extension;
pub use quadratic_extension::*;

pub mod cubic_extension;
pub use cubic_extension::*;
