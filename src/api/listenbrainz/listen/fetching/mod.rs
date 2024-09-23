use chrono::Utc;
use listenbrainz::raw::Client;
use welds::connections::sqlite::SqliteClient;

use crate::api::listenbrainz::listen_collection::SaveListenPayload;
use crate::models::listenbrainz::listen::Listen;
use crate::Error;

impl Listen {
    /// Fetch the latest listens for the provided user. If the user has no listens, it will do a full listen fetch.
    pub async fn fetch_latest_listens_of_user(
        client: &SqliteClient,
        user: &str,
    ) -> Result<(), Error> {
        let latest_listen_ts = Listen::get_latest_listen_of_user(client, user)
            .await?
            .map(|v| v.listened_at);
        let mut pull_ts = Some(Utc::now().timestamp());

        let lb_client = Client::new();

        // This loop has two possible states.
        // - Fresh dump:
        //     `latest_listen_ts` is none. We loop until `save_listen_payload_in_transaction` tell us it's over
        //
        // - Incremental dump:
        //     `latest_listen_ts` is set. We loop until pull_ts is before the latest listen
        while (latest_listen_ts.is_none() && pull_ts.is_some())
            || (latest_listen_ts.is_some_and(|a| pull_ts.is_some_and(|b| a <= b)))
        {
            pull_ts =
                Self::execute_listen_fetch(&client, &lb_client, user, pull_ts.unwrap()).await?;
        }

        Ok(())
    }

    /// Fetch listens for the user and save them in the database
    async fn execute_listen_fetch(
        client: &SqliteClient,
        lb_client: &Client,
        user: &str,
        max_ts: i64,
    ) -> Result<Option<i64>, Error> {
        println!("Fetching {max_ts}");
        let dump = lb_client.user_listens(user, None, Some(max_ts), Some(1000));

        println!("Saving {max_ts}");
        match dump {
            Ok(val) => Ok(val
                .payload
                .save_listen_payload_in_transaction(client, max_ts, 1000)
                .await?),

            #[cfg(feature = "timeout_continue")]
            Err(listenbrainz::Error::Http(_err)) => Ok(None),

            Err(err) => Err(err)?,
        }
    }
}
