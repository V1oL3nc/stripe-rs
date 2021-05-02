// ======================================
// This file was automatically generated.
// ======================================

use crate::params::Object;
use serde_derive::{Deserialize, Serialize};
use crate::ids::{ApplicationId};

/// The resource representing a Stripe "Application".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Application {
    /// Unique identifier for the object.
    pub id: ApplicationId,

    /// The name of the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Object for Application {
    type Id = ApplicationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "application"
    }
}
