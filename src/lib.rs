#![no_std]
#![allow(dead_code)] // Added to remove warnings for unused functions

mod base32;
mod contract;
mod did_trait;
mod did_uri;
mod service;
mod storage;
mod verification_method;
mod error;

#[cfg(test)]
mod test;
