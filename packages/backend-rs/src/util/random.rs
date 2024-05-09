use rand::{distributions::Alphanumeric, thread_rng, Rng};

/// Generate random string based on [thread_rng] and [Alphanumeric].
#[crate::export]
pub fn generate_secure_random_string(length: u16) -> String {
    thread_rng()
        .sample_iter(Alphanumeric)
        .take(length.into())
        .map(char::from)
        .collect()
}

#[crate::export]
pub fn generate_user_token() -> String {
    generate_secure_random_string(16)
}

#[cfg(test)]
mod unit_test {
    use pretty_assertions::{assert_eq, assert_ne};
    use std::thread;

    use super::generate_secure_random_string;

    #[test]
    fn can_generate_unique_strings() {
        assert_eq!(generate_secure_random_string(16).len(), 16);
        assert_ne!(
            generate_secure_random_string(16),
            generate_secure_random_string(16)
        );
        let s1 = thread::spawn(|| generate_secure_random_string(16));
        let s2 = thread::spawn(|| generate_secure_random_string(16));
        assert_ne!(s1.join().unwrap(), s2.join().unwrap());
    }
}
