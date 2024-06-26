use diesel::{prelude::*, ExpressionMethods, PgConnection, RunQueryDsl};
use redis::JsonCommands;
use crate::models::friends::Friend;
use crate::models::client::{CacheClient, Client, ClientState};

pub fn is_friendship_existing(
    connection: &mut PgConnection,
    _client1_id: uuid::Uuid,
    _client2_id: uuid::Uuid
) -> Result<Friend, diesel::result::Error> {
    use crate::schema::friend::dsl::*;

    friend
        .filter(
            (client1_id.eq(_client1_id).and(client2_id.eq(_client2_id)))
            .or(client1_id.eq(_client2_id).and(client2_id.eq(_client1_id)))
        )
        .first::<Friend>(connection)
}

pub fn add_friend(
    connection: &mut PgConnection,
    client1_id: uuid::Uuid,
    client2_id: uuid::Uuid
) -> Friend {
    use crate::schema::friend;

    let test_friendship = is_friendship_existing(connection, client1_id, client2_id);

    match test_friendship {
        Ok(friendship) => friendship,
        Err(_) => {
            let new_friendship = Friend {
                client1_id,
                client2_id
            };
        
            diesel::insert_into(friend::table)
                .values(&new_friendship)
                .get_result(connection)
                .expect("Error saving new friendship")
        }
    }
}

pub fn get_friends(connection: &mut PgConnection, client_id: uuid::Uuid) -> Vec<Client> {
    use diesel::prelude::*;
    use crate::schema::friend;
    use crate::schema::client;

    let (client1, client2) = diesel::alias!(client as client1, client as client2);

    let mut friends_client1: Vec<Client> = friend::table
        .inner_join(client1.on(client1.field(client::id).eq(friend::client1_id)))
        .inner_join(client2.on(client2.field(client::id).eq(friend::client2_id)))
        .select(client2.fields(client::all_columns))
        .filter(friend::client1_id.eq(client_id))
        .load::<Client>(connection)
        .expect("Error loading friends");

    let mut friends_client2: Vec<Client> = friend::table
        .inner_join(client1.on(client1.field(client::id).eq(friend::client1_id)))
        .inner_join(client2.on(client2.field(client::id).eq(friend::client2_id)))
        .select(client1.fields(client::all_columns))
        .filter(friend::client2_id.eq(client_id))
        .load::<Client>(connection)
        .expect("Error loading friends");

    friends_client1.append(&mut friends_client2);
    friends_client1
}

pub fn check_friendship(
    mut redis_connection: redis::Connection,
    friend_clients: Vec<Client>
) -> Vec<CacheClient> {

    let client_list_string = redis_connection.json_get::<_, _, String>("ALL_CLIENTS", "$").unwrap();
    let client_list = serde_json::from_str::<Vec<Vec<CacheClient>>>(&client_list_string).unwrap();
    let client_list = client_list.get(0).unwrap().clone();


    let mut friends: Vec<CacheClient> = vec![];

    for friend_client in friend_clients {
        match client_list.iter().position(|client| client.id == friend_client.id) {
            Some(index) => friends.push(client_list.get(index).unwrap().clone()),
            None => friends.push(
                CacheClient {
                    id: friend_client.id,
                    username: friend_client.username,
                    rank: "unknown".to_string(),
                    state: ClientState::Disconnected
                }
            )
        };
    }

    friends
}