use matrix_sdk::{
    config::SyncSettings,
    event_handler::Ctx,
    room::Room,
    ruma::events::room::{member::StrippedRoomMemberEvent, message::OriginalSyncRoomMessageEvent},
    ruma::RoomId,
    Client,
};
use miette::IntoDiagnostic;
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info, warn};

use crate::state::State;

pub async fn login_and_sync(state: State) -> miette::Result<()> {
    let homeserver_url = state.homeserver();
    let username = state.username();

    debug!(homeserver = %homeserver_url, "Connecting to homeserver");

    let sled_dir = dirs::data_dir()
        .ok_or_else(|| miette::miette!("no home directory found"))?
        .join("matrix-ctf-bot");

    let client = Client::builder()
        .homeserver_url(homeserver_url)
        .sled_store(sled_dir, None)
        .into_diagnostic()?
        .build()
        .await
        .unwrap();

    // Log in to the homeserver.
    client
        .login_username(state.username(), state.password())
        .send()
        .await
        .into_diagnostic()?;

    client.add_event_handler_context(state.clone());

    info!("Logged in as {username}");

    client.add_event_handler(on_stripped_state_member);

    // Do an initial sync to set up our state.
    let sync_token = client
        .sync_once(SyncSettings::default())
        .await
        .into_diagnostic()?
        .next_batch;

    // Determine if we're in the root space.
    let rooms = client.joined_rooms();
    let root_room_id = <&RoomId>::try_from(state.root_room_id()).unwrap();

    if rooms.iter().find(|r| r.room_id() == root_room_id).is_none() {
        info!("Joining the defined root space");

        client
            .join_room_by_id(root_room_id)
            .await
            .into_diagnostic()?;
    }

    client.add_event_handler(on_room_message);

    // Use our previous token to continue sync.
    let settings = SyncSettings::default().token(sync_token);

    // Syncing is important to synchronize the client state with the server.
    // This method will never return.
    let _ = client.sync(settings).await;

    Ok(())
}

async fn on_stripped_state_member(
    room_member: StrippedRoomMemberEvent,
    client: Client,
    room: Room,
    ctx: Ctx<State>,
) {
    if room_member.state_key != client.user_id().unwrap() {
        // the invite we've seen isn't for us, but for someone else. ignore
        return;
    }

    if let Room::Invited(ref r) = room {
        if r.is_space() {
            if r.room_id() == ctx.root_room_id() {
                info!("Accepting invitation to join the root space");
            }

            tokio::spawn(async move {
                join_room(room).await;
            });
        }
    } else if let Room::Joined(ref r) = room {
        debug!(?r, "joined");
    }
}

async fn join_room(room: Room) {
    if let Room::Invited(room) = room {
        info!("Autojoining room {}", room.room_id());
        let mut delay = 2;

        while let Err(err) = room.accept_invitation().await {
            // retry autojoin due to synapse sending invites, before the
            // invited user can join for more information see
            // https://github.com/matrix-org/synapse/issues/4345
            warn!(
                "Failed to join room {} ({err:?}), retrying in {delay}s",
                room.room_id()
            );

            sleep(Duration::from_secs(delay)).await;
            delay *= 2;

            if delay > 3600 {
                error!("Can't join room {} ({err:?})", room.room_id());
                break;
            }
        }
        info!("Successfully joined room {}", room.room_id());
    }
}

async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
    debug!(?event, ?room, "Received room message");
}
