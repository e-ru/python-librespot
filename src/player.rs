use cpython::{PyObject, PyResult, Python};
use librespot;
use pyfuture::py_wrap_future;
use std::cell::RefCell;
use tokio_core::reactor::Remote;
use SpotifyId;
// https://octobus.net/blog/2019-07-25-rust-cpython-shared-ref.html

py_class!(pub class Player |py| {
    data player: RefCell<librespot::playback::player::Player>;
    data handle: Remote;

    def load(&self, track: SpotifyId, play: bool = true, position_ms: u32 = 0) -> PyResult<PyObject> {
        let player = &mut self.player(py).borrow_mut();
        let handle = self.handle(py).clone();
        let track = *track.id(py);

        player.load(track, play, position_ms);

        py_wrap_future(py, handle, player.get_end_of_track_future(), |_py, _result| {
            Ok(true)
        })
    }

    def play(&self) -> PyResult<PyObject> {
        let player = self.player(py).borrow_mut();
        player.play();
        Ok(py.None())
    }

    def pause(&self) -> PyResult<PyObject> {
        let player = self.player(py).borrow_mut();
        player.pause();
        Ok(py.None())
    }

    def stop(&self) -> PyResult<PyObject> {
        let player = self.player(py).borrow_mut();
        player.stop();
        Ok(py.None())
    }
});

impl Player {
    pub fn new(
        py: Python,
        session: librespot::core::session::Session,
        device: PyObject,
        handle: Remote,
    ) -> PyResult<Player> {
        use librespot::playback::config::PlayerConfig;

        let config = PlayerConfig::default();

        println!("LibRespot: device = {:?}", device);
        let device_type = device.get_type(py);
        let type_name = device_type.name(py);
        println!("LibRespot: Device Type obj {:?}", type_name);

        // let backend = librespot::playback::audio_backend::dynamic_sink();

        // Uses default backend: Pipe
        let backend = librespot::playback::audio_backend::find(None).unwrap();

        let (player, _) =
            librespot::playback::player::Player::new(config, session, None, move || {
                // Argument to sink builder is Option<String>
                // (backend)(move |x: &[u8]| {
                //     {
                //         let guard = Python::acquire_gil();
                //         let py = guard.python();
                //         let python_bytes = PyBytes::new(py, x);
                //         // TODO: Should check this error!
                //         device.call_method(py, "write", (python_bytes,), None).unwrap();
                //     }
                //     Ok(())
                // })
                (backend)(None)
            });

        Player::create_instance(py, RefCell::new(player), handle)
    }
}
