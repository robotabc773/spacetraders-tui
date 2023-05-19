pub mod handler;

#[derive(Clone, Copy, Debug)]
pub enum IoEvent {
    UpdateAgent,
    UpdateContracts,
    UpdateFactions,
}
