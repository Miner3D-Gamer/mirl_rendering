//! A library for rendering on [Buffer](mirl_core::render::Buffer), [`ConstBuffer`](mirl_core::render::ConstBuffer), or a custom Buffer
// Const
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(const_destruct)]
// Additions
#![feature(specialization)]
#![feature(portable_simd)]
// Core
#![cfg_attr(feature = "font_support", feature(core_intrinsics))]

/// Other functions/structs like 3D points and Polygons
pub mod extra;

// #[cfg(feature = "image")]
// mod image_support;
// #[cfg(feature = "image")]
// pub use image_support::*;

// ---------------------------------------------------------------------
mod rendering;
pub use rendering::*;

#[cfg(feature = "std")]
mod buffer;
// #[cfg(feature = "std")]
// pub use buffer::*;

/// A const buffer making money on more compile time optimizations and `no_std` support
pub mod const_buffer;
// pub use const_buffer::*;

/// All the traits needed to have a seamless experience
pub mod prelude;
