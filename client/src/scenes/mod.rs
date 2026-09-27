pub mod character_create;
pub mod character_select;
pub mod in_game;
pub mod loading;
pub mod login;
pub mod state;

pub use character_create::{CharacterCreatePlugin, CharacterCreationDraft};
pub use character_select::CharacterSelectPlugin;
pub use in_game::{EnteringGameState, InGamePlugin};
pub use loading::{LoadingPlugin, LoadingProgress};
pub use login::{LoginAnimationState, LoginModalState, LoginPlugin};
pub use state::{
    AccountData, CharacterClassInfo, CharacterSummary, CleanupOnExit, SceneEntity, SceneState,
    CHARACTER_CLASSES, COLOR_PALETTE,
};
