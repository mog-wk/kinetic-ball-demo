
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Genereic error")]
    Generic(String),
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    WindowBuildError(#[from] sdl2::video::WindowBuildError),
    #[error(transparent)]
    CanvasBuildError(#[from] sdl2::IntegerOrSdlError),

}

impl std::convert::From<String> for Error {
    fn from(st: String) -> Self {
        Self::Generic(st)
    }
}
