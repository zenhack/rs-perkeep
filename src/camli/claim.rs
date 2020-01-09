use serde::{Serialize, Deserialize};
use crate::blob;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Shared {
    pub claim_date: Option<String>, // TODO: make a proper date.
    pub camli_signer: blob::Ref,
    // N.B. we leave camliSig out. It doesn't make sense to have it
    // here because:
    //
    // - When creating claims, the signature has to be added after
    //   serialization, so there's no sense in setting a field here.
    // - If we try to re-serialize an existing blob, we'll likely break
    //   the signature due to whitespace differences, so there's no reason
    //   to retain the field for the round trip.
    // - When decerializing, we need to check the signature first, so
    //   the parse routine should just check the sig and return a value
    //   encompassing the verification result.
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keep {
    pub target: blob::Ref,
    #[serde(flatten)]
    pub shared: Shared,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Delete {
    pub target: blob::Ref,
    #[serde(flatten)]
    pub shared: Shared,
}

pub mod share {
    use crate::blob;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Share {
        pub auth_type: AuthType,
        pub transitive: bool,
        #[serde(flatten)]
        pub what: Target,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub enum AuthType {
        Haveref,
    }


    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub enum Target {
        Target(blob::Ref),
        // Search(search::SearchQuery), // TODO
    }
}
