use thiserror::Error;

pub mod list_asset_loader;
pub mod map_assset_loader;
pub mod rle_asset_loader;
pub mod rmd_asset_loader;

/// Possible errors that can be produced by [`CustomAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum AssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not load asset: {0}")]
    Core(#[from] core_compat::error::Error),
}
