// Decibel -- Quick conversion utilities for decibel values.
// Copyright (c) 2016 Kevin Brothaler and the Decibel project authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Conversion utilities to convert between amplitudes and decibels.
//!
//! See also: [Decibel][1] and [dBFS][2].
//! [1]: https://en.wikipedia.org/wiki/Decibel
//! [2]: https://en.wikipedia.org/wiki/DBFS
//!
//! # Converting amplitude values into decibel values
//!
//! To convert from an amplitude into a decibel value, call `into()` on the
//! `AmplitudeRatio`.
//!
//! When used for normalized amplitudes in the range of 0 to 1, this will give
//! the value in dBFS (decibels relative to full scale).
//!
//! ## Example
//!
//! ```rust
//! extern crate decibel;
//!
//! use decibel::{AmplitudeRatio, DecibelRatio};
//!
//! fn main() {
//!     // An amplitude halfway between 1 and zero should be close to -6 dBFS.
//!     let result: DecibelRatio<_> = AmplitudeRatio(0.5).into();
//!     let expected_decibels = -6.02059991327962;
//!     assert!(result.decibel_value() >= expected_decibels - 0.001
//!          && result.decibel_value() <= expected_decibels + 0.001);
//! }
//! ```
//!
//! # Converting decibel values into amplitude values
//!
//! To convert from a decibel value into an amplitude, call `into()` on the
//! `DecibelRatio`.
//!
//! ## Example
//!
//! Let's say we want to scale our audio by 10dB. To figure out how much we
//! need to scale each sample by, let's convert this into an amplitude ratio:
//!
//! ```rust
//! extern crate decibel;
//!
//! use decibel::{AmplitudeRatio, DecibelRatio};
//!
//! fn main() {
//!     // A +10dB gain should require us to scale each sample by around
//!     // 3.1622776601683795.
//!     let result: AmplitudeRatio<_> = DecibelRatio(10.0).into();
//!     let expected_amplitude = 3.1622776601683795;
//!     assert!(result.amplitude_value() >= expected_amplitude - 0.001
//!          && result.amplitude_value() <= expected_amplitude + 0.001);
//! }
//! ```
//!
//! To scale our audio by 10dB, we need to scale each sample by approximately
//! 3.162 times.

#![warn(missing_docs)]

/// An amplitude value.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AmplitudeRatio<T: Copy>(pub T);

/// A decibel value.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DecibelRatio<T: Copy>(pub T);


macro_rules! impl_from_amplitude_ratio {
    ($T: ty) => {        
        impl From<AmplitudeRatio<$T>> for DecibelRatio<$T> {
            #[inline]
            fn from(amplitude: AmplitudeRatio<$T>) -> DecibelRatio<$T> {
                DecibelRatio(<$T>::log10(amplitude.amplitude_value()) * 20.0)
            }
        }                    
    }
}

impl_from_amplitude_ratio!(f32);
impl_from_amplitude_ratio!(f64);

macro_rules! impl_from_decibel_ratio {
    ($T: ty) => {        
        impl From<DecibelRatio<$T>> for AmplitudeRatio<$T> {
            #[inline]
            fn from(decibels: DecibelRatio<$T>) -> AmplitudeRatio<$T> {
                AmplitudeRatio(<$T>::powf(10.0, decibels.decibel_value() / 20.0))
            }
        }                    
    }
}

impl_from_decibel_ratio!(f32);
impl_from_decibel_ratio!(f64);

impl<T: Copy> AmplitudeRatio<T> {
    /// Returns the wrapped amplitude value.
    #[inline]
    pub fn amplitude_value(&self) -> T {
        self.0
    }
}

impl<T: Copy> DecibelRatio<T> {
    /// Returns the wrapped decibel value.
    #[inline]
    pub fn decibel_value(&self) -> T {
        self.0
    }
}

#[cfg(test)]
mod test {    
    use std::{f32, f64};
    use AmplitudeRatio;
    use DecibelRatio;

