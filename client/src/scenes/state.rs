use bevy::prelude::*;

/// Top-level application and scene states for the client lifecycle.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SceneState {
    #[default]
    Loading,
    Login,
    CharacterSelect,
    CharacterCreate,
    EnteringGame,
    InGame,
}

/// Marker component for entities spawned dynamically in a transient scene.
/// These entities are despawned when exiting their respective scene.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct SceneEntity;

/// Marker component to clean up entities when exiting a specific scene.
#[derive(Component, Debug, Clone, Copy)]
pub struct CleanupOnExit<S: States>(pub S);

/// Character summary representation stored in local account state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterSummary {
    pub name: String,
    pub class: u32,
    pub level: u32,
    pub hair_color: usize,
    pub body_color: usize,
    pub selected: bool,
}

/// Character class metadata (lore descriptions, archetypes, and class names).
pub struct CharacterClassInfo {
    pub id: u32,
    pub name: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

pub const CHARACTER_CLASSES: [CharacterClassInfo; 9] = [
    CharacterClassInfo {
        id: 0,
        name: "Philar",
        title: "Warrior / Martial Artist",
        description: "A balanced martial fighter skilled in hand-to-hand combat and heavy blade techniques.",
    },
    CharacterClassInfo {
        id: 1,
        name: "Azlar",
        title: "Spiritual Sorcerer",
        description: "A wielder of potent spiritual energies capable of casting destructive elemental spells.",
    },
    CharacterClassInfo {
        id: 2,
        name: "Sadad",
        title: "Shadow Assassin",
        description: "A fast and lethal shadow striker excelling in quick strikes and evasive maneuvers.",
    },
    CharacterClassInfo {
        id: 3,
        name: "Destino",
        title: "Tactical Guardian",
        description: "A defensive powerhouse providing stalwart protection and frontline crowd control.",
    },
    CharacterClassInfo {
        id: 4,
        name: "Jarexx",
        title: "Cybernetic Enforcer",
        description: "A heavy arms master augmented with enhanced strength and projectile weapons.",
    },
    CharacterClassInfo {
        id: 5,
        name: "Canon",
        title: "Mystic Channeler",
        description: "An esoteric master channeling ancient psychic forces to disrupt and disorient foes.",
    },
    CharacterClassInfo {
        id: 6,
        name: "Kitara",
        title: "Bladed Archer",
        description: "An agile combatant combining deadly archery with precise dual-blade techniques.",
    },
    CharacterClassInfo {
        id: 7,
        name: "Lunarena",
        title: "Lunar Enchantress",
        description: "A graceful spellcaster harnessing lunar energies to heal allies and weaken adversaries.",
    },
    CharacterClassInfo {
        id: 8,
        name: "Lavita",
        title: "Celestial Mage",
        description: "A versatile magic wielder specializing in high-damage celestial incantations.",
    },
];

/// Account state resource holding active characters, credentials, and selection.
#[derive(Resource, Debug, Clone)]
pub struct AccountData {
    pub username: String,
    pub logged_in: bool,
    pub characters: Vec<CharacterSummary>,
    pub selected_character_index: Option<usize>,
}

impl Default for AccountData {
    fn default() -> Self {
        Self {
            username: "Player1".to_string(),
            logged_in: false,
            characters: vec![
                CharacterSummary {
                    name: "PhilarHero".to_string(),
                    class: 0,
                    level: 25,
                    hair_color: 1,
                    body_color: 7,
                    selected: true,
                },
                CharacterSummary {
                    name: "AzlarMage".to_string(),
                    class: 1,
                    level: 50,
                    hair_color: 15,
                    body_color: 24,
                    selected: false,
                },
            ],
            selected_character_index: Some(0),
        }
    }
}

/// 30 Character Customization Colors in sRGB [0..255]
pub const COLOR_PALETTE: [[u8; 3]; 30] = [
    [255, 255, 255], // 0: White
    [186, 65, 57],   // 1: Red
    [222, 154, 115], // 2: Peach Tan
    [255, 160, 120], // 3: Light Salmon
    [240, 150, 50],  // 4: Orange
    [255, 160, 120], // 5: Soft Coral
    [240, 150, 96],  // 6: Warm Amber
    [231, 219, 99],  // 7: Blonde
    [136, 164, 104], // 8: Olive Green
    [128, 210, 160], // 9: Mint Green
    [156, 207, 157], // 10: Light Sage
    [90, 158, 156],  // 11: Teal
    [115, 158, 132], // 12: Slate Green
    [82, 125, 107],  // 13: Dark Pine
    [156, 203, 165], // 14: Seafoam
    [123, 142, 189], // 15: Periwinkle
    [140, 150, 189], // 16: Lavender Blue
    [107, 125, 173], // 17: Steel Blue
    [222, 219, 231], // 18: Silver White
    [132, 162, 206], // 19: Sky Blue
    [123, 178, 206], // 20: Cyan Blue
    [198, 186, 222], // 21: Pale Violet
    [148, 142, 181], // 22: Muted Purple
    [132, 113, 140], // 23: Plum Violet
    [123, 121, 189], // 24: Royal Indigo
    [189, 138, 156], // 25: Dusty Rose
    [165, 125, 115], // 26: Earth Brown
    [198, 166, 132], // 27: Light Sand
    [165, 130, 90],  // 28: Golden Brown
    [82, 85, 82],    // 29: Dark Charcoal
];

/// Cleanup system to despawn all transient scene entities.
pub fn cleanup_scene_entities(
    mut commands: Commands,
    query: Query<Entity, With<SceneEntity>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
