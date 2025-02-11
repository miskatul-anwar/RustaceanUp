//! # Algom
//!
//! `algom` is a collection of utilities to make performing certain calculations more convenient.
//!
//! ## Modules
//!
//! - [math](algo::math)
//! - [search](algo::search)
//! - [sort](algo::sort)
//!
//! ## Examples
//!
//! ```rust
//! use algom::sort::counting_sort;
//!
//! let mut arr = vec![4, 2, 2, 8, 3, 3, 1, 7];
//! counting_sort(&mut arr);
//! assert_eq!(arr, vec![1, 2, 2, 3, 3, 4, 7, 8]);
//! ```
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! algom = "0.1.0"
//! ```
//!
//! Then, import the crate in your code:
//!
//! ```rust
//! extern crate algom;
//! use algom::*;
//! ```
//!
//! ## License
//!
//! This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

pub mod algo;
