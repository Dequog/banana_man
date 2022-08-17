#[derive(PartialEq, Clone, Copy)]
pub enum GameState {
    Menu,
    Dead,
    Battling,
    Upgrading,
    Tutorial,
}
