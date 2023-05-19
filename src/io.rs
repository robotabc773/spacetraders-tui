pub mod handler;

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub enum IoEvent {
    UpdateAgent,
    UpdateContracts,
    UpdateFactions,
    AcceptContract(String),
    FulfillContract(String),
}
