pub mod handler;

#[derive(Clone, Debug)]
pub enum IoEvent {
    UpdateAgent,
    UpdateContracts,
    UpdateFactions,
    AcceptContract(String),
    FulfillContract(String),
}
