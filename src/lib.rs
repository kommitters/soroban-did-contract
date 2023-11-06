#![no_std]
#![allow(dead_code)] // Added to remove warnings for unused functions

mod did_trait;
mod service;
mod storage;
mod test;
mod verification_method;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
