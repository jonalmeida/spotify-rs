//#[feature(libc)]
//extern crate libc;

use super::*;

pub struct sp_session_config;
pub struct sp_session;

#[link(name = "spotify")]
extern {
    fn sp_session_create(config: sp_session_config , session: *mut sp_session) -> Error;
}

pub fn session_create(config: sp_session_config, session: *mut sp_session) -> Error {
    unsafe {
        sp_session_create(config, session)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_session_create() {
        /*
        let spconfig: sp_session_config = {
            api_version = 12,
            cache_location = "tmp",
            settings_location = "tmp",
            application_key = "",
            application_key_size = 0, // Set in main()
            user_agent = "spotify-jukebox-example",
            callbacks = 0,
            0,
        };
        */
    }

}