// `clippy` is a code linting tool for improving code quality by catching
// common mistakes or strange code patterns. If the `clippy` feature is
// provided, it is enabled and all compiler warnings are prohibited.
#![cfg_attr(feature = "clippy", deny(warnings))]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", allow(const_static_lifetime))]
#![cfg_attr(feature = "clippy", allow(inline_always))]
#![cfg_attr(feature = "clippy", allow(many_single_char_names))]
#![cfg_attr(feature = "clippy", allow(or_fun_call))]
#![cfg_attr(feature = "clippy", allow(too_many_arguments))]
#![cfg_attr(feature = "clippy", allow(type_complexity))]
#![cfg_attr(feature = "clippy", allow(unreadable_literal))]
#![cfg_attr(feature = "clippy", allow(wrong_self_convention))]

extern crate pairing;
extern crate bellman;
extern crate blake2_rfc;
extern crate digest;
extern crate rand;

extern crate byteorder;

#[cfg(test)]
#[macro_use]
extern crate hex_literal;

pub mod jubjub;
pub mod circuit;
pub mod group_hash;
pub mod pedersen_hash;
pub mod primitives;

// BLAKE2s invocation personalizations
/// BLAKE2s Personalization for CRH^ivk = BLAKE2s(ak | rk)
const CRH_IVK_PERSONALIZATION: &'static [u8; 8] = b"Zcashivk";
/// BLAKE2s Personalization for PRF^nr = BLAKE2s(rk | cm + position)
const PRF_NR_PERSONALIZATION: &'static [u8; 8]  = b"WhatTheH";

// Group hash personalizations
/// BLAKE2s Personalization for Pedersen hash generators.
const PEDERSEN_HASH_GENERATORS_PERSONALIZATION: &'static [u8; 8] = b"PEDERSEN";
/// BLAKE2s Personalization for the proof generation key base point
const PROOF_GENERATION_KEY_BASE_GENERATOR_PERSONALIZATION: &'static [u8; 8] = b"12345678";
/// BLAKE2s Personalization for the note commitment randomness generator
const NOTE_COMMITMENT_RANDOMNESS_GENERATOR_PERSONALIZATION: &'static [u8; 8] = b"abcdefgh";
/// BLAKE2s Personalization for the nullifier position generator (for PRF^nr)
const NULLIFIER_POSITION_IN_TREE_GENERATOR_PERSONALIZATION: &'static [u8; 8] = b"nfnfnfnf";
/// BLAKE2s Personalization for the value commitment generator for the value
const VALUE_COMMITMENT_VALUE_GENERATOR_PERSONALIZATION: &'static [u8; 8]     = b"45u8gh45";
/// BLAKE2s Personalization for the value commitment randomness generator
const VALUE_COMMITMENT_RANDOMNESS_GENERATOR_PERSONALIZATION: &'static [u8; 8] = b"11111111";
/// BLAKE2s Personalization for the spending key base point
const SPENDING_KEY_GENERATOR_PERSONALIZATION: &'static [u8; 8] = b"sksksksk";
