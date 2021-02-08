#[macro_use]
extern crate cpython;
extern crate futures;
extern crate librespot;
extern crate tokio_core;

use cpython::{Python, PyResult};

mod pyfuture;
mod player;
mod session;
mod metadata;
mod webtoken;

py_class!(pub class SpotifyId |py| {
    data id : librespot::core::spotify_id::SpotifyId;

    def __new__(_cls, base62: &str) -> PyResult<SpotifyId> {
        let id = librespot::core::spotify_id::SpotifyId::from_base62(base62);
        SpotifyId::create_instance(py, id)
    }
});

impl SpotifyId {
    pub fn new(py: Python, id: librespot::core::spotify_id::SpotifyId) -> PyResult<SpotifyId> {
        SpotifyId::create_instance(py, id)
    }
}

py_module_initializer!(librespot, initlibrespot, PyInit_librespot, |py, m| {
    m.add_class::<session::Session>(py)?;
    m.add_class::<SpotifyId>(py)?;
    Ok(())
});
