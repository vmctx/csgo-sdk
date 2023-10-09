//! Minimalistic custom Error type.
//! # Examples
//! ```
//! use sdk::utils::error::Error;
//!
//! fn throws() -> Result<(), Error> {
//!     Err(Error::Unknown {
//!         message: "Example".into()
//!     })
//! }
//! ```

use crate::alloc::string::ToString;
use alloc::string::String;
use custom_error::custom_error;

custom_error! {pub Error
    Null{item:String} = "{item} is null.",
    NotFound{item:String} = "couldn't find {item}.",
    Other{msg:String} = "{msg}",
    Unknown{message:String} = "unknown error: {message}"
}
