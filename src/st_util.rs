use std::fmt::Display;

use anyhow::Result;
use spacedust::{
    apis::{
        contracts_api::{get_contracts, GetContractsError},
        factions_api::{get_factions, GetFactionsError},
        fleet_api::{get_my_ships, GetMyShipsError},
        systems_api::{
            get_system_waypoints, get_systems, GetSystemWaypointsError, GetSystemsError,
        },
    },
    models::{Contract, Faction, Ship, System, Waypoint},
};

use crate::config::CONFIGURATION;

const MAX_PAGE_SIZE: i32 = 20;

macro_rules! impl_list {
    ($(#[$attr:meta])* $func:path => $vis:vis async fn $name:ident ( $($extra_i:ident : $extra_t:ty,)* ) -> Result<Vec<$out:ty>, Error<$err:ty>> ) => {
        $(#[$attr])*
        $vis async fn $name($($extra_i : $extra_t,)*) -> Result<Vec<$out>, spacedust::apis::Error<$err>>
        {
            let mut page = 1;
            let mut result: Vec<$out> = Vec::new();
            loop {
                match $func(&CONFIGURATION, $($extra_i,),* Some(page), Some(MAX_PAGE_SIZE)).await {
                    Ok(res) => {
                        let data = res.data;
                        let meta = *(res.meta);
                        result.extend(data);
                        if meta.total > meta.page * meta.limit {
                            page += 1;
                        } else {
                            break;
                        }
                    }
                    Err(err_res) => {
                        return Err(err_res);
                    }
                }
            }
            Ok(result)
        }
    };
}

impl_list!(
    /// Get a list of all waypoints in a given system
    ///
    /// # Errors
    /// Propogates any error from `get_system_waypoints`
    get_system_waypoints => pub async fn list_system_waypoints(
        system_symbol: &str,
    ) -> Result<Vec<Waypoint>, Error<GetSystemWaypointsError>>
);
impl_list!(
    /// Get a list of all known factions
    ///
    /// # Errors
    /// Propogates any error from `get_factions`
    get_factions => pub async fn list_factions() -> Result<Vec<Faction>, Error<GetFactionsError>>
);
impl_list!(
    /// Get a list of all your contracts
    ///
    /// # Errors
    /// Propogates any error from `get_contracts`
    get_contracts => pub async fn list_contracts() -> Result<Vec<Contract>, Error<GetContractsError>>
);
impl_list!(
    /// Get a list of all your ships
    ///
    /// # Errors
    /// Propogates any error from `get_ships`
    get_my_ships => pub async fn list_ships() -> Result<Vec<Ship>, Error<GetMyShipsError>>
);
impl_list!(
    /// Get a list of all known systems
    ///
    /// # Errors
    /// Propogates any error from `get_systems`
    get_systems => pub async fn list_systems() -> Result<Vec<System>, Error<GetSystemsError>>
);

#[must_use]
pub fn contract_type_to_string(contract_type: &spacedust::models::contract::RHashType) -> &str {
    match contract_type {
        spacedust::models::contract::RHashType::Shuttle => "Shuttle",
        spacedust::models::contract::RHashType::Transport => "Transport",
        spacedust::models::contract::RHashType::Procurement => "Procurement",
    }
}
