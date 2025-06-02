#![doc = include_str!("../README.md")]

#[cfg(feature = "dim2")]
pub extern crate salva2d as rapier;
#[cfg(feature = "dim3")]
pub extern crate salva3d as rapier;
#[cfg(feature = "dim2")]
pub extern crate wgparry2d as wgparry;
#[cfg(feature = "dim3")]
pub extern crate wgparry3d as wgparry;

pub mod liquid_world;
