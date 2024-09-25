pub mod crud;
pub mod redirection;
use welds::WeldsModel;

#[derive(Debug, WeldsModel, Clone)]
#[welds(table = "artists")]
pub struct Artist {
    #[welds(primary_key)]
    pub id: i64,
    pub mbid: String,
    pub name: String,
    pub sort_name: String,
    pub disambiguation: String,
    pub country: Option<String>,
    pub annotation: Option<String>,
}

impl Artist {}
