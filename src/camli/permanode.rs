use serde::{Serialize, Deserialize};
use crate::blob;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Permanode {
    random: String,
    claim_date: String, // TODO: should be more specialized date type.
    camli_signer: blob::Ref,
    // N.B. we leave camliSig out. It doesn't make sense to have it
    // here because:
    //
    // - When creating permanodes, the signature has to be added after
    //   serialization, so there's no sense in setting a field here.
    // - If we try to re-serialize an existing blob, we'll likely break
    //   the signature due to whitespace differences, so there's no reason
    //   to retain the field for the round trip.
    // - When decerializing, we need to check the signature first, so
    //   the parse routine should just check the sig and return a value
    //   encompassing the verification result.
}
