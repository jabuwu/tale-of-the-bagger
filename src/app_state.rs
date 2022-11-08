use strum_macros::EnumIter;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Hash, EnumIter)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    Game,
}
