use lazy_static::lazy_static;
use std::env;

lazy_static! {
    /// Interval in seconds for cleanup of expired values from the caches
    pub static ref CACHE_CLEANUP_INTERVAL: u64 = env::var("CACHE_CLEANUP_INTERVAL")
        .expect("missing CACHE_CLEANUP_INTERVAL env var")
        .parse::<u64>()
        .expect("CACHE_CLEANUP_INTERVAL must be a number");

    /// Pictures directory
    pub static ref PICTURES_DIR: String = env::var("PICTURES_DIR")
        .expect("missing PICTURES_DIR env var");

    /// Pictures maximum size in mebibytes
    pub static ref PICTURES_SIZEMAX: usize = env::var("PICTURES_SIZEMAX")
        .expect("missing PICTURES_SIZEMAX env var")
        .parse::<usize>()
        .expect("PICTURES_SIZEMAX must be a number");

    /// Superposables directory
    pub static ref SUPERPOSABLES_DIR: String = env::var("SUPERPOSABLES_DIR")
        .expect("missing SUPERPOSABLES_DIR env var");

    /// Superposables width and height in pixels
    pub static ref SUPERPOSABLES_SIDE: u32 = env::var("SUPERPOSABLES_SIDE")
        .expect("missing SUPERPOSABLES_SIDE env var")
        .parse::<u32>()
        .expect("SUPERPOSABLES_SIDE must be a number");
}
