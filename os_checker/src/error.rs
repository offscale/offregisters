#[derive(Debug, Fail)]
pub enum OSDetectionError {
    #[fail(display = "lsb_release command failed")]
    LSBReleaseCommandFail
}
