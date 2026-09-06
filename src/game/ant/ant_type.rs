#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AntType {
    #[default]
    Unit,
    Worker,
    Soldier,
}
