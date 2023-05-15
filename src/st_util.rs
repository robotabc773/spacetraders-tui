use anyhow::Result;
use spacedust::{
    apis::{
        agents_api::get_my_agent,
        contracts_api::{get_contracts, GetContractsError},
        factions_api::{get_factions, GetFactionsError},
        fleet_api::{get_my_ships, GetMyShipsError},
        systems_api::{
            get_system_waypoints, get_systems, GetSystemWaypointsError, GetSystemsError,
        },
    },
    models::{Contract, Faction, Ship, System, Waypoint},
};

use crate::config::{get_global_db_pool, CONFIGURATION};

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

/// Updates information on the agent page from SpaceTraders API
///
/// # Errors
/// Errors on request failure or database failure
///
/// # Panics
/// Pretty sure it won't, but the compiler thinks it might
pub async fn refresh_agent_page() -> Result<()> {
    let agent = get_my_agent(&CONFIGURATION).await?.data;

    sqlx::query!(
        "INSERT INTO agents(account_id, symbol, headquarters, credits)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (symbol) DO 
                UPDATE SET credits = EXCLUDED.credits",
        agent.account_id,
        agent.symbol,
        agent.symbol,
        agent.credits
    )
    .execute(get_global_db_pool().await)
    .await?;

    Ok(())
}
