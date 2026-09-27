use client::scenes::{
    AccountData, CharacterCreationDraft, CharacterSummary, EnteringGameState,
    LoginAnimationState, LoginModalState, LoadingProgress, SceneState, CHARACTER_CLASSES,
    COLOR_PALETTE,
};

#[test]
fn test_scene_state_flow() {
    assert_eq!(SceneState::default(), SceneState::Loading);
    let states = vec![
        SceneState::Loading,
        SceneState::Login,
        SceneState::CharacterSelect,
        SceneState::CharacterCreate,
        SceneState::EnteringGame,
        SceneState::InGame,
    ];
    assert_eq!(states.len(), 6);
}

#[test]
fn test_loading_progress_state() {
    let mut progress = LoadingProgress::default();
    assert_eq!(progress.current, 0.0);
    assert_eq!(progress.max, 100.0);

    progress.current = 50.0;
    let ratio = (progress.current / progress.max).clamp(0.0, 1.0);
    assert!((ratio - 0.5).abs() < f32::EPSILON);
}

#[test]
fn test_login_animation_and_modal_state() {
    let anim = LoginAnimationState::default();
    assert_eq!(anim.sun_frame, 0);
    assert!(!anim.is_solo);

    let mut modal = LoginModalState::default();
    assert!(!modal.is_open);
    assert_eq!(modal.username, "Player1");
    assert_eq!(modal.password, "password");

    modal.is_open = true;
    modal.username = "TestUser".to_string();
    assert!(modal.is_open);
    assert_eq!(modal.username, "TestUser");
}

#[test]
fn test_entering_game_state() {
    let entering = EnteringGameState::default();
    assert_eq!(entering.anim_frame, 0);
    assert!(!entering.timer.is_finished());
}

#[test]
fn test_account_data_defaults_and_operations() {
    let mut account = AccountData::default();
    assert_eq!(account.characters.len(), 2);
    assert_eq!(account.selected_character_index, Some(0));

    // Add 3rd character
    let new_char = CharacterSummary {
        name: "TestHero".to_string(),
        class: 3,
        level: 1,
        hair_color: 5,
        body_color: 10,
        selected: true,
    };
    account.characters.push(new_char);
    assert_eq!(account.characters.len(), 3);

    // Remove first character
    account.characters.remove(0);
    assert_eq!(account.characters.len(), 2);
    assert_eq!(account.characters[0].name, "AzlarMage");
}

#[test]
fn test_character_classes_metadata() {
    assert_eq!(CHARACTER_CLASSES.len(), 9);
    assert_eq!(CHARACTER_CLASSES[0].name, "Philar");
    assert_eq!(CHARACTER_CLASSES[1].name, "Azlar");
    assert_eq!(CHARACTER_CLASSES[2].name, "Sadad");
    assert_eq!(CHARACTER_CLASSES[8].name, "Lavita");
}

#[test]
fn test_color_palette_completeness() {
    assert_eq!(COLOR_PALETTE.len(), 30);
    // 0 is default white
    assert_eq!(COLOR_PALETTE[0], [255, 255, 255]);
    // 1 is crimson red
    assert_eq!(COLOR_PALETTE[1], [186, 65, 57]);
}

#[test]
fn test_character_creation_draft() {
    let mut draft = CharacterCreationDraft::default();
    assert_eq!(draft.name, "");
    assert_eq!(draft.class_index, 0);

    draft.name = "ValidHero_1".to_string();
    assert!(draft.name.len() <= 14);
    assert!(draft.name.chars().all(|c| c.is_alphanumeric() || c == '_'));

    // Test slot limit handling
    let mut account = AccountData::default();
    assert_eq!(account.characters.len(), 2);
    assert!(account.characters.len() < 3);

    // Add up to max capacity
    account.characters.push(CharacterSummary {
        name: "ThirdHero".to_string(),
        class: 1,
        level: 1,
        hair_color: 2,
        body_color: 3,
        selected: false,
    });
    assert_eq!(account.characters.len(), 3);
    assert!(!(account.characters.len() < 3));
}
