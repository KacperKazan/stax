//! stax library interface
//!
//! This module exposes internal functionality for integration testing.
//! The main binary is in main.rs.

#![allow(dead_code)]
#![allow(unused_imports)]

// Internal modules needed by github module
mod cache;
mod config;
mod engine;
mod git;
mod remote;

// Expose github module for tests
pub mod github;
