#[derive(Debug, Fail)]
pub enum OSDetectionError {
    #[fail(display = "lsb_release command failed")]
    LSBReleaseCommandFail,
    #[fail(display = "sw_vers command failed")]
    SwVersCommandFailed,
}
