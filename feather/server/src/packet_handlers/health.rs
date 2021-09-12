use crate::{ClientId, NetworkId, Position, Server};
use common::Game;
use ecs::{Entity, SysResult};
use protocol::packets::client::ClientStatus;
use uuid::Uuid;

pub fn handle_client_status(
    game: &mut Game,
    server: &mut Server,
    player_id: Entity,
    packet: ClientStatus,
) -> SysResult {
    match packet {
        ClientStatus::PerformRespawn => {
            let client_id = game.ecs.get::<ClientId>(player_id).unwrap();
            let client = server.clients.get(*client_id).unwrap();

            client.respawn_player(server.options.default_gamemode);

            let player = game.ecs.entity(player_id)?;
            game.reset_player(player)?;

            let network_id = game.ecs.get::<NetworkId>(player_id).unwrap();
            let position = game.ecs.get::<Position>(player_id).unwrap();
            let uuid = game.ecs.get::<Uuid>(player_id).unwrap();

            // Recreate the player for all clients.
            server.broadcast_nearby_with(*position, |client| {
                if client.network_id() != *network_id {
                    client.send_player(*network_id, *uuid, *position);
                }
            });
        }
        ClientStatus::RequestStats => {}
    }

    Ok(())
}
