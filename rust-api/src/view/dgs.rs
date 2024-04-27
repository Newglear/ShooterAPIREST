use actix_web::{web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;
use crate::{controller::database_manager::establish_redis_connection, handle_jwt_token, models::dgs::{DedicatedGameServer, DedicatedGameServerRegister}, DbPool};

/// Should be called from DGS
#[actix_web::post("/dgs/register")]
pub async fn register_dgs(
    req: HttpRequest,
    dgs_port: web::Json<u16>
) -> actix_web::Result<impl Responder> {
    
    let sending_socket = match req.peer_addr() {
        Some(socket) => socket,
        None => return Err(actix_web::error::ErrorInternalServerError("Couldn't get socket"))
    };
    let claim = match handle_jwt_token(req) {
        Ok(claim) => claim,
        Err(err) => return Err(err)
    };
    let id: Uuid = match claim.extract_uuid() {
        Ok(id) => id,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err))
    };
    

    let request_dgs = DedicatedGameServerRegister {
        ip: sending_socket.ip(),
        port: dgs_port.into_inner()
    };
    println!("Registering DGS {} at {}:{}", id, request_dgs.ip, request_dgs.port);

    let dgs = DedicatedGameServer {
        id,
        ip: request_dgs.ip,
        port: request_dgs.port,
        players: vec![]
    };
    let con = match establish_redis_connection() {
        Ok(con) => con,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err))
    };
    let res = crate::controller::dgs::register_dgs(con, dgs);
    
    Ok(HttpResponse::Ok().body(format!("Registered DGS: {}", res.id)))
}

/// Should be called from DGS
#[actix_web::post("/dgs/addplayer")]
pub async fn add_player_to_dgs(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    player: web::Json<String>
) -> actix_web::Result<impl Responder> {
    let claim = match handle_jwt_token(req) {
        Ok(claim) => claim,
        Err(err) => return Err(err)
    };
    let id: Uuid = match claim.extract_uuid() {
        Ok(id) => id,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err))
    };

    let player_client = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        crate::controller::client::get_user_by_username(&mut conn, player.into_inner().as_str())
    }).await?;

    let cached_client = crate::models::client::CacheClientDGS {
        id: player_client.id,
        username: player_client.username,
        rank_id: player_client.rank_id
    };

    let con = match establish_redis_connection() {
        Ok(con) => con,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err))
    };
    let res = crate::controller::dgs::add_player_to_dgs(con, &*id.to_string(), cached_client.clone());
    
    Ok(HttpResponse::Ok().body(
        format!("Add player {} to DGS: {}",
        cached_client.username,
        res.id)
    ))
}

#[actix_web::delete("/dgs/removeplayer")]
pub async fn remove_player_from_dgs(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    player: web::Path<String>
) -> actix_web::Result<impl Responder> {
    let claim = match handle_jwt_token(req) {
        Ok(claim) => claim,
        Err(err) => return Err(err)
    };
    let id: Uuid = match claim.extract_uuid() {
        Ok(id) => id,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err))
    };

    let player_client = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        crate::controller::client::get_user_by_username(&mut conn, player.into_inner().as_str())
    }).await?;

    let con = match establish_redis_connection() {
        Ok(con) => con,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err))
    };
    let res = crate::controller::dgs::remove_player_from_dgs(con, &*id.to_string(), player_client.id);
    
    Ok(HttpResponse::Ok().body(
        format!("Remove player {} from DGS: {}",
        player_client.username,
        res.id)
    ))
}

/// Should be called from player client
    #[actix_web::get("/client/finddgs")]
pub async fn find_dgs(
    req: HttpRequest,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let claim = match handle_jwt_token(req) {
        Ok(claim) => claim,
        Err(err) => return Err(err)
    };
    let id: Uuid = match claim.extract_uuid() {
        Ok(id) => id,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err))
    };

    let player_client = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        crate::controller::client::get_user_by_id(&mut conn, id)
    }).await?;

    let con = match establish_redis_connection() {
        Ok(con) => con,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err))
    };
    let res = crate::controller::dgs::find_dgs_by_rank(con, player_client.rank_id);
    
    Ok(HttpResponse::Ok().json(res))
}

#[actix_web::get("/dgs/players")]
pub async fn get_clients_in_dgs(
    req: HttpRequest
) -> actix_web::Result<impl Responder> {
    let claim = match handle_jwt_token(req) {
        Ok(claim) => claim,
        Err(err) => return Err(err)
    };
    let id: Uuid = match claim.extract_uuid() {
        Ok(id) => id,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err))
    };

    let con = match establish_redis_connection() {
        Ok(con) => con,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err))
    };
    let res = crate::controller::dgs::get_players_in_dgs(con, &*id.to_string());
    
    Ok(HttpResponse::Ok().json(res))
}