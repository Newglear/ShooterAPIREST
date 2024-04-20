use crate::models::client::CacheClient;
use crate::models::dgs::{DedicatedGameServer, DgsCluster, RatedDgs};
use redis::JsonCommands; // Add this line
use serde_json::{self};

pub fn setup_dgs_map(mut connection: redis::Connection) -> () {
    let main_cluster = DgsCluster {
        name: "ALL_DGS".to_string(),
        dgs: vec![]
    };
    connection.json_set::<_, _, DgsCluster, ()>(&main_cluster.name, "$", &main_cluster).unwrap()
}

pub fn register_dgs(mut connection: redis::Connection, dgs: DedicatedGameServer) -> DedicatedGameServer {
    // Convert the DGS to a JSON string
    let dgs_json = serde_json::to_string(&dgs).unwrap();

    // Add the DGS to the 'dgs' field of 'ALL_DGS'
    let path = format!("$.dgs[{}]", dgs.name); // Assuming 'id' is a field of DedicatedGameServer
    connection.json_set::<_, _, _, ()>("ALL_DGS", &path, &dgs_json).unwrap();

    dgs
}

pub fn add_player_to_dgs(mut connection: redis::Connection, dgs_name: &str, player: CacheClient) -> DedicatedGameServer {
    let path = format!("$.dgs[{}]", dgs_name);

    let string_dgs = match connection.json_get::<_, &str, String>("ALL_DGS", &path) {
        Ok(dgs) => dgs,
        Err(_) => "".to_string()
    };

    let mut dgs: DedicatedGameServer = serde_json::from_str(&string_dgs).unwrap();
    dgs.players.push(player);
    
    connection.json_set::<_, _, String, ()>("ALL_DGS", path, &serde_json::to_string(&dgs).unwrap()).unwrap();
    dgs
}

pub fn find_dgs_by_rank(mut connection: redis::Connection, rank: i32) -> DedicatedGameServer {
    let path = "$.dgs";
    let mut dgs_list: Vec<RatedDgs> = vec![];

    let dgs_list_string = connection.json_get::<_, &str, String>("ALL_DGS", &path).unwrap();
    let dgs_list_json: Vec<DedicatedGameServer> = serde_json::from_str(&dgs_list_string).unwrap();
    for dgs in dgs_list_json {
        let ranks: Vec<i32> = dgs.players.iter().map(|player| { player.rank_id}).collect();
        let median = ranks.get(ranks.len()/2);
        let rating = match median {
            Some(median) => {
                ((median - rank) as f32 / (ranks.len() + 1) as f32).abs() // Modified this line
            },
            None => 0.5
        };
        dgs_list.push(RatedDgs {
            dgs,
            rating
        });
    }
    dgs_list.sort_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap());

    dgs_list.first().unwrap().dgs.clone()
}