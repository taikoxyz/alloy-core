use crate::U160;

use super::{Address, FixedBytes};

/// Default chain ID for Gwyneth, which is 1.
pub const DEFAULT_CHAIN_ID: u64 = 1;

/// Wrap a fixed-size byte array in a newtype, delegating all methods to the
/// underlying [`crate::FixedBytes`].
///
/// This functionally creates a new named `FixedBytes` that cannot be
/// type-confused for another named `FixedBytes`.
///
/// **NOTE:** This macro currently requires:
/// - `#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]` at the top level of the crate.
/// - The `derive_more` crate in scope.
///
/// # Examples
///
/// ```
/// use alloy_primitives::wrap_fixed_bytes;
///
/// // These hashes are the same length, and have the same functionality, but
/// // are distinct types
/// wrap_fixed_bytes!(pub struct KeccakOutput<32>;);
/// wrap_fixed_bytes!(pub struct MerkleTreeItem<32>;);
/// ```
#[macro_export]
macro_rules! wrap_fixed_bytes_with_chain_id {
    (
        extra_derives: [$($extra_derives:path),* $(,)?],
        $(#[$attrs:meta])*
        $vis:vis struct $name:ident<$n:literal>;
    ) => {
        $(#[$attrs])*
        #[allow(clippy::derived_hash_with_manual_eq)]
        #[derive(
            Clone,
            Copy,
            Default,
            Eq,
            $crate::private::derive_more::AsMut,
            $crate::private::derive_more::AsRef,
            $crate::private::derive_more::BitAnd,
            $crate::private::derive_more::BitAndAssign,
            $crate::private::derive_more::BitOr,
            $crate::private::derive_more::BitOrAssign,
            $crate::private::derive_more::BitXor,
            $crate::private::derive_more::BitXorAssign,
            $crate::private::derive_more::Not,
            $crate::private::derive_more::Deref,
            $crate::private::derive_more::DerefMut,
            $crate::private::derive_more::From,
            $crate::private::derive_more::Index,
            $crate::private::derive_more::IndexMut,
            $crate::private::derive_more::Into,
            $crate::private::derive_more::IntoIterator,
            $crate::private::derive_more::LowerHex,
            $crate::private::derive_more::UpperHex,
            $(
                $extra_derives,
            )*
        )]
        #[lower_hex("{}", _0)] #[upper_hex("{}", _0)]
        $vis struct $name(#[index] #[index_mut] #[deref] #[deref_mut] #[into_iterator(owned, ref, ref_mut)] pub $crate::FixedBytes<$n>, pub u64);

        impl $crate::private::From<[u8; $n]> for $name {
            #[inline]
            fn from(value: [u8; $n]) -> Self {
                Self($crate::FixedBytes(value), $crate::DEFAULT_CHAIN_ID)
            }
        }

        impl $crate::private::PartialEq for $name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl $crate::private::core::hash::Hash for $name {
            fn hash<H: $crate::private::core::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        impl $crate::private::Ord for $name {
            #[inline]
            fn cmp(&self, other: &Self) -> $crate::private::core::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl $crate::private::PartialOrd for $name {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<$crate::private::core::cmp::Ordering> {
                Some(self.0.cmp(&other.0))
            }
        }

        impl $crate::private::core::str::FromStr for $name {
            type Err = $crate::hex::FromHexError;
            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $crate::FixedBytes::<$n>::from_str(s).map(|bytes| Self(bytes, $crate::DEFAULT_CHAIN_ID))
            }
        }

        impl $crate::private::From<$name> for [u8; $n] {
            #[inline]
            fn from(value: $name) -> Self {
                value.0 .0
            }
        }

        impl $crate::private::From<&[u8; $n]> for $name {
            #[inline]
            fn from(value: &[u8; $n]) -> Self {
                Self($crate::FixedBytes(*value), $crate::DEFAULT_CHAIN_ID)
            }
        }

        impl $crate::private::From<&mut [u8; $n]> for $name {
            #[inline]
            fn from(value: &mut [u8; $n]) -> Self {
                Self($crate::FixedBytes(*value), $crate::DEFAULT_CHAIN_ID)
            }
        }


        impl $crate::private::From<(&[u8; $n], u64)> for $name {
            #[inline]
            fn from((value, chain_id): (&[u8; $n], u64)) -> Self {
                Self($crate::FixedBytes(*value), chain_id)
            }
        }

        impl $crate::private::From<(&mut [u8; $n], u64)> for $name {
            #[inline]
            fn from((value, chain_id): (&mut [u8; $n], u64)) -> Self {
                Self($crate::FixedBytes(*value), chain_id)
            }
        }

        impl $crate::private::TryFrom<&[u8]> for $name {
            type Error = $crate::private::core::array::TryFromSliceError;

            #[inline]
            fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
                // SAFETY: `$name` is `repr(transparent)` for `FixedBytes<$n>`
                // and consequently `[u8; $n]`
                <&[u8; $n] as $crate::private::TryFrom<&[u8]>>::try_from(slice)
                    .map(|array_ref| Self(unsafe { $crate::private::core::mem::transmute::<&[u8; $n], &$crate::FixedBytes<$n>>(array_ref).clone() }, $crate::DEFAULT_CHAIN_ID))
            }
        }

        impl $crate::private::TryFrom<&mut [u8]> for $name {
            type Error = $crate::private::core::array::TryFromSliceError;

            #[inline]
            fn try_from(slice: &mut [u8]) -> Result<Self, Self::Error> {
                <Self as $crate::private::TryFrom<&[u8]>>::try_from(&*slice)
            }
        }

        impl $crate::private::TryFrom<(&[u8], u64)> for $name {
            type Error = $crate::private::core::array::TryFromSliceError;

            #[inline]
            fn try_from((slice, chain_id): (&[u8], u64)) -> Result<Self, Self::Error> {
                // SAFETY: `$name` is `repr(transparent)` for `FixedBytes<$n>`
                // and consequently `[u8; $n]`
                <&[u8; $n] as $crate::private::TryFrom<&[u8]>>::try_from(slice)
                    .map(|array_ref| Self(unsafe { $crate::private::core::mem::transmute::<&[u8; $n], &$crate::FixedBytes<$n>>(array_ref).clone() }, chain_id))
            }
        }

        impl $crate::private::TryFrom<(&mut [u8], u64)> for $name {
            type Error = $crate::private::core::array::TryFromSliceError;

            #[inline]
            fn try_from((slice, chain_id): (&mut [u8], u64)) -> Result<Self, Self::Error> {
                <Self as $crate::private::TryFrom<(&[u8], u64)>>::try_from((&*slice, chain_id))
            }
        }

        impl $crate::private::AsRef<[u8; $n]> for $name {
            #[inline]
            fn as_ref(&self) -> &[u8; $n] {
                &self.0 .0
            }
        }

        impl $crate::private::AsMut<[u8; $n]> for $name {
            #[inline]
            fn as_mut(&mut self) -> &mut [u8; $n] {
                &mut self.0 .0
            }
        }

        impl $crate::private::AsRef<[u8]> for $name {
            #[inline]
            fn as_ref(&self) -> &[u8] {
                &self.0 .0
            }
        }

        impl $crate::private::AsMut<[u8]> for $name {
            #[inline]
            fn as_mut(&mut self) -> &mut [u8] {
                &mut self.0 .0
            }
        }

        impl $crate::private::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                $crate::private::core::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl $crate::private::core::ops::BitAnd<&Self> for $name {
            type Output = Self;

            #[inline]
            fn bitand(self, rhs: &Self) -> Self {
                Self(self.0.bitand(&rhs.0), self.1)
            }
        }

        impl $crate::private::core::ops::BitAndAssign<&Self> for $name {
            #[inline]
            fn bitand_assign(&mut self, rhs: &Self) {
                self.0.bitand_assign(&rhs.0)
            }
        }

        impl $crate::private::core::ops::BitOr<&Self> for $name {
            type Output = Self;

            #[inline]
            fn bitor(self, rhs: &Self) -> Self {
                Self(self.0.bitor(&rhs.0), self.1)
            }
        }

        impl $crate::private::core::ops::BitOrAssign<&Self> for $name {
            #[inline]
            fn bitor_assign(&mut self, rhs: &Self) {
                self.0.bitor_assign(&rhs.0)
            }
        }

        impl $crate::private::core::ops::BitXor<&Self> for $name {
            type Output = Self;

            #[inline]
            fn bitxor(self, rhs: &Self) -> Self {
                Self(self.0.bitxor(&rhs.0), self.1)
            }
        }

        impl $crate::private::core::ops::BitXorAssign<&Self> for $name {
            #[inline]
            fn bitxor_assign(&mut self, rhs: &Self) {
                self.0.bitxor_assign(&rhs.0)
            }
        }

        $crate::impl_fb_traits!($name, $n);
        $crate::impl_rlp_with_chain_id!($name, $n);
        $crate::impl_serde_with_chain_id!($name);
        $crate::impl_allocative!($name);
        $crate::impl_arbitrary_with_chain_id!($name, $n);
        $crate::impl_rand_with_chain_id!($name);
        $crate::impl_diesel!($name, $n);

        impl $name {
            /// Array of Zero bytes.
            pub const ZERO: Self = Self($crate::FixedBytes::ZERO, $crate::DEFAULT_CHAIN_ID);

            /// Wraps the given byte array in this type.
            #[inline]
            pub const fn new(bytes: [u8; $n]) -> Self {
                Self($crate::FixedBytes(bytes), $crate::DEFAULT_CHAIN_ID)
            }

            /// Creates a new byte array with the last byte set to `x`.
            #[inline]
            pub const fn with_last_byte(x: u8) -> Self {
                Self($crate::FixedBytes::with_last_byte(x), $crate::DEFAULT_CHAIN_ID)
            }

            /// Creates a new byte array where all bytes are set to `byte`.
            #[inline]
            pub const fn repeat_byte(byte: u8) -> Self {
                Self($crate::FixedBytes::repeat_byte(byte), $crate::DEFAULT_CHAIN_ID)
            }

            /// Returns the size of this array in bytes.
            #[inline]
            pub const fn len_bytes() -> usize {
                $n
            }

            $crate::impl_getrandom_with_chain_id!();

            /// Create a new byte array from the given slice `src`.
            ///
            /// For a fallible version, use the `TryFrom<&[u8]>` implementation.
            ///
            /// # Note
            ///
            /// The given bytes are interpreted in big endian order.
            ///
            /// # Panics
            ///
            /// If the length of `src` and the number of bytes in `Self` do not match.
            #[inline]
            #[track_caller]
            pub fn from_slice(src: &[u8]) -> Self {
                match Self::try_from(src) {
                    Ok(x) => x,
                    Err(_) => panic!("cannot convert a slice of length {} to {}", src.len(), stringify!($name)),
                }
            }

            /// Create a new byte array from the given slice `src`, left-padding it
            /// with zeroes if necessary.
            ///
            /// # Note
            ///
            /// The given bytes are interpreted in big endian order.
            ///
            /// # Panics
            ///
            /// Panics if `src.len() > N`.
            #[inline]
            #[track_caller]
            pub fn left_padding_from(value: &[u8]) -> Self {
                Self($crate::FixedBytes::left_padding_from(value), $crate::DEFAULT_CHAIN_ID)
            }

            /// Create a new byte array from the given slice `src`, right-padding it
            /// with zeroes if necessary.
            ///
            /// # Note
            ///
            /// The given bytes are interpreted in big endian order.
            ///
            /// # Panics
            ///
            /// Panics if `src.len() > N`.
            #[inline]
            #[track_caller]
            pub fn right_padding_from(value: &[u8]) -> Self {
                Self($crate::FixedBytes::right_padding_from(value), $crate::DEFAULT_CHAIN_ID)
            }

            /// Returns the inner bytes array.
            #[inline]
            pub const fn into_array(self) -> [u8; $n] {
                self.0 .0
            }

            /// Returns `true` if all bits set in `b` are also set in `self`.
            #[inline]
            pub fn covers(&self, b: &Self) -> bool {
                &(*b & *self) == b
            }

            /// Compile-time equality. NOT constant-time equality.
            pub const fn const_eq(&self, other: &Self) -> bool {
                self.0.const_eq(&other.0)
            }

            /// Computes the bitwise AND of two `FixedBytes`.
            pub const fn bit_and(self, rhs: Self) -> Self {
                Self(self.0.bit_and(rhs.0), self.1)
            }

            /// Computes the bitwise OR of two `FixedBytes`.
            pub const fn bit_or(self, rhs: Self) -> Self {
                Self(self.0.bit_or(rhs.0), self.1)
            }

            /// Computes the bitwise XOR of two `FixedBytes`.
            pub const fn bit_xor(self, rhs: Self) -> Self {
                Self(self.0.bit_xor(rhs.0), self.1)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "serde"))]
macro_rules! impl_serde_with_chain_id {
    ($t:ty) => {};
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "serde")]
macro_rules! impl_serde_with_chain_id {
    ($t:ty) => {
        #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
        impl $crate::private::serde::Serialize for $t {
            #[inline]
            fn serialize<S: $crate::private::serde::Serializer>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                $crate::private::serde::Serialize::serialize(&self.0, serializer)
            }
        }

        #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
        impl<'de> $crate::private::serde::Deserialize<'de> for $t {
            #[inline]
            fn deserialize<D: $crate::private::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                $crate::private::serde::Deserialize::deserialize(deserializer)
                    .map(|bytes| Self(bytes, $crate::DEFAULT_CHAIN_ID))
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "rlp")]
macro_rules! impl_rlp_with_chain_id {
    ($t:ty, $n:literal) => {
        #[cfg_attr(docsrs, doc(cfg(feature = "rlp")))]
        impl $crate::private::alloy_rlp::Decodable for $t {
            #[inline]
            fn decode(buf: &mut &[u8]) -> $crate::private::alloy_rlp::Result<Self> {
                $crate::private::alloy_rlp::Decodable::decode(buf)
                    .map(|addr| Self(addr, $crate::DEFAULT_CHAIN_ID))
            }
        }

        #[cfg_attr(docsrs, doc(cfg(feature = "rlp")))]
        impl $crate::private::alloy_rlp::Encodable for $t {
            #[inline]
            fn length(&self) -> usize {
                $crate::private::alloy_rlp::Encodable::length(&self.0)
            }

            #[inline]
            fn encode(&self, out: &mut dyn $crate::private::alloy_rlp::BufMut) {
                $crate::private::alloy_rlp::Encodable::encode(&self.0, out)
            }
        }

        $crate::private::alloy_rlp::impl_max_encoded_len!($t, {
            $n + $crate::private::alloy_rlp::length_of_length($n)
        });
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "getrandom")]
macro_rules! impl_getrandom_with_chain_id {
    () => {
        /// Creates a new fixed byte array with the default cryptographic random number
        /// generator and the default chain ID.
        #[inline]
        #[track_caller]
        #[cfg_attr(docsrs, doc(cfg(feature = "getrandom")))]
        pub fn random() -> Self {
            Self($crate::FixedBytes::random(), $crate::DEFAULT_CHAIN_ID)
        }

        /// Tries to create a new fixed byte array with the default cryptographic random number
        /// generator and the default chain ID.
        #[inline]
        #[cfg_attr(docsrs, doc(cfg(feature = "getrandom")))]
        pub fn try_random() -> $crate::private::Result<Self, $crate::private::getrandom::Error> {
            $crate::FixedBytes::try_random().map(|fb| Self(fb, $crate::DEFAULT_CHAIN_ID))
        }

        /// Fills this fixed byte array with the default cryptographic random number generator.
        #[inline]
        #[track_caller]
        #[cfg_attr(docsrs, doc(cfg(feature = "getrandom")))]
        pub fn randomize(&mut self) {
            self.0.randomize();
        }

        /// Tries to fill this fixed byte array with the default cryptographic random number
        /// generator.
        #[inline]
        #[cfg_attr(docsrs, doc(cfg(feature = "getrandom")))]
        pub fn try_randomize(
            &mut self,
        ) -> $crate::private::Result<(), $crate::private::getrandom::Error> {
            self.0.try_randomize()
        }
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "getrandom"))]
macro_rules! impl_getrandom_with_chain_id {
    () => {};
}
// Fixed implementations based on the original macros

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "arbitrary"))]
macro_rules! impl_arbitrary_with_chain_id {
    ($t:ty, $n:literal) => {};
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "arbitrary")]
macro_rules! impl_arbitrary_with_chain_id {
    ($t:ty, $n:literal) => {
        // Fix 1: Add explicit lifetime parameter - based on original impl_arbitrary
        #[cfg_attr(docsrs, doc(cfg(feature = "arbitrary")))]
        impl<'a> $crate::private::arbitrary::Arbitrary<'a> for $t {
            #[inline]
            fn arbitrary(u: &mut $crate::private::arbitrary::Unstructured<'a>) -> $crate::private::arbitrary::Result<Self> {
                <$crate::FixedBytes<$n> as $crate::private::arbitrary::Arbitrary>::arbitrary(u)
                    .map(|bytes| Self(bytes, $crate::DEFAULT_CHAIN_ID))
            }

            #[inline]
            fn arbitrary_take_rest(u: $crate::private::arbitrary::Unstructured<'a>) -> $crate::private::arbitrary::Result<Self> {
                <$crate::FixedBytes<$n> as $crate::private::arbitrary::Arbitrary>::arbitrary_take_rest(u)
                    .map(|bytes| Self(bytes, $crate::DEFAULT_CHAIN_ID))
            }

            #[inline]
            fn size_hint(depth: usize) -> (usize, Option<usize>) {
                <$crate::FixedBytes<$n> as $crate::private::arbitrary::Arbitrary>::size_hint(depth)
            }
        }

        // For proptest support - based on original impl_arbitrary
        #[cfg_attr(docsrs, doc(cfg(feature = "arbitrary")))]
        impl $crate::private::proptest::arbitrary::Arbitrary for $t {
            type Parameters = <$crate::FixedBytes<$n> as $crate::private::proptest::arbitrary::Arbitrary>::Parameters;
            type Strategy = $crate::private::proptest::strategy::Map<
                <$crate::FixedBytes<$n> as $crate::private::proptest::arbitrary::Arbitrary>::Strategy,
                fn($crate::FixedBytes<$n>) -> Self,
            >;

            #[inline]
            fn arbitrary() -> Self::Strategy {
                use $crate::private::proptest::strategy::Strategy;
                <$crate::FixedBytes<$n> as $crate::private::proptest::arbitrary::Arbitrary>::arbitrary()
                    .prop_map(|bytes| Self(bytes, $crate::DEFAULT_CHAIN_ID))
            }

            #[inline]
            fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                use $crate::private::proptest::strategy::Strategy;
                <$crate::FixedBytes<$n> as $crate::private::proptest::arbitrary::Arbitrary>::arbitrary_with(args)
                    .prop_map(|bytes| Self(bytes, $crate::DEFAULT_CHAIN_ID))
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "rand"))]
macro_rules! impl_rand_with_chain_id {
    ($t:ty) => {};
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "rand")]
macro_rules! impl_rand_with_chain_id {
    ($t:ty) => {
        // Fix 2: Use the exact same pattern as the original impl_rand macro
        #[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
        impl $crate::private::rand::distr::Distribution<$t>
            for $crate::private::rand::distr::StandardUniform
        {
            #[inline]
            fn sample<R: $crate::private::rand::Rng + ?Sized>(&self, rng: &mut R) -> $t {
                <$t>::random_with(rng)
            }
        }

        impl $t {
            /// Creates a new fixed byte array with the given random number generator.
            #[inline]
            #[doc(alias = "random_using")]
            #[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
            pub fn random_with<R: $crate::private::rand::RngCore + ?Sized>(rng: &mut R) -> Self {
                Self($crate::FixedBytes::random_with(rng), $crate::DEFAULT_CHAIN_ID)
            }

            /// Tries to create a new fixed byte array with the given random number generator.
            #[inline]
            #[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
            pub fn try_random_with<R: $crate::private::rand::TryRngCore + ?Sized>(
                rng: &mut R,
            ) -> $crate::private::Result<Self, R::Error> {
                $crate::FixedBytes::try_random_with(rng)
                    .map(|bytes| Self(bytes, $crate::DEFAULT_CHAIN_ID))
            }

            /// Fills this fixed byte array with the given random number generator.
            #[inline]
            #[doc(alias = "randomize_using")]
            #[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
            pub fn randomize_with<R: $crate::private::rand::RngCore + ?Sized>(
                &mut self,
                rng: &mut R,
            ) {
                self.0.randomize_with(rng);
            }

            /// Tries to fill this fixed byte array with the given random number generator.
            #[inline]
            #[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
            pub fn try_randomize_with<R: $crate::private::rand::TryRngCore + ?Sized>(
                &mut self,
                rng: &mut R,
            ) -> $crate::private::Result<(), R::Error> {
                self.0.try_randomize_with(rng)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "rlp"))]
macro_rules! impl_rlp_with_chain_id {
    ($t:ty, $n:literal) => {};
}

impl From<(U160, u64)> for Address {
    #[inline]
    fn from((value, chain_id): (U160, u64)) -> Self {
        Self(FixedBytes(value.to_be_bytes()), chain_id)
    }
}

impl Address {
    /// Creates an Ethereum address from an EVM word's upper 20 bytes and chain id
    /// (`word[12..]`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use alloy_primitives::{address, b256, Address};
    /// let word = b256!("0x000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa96045");
    /// assert_eq!(Address::from_word(word), address!("0xd8da6bf26964af9d7eed9e03e53415d37aa96045"));
    /// ```
    #[inline]
    #[must_use]
    pub fn from_word_and_chain_id(word: FixedBytes<32>, chain_id: u64) -> Self {
        Self(FixedBytes(word[12..].try_into().unwrap()), chain_id)
    }

    /// Changes the chain id of this address.
    #[inline]
    pub fn set_chain_id(&mut self, chain_id: u64) {
        self.1 = chain_id;
    }

    /// Gets the chain id of this address.
    #[inline]
    pub fn chain_id(&self) -> u64 {
        self.1
    }

    /// Returns a new address with the specified chain id.
    pub fn on_chain(self, chain_id: u64) -> Address {
        Address(self.0, chain_id)
    }
}
