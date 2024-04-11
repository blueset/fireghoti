//! ID generation utility based on [cuid2]

use basen::BASE36;
use cfg_if::cfg_if;
use chrono::NaiveDateTime;
use once_cell::sync::OnceCell;
use std::cmp;

use crate::impl_into_napi_error;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error("ID generator has not been initialized yet")]
pub struct ErrorUninitialized;

impl_into_napi_error!(ErrorUninitialized);

static FINGERPRINT: OnceCell<String> = OnceCell::new();
static GENERATOR: OnceCell<cuid2::CuidConstructor> = OnceCell::new();

const TIME_2000: i64 = 946_684_800_000;
const TIMESTAMP_LENGTH: u16 = 8;

/// Initializes Cuid2 generator. Must be called before any [create_id].
#[cfg_attr(feature = "napi", crate::export)]
pub fn init_id_generator(length: u16, fingerprint: &str) {
    FINGERPRINT.get_or_init(move || format!("{}{}", fingerprint, cuid2::create_id()));
    GENERATOR.get_or_init(move || {
        cuid2::CuidConstructor::new()
            // length to pass shoule be greater than or equal to 8.
            .with_length(cmp::max(length - TIMESTAMP_LENGTH, 8))
            .with_fingerprinter(|| FINGERPRINT.get().unwrap().clone())
    });
}

/// Returns Cuid2 with the length specified by [init_id]. Must be called after
/// [init_id], otherwise returns [ErrorUninitialized].
pub fn create_id(datetime: &NaiveDateTime) -> Result<String, ErrorUninitialized> {
    match GENERATOR.get() {
        None => Err(ErrorUninitialized),
        Some(gen) => {
            let date_num = cmp::max(0, datetime.and_utc().timestamp_millis() - TIME_2000) as u64;
            Ok(format!(
                "{:0>8}{}",
                BASE36.encode_var_len(&date_num),
                gen.create_id()
            ))
        }
    }
}

#[cfg_attr(feature = "napi", crate::export)]
pub fn get_timestamp(id: &str) -> i64 {
    let n: Option<u64> = BASE36.decode_var_len(&id[0..8]);
    match n {
        None => -1,
        Some(n) => n as i64 + TIME_2000,
    }
}

cfg_if! {
    if #[cfg(feature = "napi")] {
        use chrono::{DateTime, Utc};

        /// The generated ID results in the form of `[8 chars timestamp] + [cuid2]`.
        /// The minimum and maximum lengths are 16 and 24, respectively.
        /// With the length of 16, namely 8 for cuid2, roughly 1427399 IDs are needed
        /// in the same millisecond to reach 50% chance of collision.
        ///
        /// Ref: https://github.com/paralleldrive/cuid2#parameterized-length
        #[napi_derive::napi]
        pub fn gen_id(date: Option<DateTime<Utc>>) -> String {
            create_id(&date.unwrap_or_else(Utc::now).naive_utc()).unwrap()
        }
    }
}

#[cfg(test)]
mod unit_test {
    use crate::util::id;
    use chrono::Utc;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::thread;

    #[test]
    fn can_create_and_decode_id() {
        let now = Utc::now().naive_utc();
        assert_eq!(id::create_id(&now), Err(id::ErrorUninitialized));
        id::init_id_generator(16, "");
        assert_eq!(id::create_id(&now).unwrap().len(), 16);
        assert_ne!(id::create_id(&now).unwrap(), id::create_id(&now).unwrap());
        let id1 = thread::spawn(move || id::create_id(&now).unwrap());
        let id2 = thread::spawn(move || id::create_id(&now).unwrap());
        assert_ne!(id1.join().unwrap(), id2.join().unwrap());

        let test_id = id::create_id(&now).unwrap();
        let timestamp = id::get_timestamp(&test_id);
        assert_eq!(now.and_utc().timestamp_millis(), timestamp);
    }
}