    #[test]
    fn test_decibels_to_amplitude_with_different_values_f32() {
        // A dB of 0 should map to an amplitude of 1.0.
        test_decibels_to_amplitude_f32(0.0, 1.0);
        test_decibels_to_amplitude_f64(0.0, 1.0);

        // A dB of around -6dB should be around an amplitude of 0.5.
        test_decibels_to_amplitude_f32(-6.02059991327962, 0.5);
        test_decibels_to_amplitude_f64(-6.02059991327962, 0.5);

        // A dB of around +6dB should be an amplitude of around 2.0.
        test_decibels_to_amplitude_f32(6.02059991327962, 2.0);
        test_decibels_to_amplitude_f64(6.02059991327962, 2.0);

        // +1 or -1 in a 16-bit signed sample should be approximately -90.3dB.
        test_decibels_to_amplitude_f32(-90.30873362169473, 1.0 / 32767.0);
        test_decibels_to_amplitude_f64(-90.30873362169473, 1.0 / 32767.0);

        // A dB of negative infinity should map to an amplitude of zero.
        test_decibels_to_amplitude_f32(f32::NEG_INFINITY, 0.0);
        test_decibels_to_amplitude_f64(f64::NEG_INFINITY, 0.0);
    }

    fn test_decibels_to_amplitude_f32(decibels: f32, expected_amplitude: f32) {
        let result: AmplitudeRatio<_> = DecibelRatio(decibels).into();
        assert!(result.amplitude_value() >= expected_amplitude - 0.001 &&
                result.amplitude_value() <= expected_amplitude + 0.001);
    }

    fn test_decibels_to_amplitude_f64(decibels: f64, expected_amplitude: f64) {
        let result: AmplitudeRatio<_> = DecibelRatio(decibels).into();
        assert!(result.amplitude_value() >= expected_amplitude - 0.001 &&
                result.amplitude_value() <= expected_amplitude + 0.001);
    }

    #[test]
    fn test_amplitude_to_decibels_with_different_values() {
        // An amplitude at the peak should be 0 dBFS.
        test_amplitude_to_decibels_f32(1.0, 0.0);        
        test_amplitude_to_decibels_f64(1.0, 0.0);        

        // An amplitude halfway between 1 and zero should be close to -6 dBFS.
        test_amplitude_to_decibels_f32(0.5, -6.02059991327962);
        test_amplitude_to_decibels_f64(0.5, -6.02059991327962);

        // A double amplitude should be close to +6 dBFS.
        // Note that we can't actually get an amplitude higher than 1 from the
        // hardware if 1 represents the max amplitude possible.
        test_amplitude_to_decibels_f32(2.0, 6.02059991327962);
        test_amplitude_to_decibels_f64(2.0, 6.02059991327962);

        // +1 or -1 in a 16-bit signed sample should be approximately -90.3 dBFS.
        test_amplitude_to_decibels_f32(1.0 / 32767.0, -90.30873362169473);
        test_amplitude_to_decibels_f64(1.0 / 32767.0, -90.30873362169473);

        // 0 is a special case. We should get an infinity.
        test_amplitude_to_decibels_f32(0.0, f32::NEG_INFINITY);
        test_amplitude_to_decibels_f64(0.0, f64::NEG_INFINITY);
    }   

    fn test_amplitude_to_decibels_f32(amplitude: f32, expected_decibels: f32) {
        let result: DecibelRatio<_> = AmplitudeRatio(amplitude).into();
                assert!(result.decibel_value() >= expected_decibels - 0.001 &&
                        result.decibel_value() <= expected_decibels + 0.001);
    }

    fn test_amplitude_to_decibels_f64(amplitude: f64, expected_decibels: f64) {
        let result: DecibelRatio<_> = AmplitudeRatio(amplitude).into();
                assert!(result.decibel_value() >= expected_decibels - 0.001 &&
                        result.decibel_value() <= expected_decibels + 0.001);
    } 
}
