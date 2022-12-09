use matrix_sdk::{
    config::SyncSettings,
    event_handler::Ctx,
    room::Room,
    ruma::events::room::{member::StrippedRoomMemberEvent, message::OriginalSyncRoomMessageEvent},
    Client,
};
use miette::IntoDiagnostic;
use std::sync::{Arc, RwLock};
use tracing::{debug, info};

use crate::Config;

#[derive(Clone)]
pub struct State {
    pub inner: Arc<Inner>,
}

pub struct Inner {
    /// The matrix username.
    pub username: String,
    /// The matrix password.
    pub password: String,
    /// The user-specified configuration.
    pub config: Config,
    /// URL of the homeserver.
    pub homeserver_url: String,
}

impl State {
    /// Creates a new [`Core`] using the given `config`.
    pub fn with_config(
        homeserver_url: String,
        username: String,
        password: String,
        config: Config,
    ) -> State {
        let inner = Inner {
            username,
            password,
            config,
            homeserver_url,
        };

        State {
            inner: Arc::new(inner),
        }
    }

    pub fn init(&self) {}

    /// Returns the homeserver.
    pub fn homeserver(&self) -> &str {
        &self.inner.homeserver_url
    }

    /// Returns the username.
    pub fn username(&self) -> &str {
        &self.inner.username
    }

    /// Returns the passwird.
    pub fn password(&self) -> &str {
        &self.inner.password
    }

    /// Returns the id of the root space.
    pub fn root_room_id(&self) -> &str {
        &self.inner.config.matrix.space.room_id
    }

    // pub async fn login_and_sync(&self) -> miette::Result<()> {
    //     let homeserver_url = self.inner.homeserver_url
    //         let state = self.inner.read().unwrap();

    //         state.homeserver_url.clone()
    //     };

    //     debug!(homeserver = %homeserver_url, "Connecting to homeserver");

    //     let sled_dir = dirs::data_dir()
    //         .ok_or_else(|| miette::miette!("no home directory found"))?
    //         .join("matrix-ctf-bot");

    //     let client = Client::builder()
    //         .homeserver_url(&homeserver_url)
    //         .sled_store(sled_dir, None)
    //         .into_diagnostic()?
    //         .build()
    //         .await
    //         .unwrap();

    //     let auth = {
    //         let state = self.inner.read().unwrap();

    //         state.auth.clone()
    //     };

    //     // Log in to the homeserver.
    //     let Authentication::Credentials {
    //         ref username,
    //         ref password,
    //     } = auth;

    //     client
    //         .login_username(username, password)
    //         .send()
    //         .await
    //         .into_diagnostic()?;

    //     client.add_event_handler_context(self.clone());

    //     client.add_event_handler(
    //         |room_member: StrippedRoomMemberEvent,
    //          client: Client,
    //          room: Room,
    //          state: Ctx<State>| async move {},
    //     );

    //     info!("Logged in as {username}");

    //     Ok(())
    // }
}
