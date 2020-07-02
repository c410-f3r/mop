//! MOP (*M*any *OP*timizations)

#![cfg_attr(not(feature = "std"), no_std)]

pub extern crate mop_blocks as blocks;
pub extern crate mop_facades as facades;
#[cfg(feature = "with-mop-solvers")]
pub extern crate mop_solvers as solvers;
