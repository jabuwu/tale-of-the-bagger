#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AppState {
    #[default]
    Loading,
    Game,
}
