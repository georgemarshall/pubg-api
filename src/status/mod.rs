use chrono::prelude::*;
use hyper::client::Connect;

use {GenericFuture, Pubg};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Status {
    id: String,
    /// releasedAt
    released_at: DateTime<Utc>,
    /// version
    version: String,
}
resource!(Status, |&self| {
    kind "status";
    id self.id;
    attrs released_at, version;
});

pub struct StatusRequest<C>
where
    C: Clone + Connect,
{
    pubg: Pubg<C>,
}

impl<C: Clone + Connect> StatusRequest<C> {
    pub fn new(pubg: Pubg<C>) -> Self {
        Self { pubg }
    }

    pub fn get(&self) -> GenericFuture<Status> {
        self.pubg.get("/status")
    }
}
