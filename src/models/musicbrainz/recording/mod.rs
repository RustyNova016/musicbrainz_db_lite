use welds::{state::DbState, Client, WeldsError, WeldsModel};

pub mod redirect;

#[derive(Debug, WeldsModel)]
#[welds(table = "recordings")]
pub struct Recording {
    #[welds(primary_key)]
    pub id: i64,

    pub gid: String,

    pub title: String,

    pub length: Option<u32>,

    pub disambiguation: Option<String>,

    pub annotation: Option<String>,
}

impl Recording {
    pub async fn find_by_mbid(client: &dyn Client, mbid: &str) -> Result<Option<DbState<Recording>>, WeldsError> {
        Ok(Recording::where_col(|c| c.gid.equal(mbid)).limit(1).run(client).await?.pop())
    }

    pub fn replace(mut row: DbState<Recording>, new: Recording) -> DbState<Self> {
        let id = row.id;

        *row = new;
        row.id = id;

        row
    }
}
