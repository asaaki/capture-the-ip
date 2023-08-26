pub type GenericResult<T> = anyhow::Result<T>;
pub type AppResult = GenericResult<()>;
pub type AnyResult = Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;

#[repr(C)]
pub enum RunMode {
    Dual = 0,
    Api = 1,
    Background = 2,
}

impl TryFrom<u8> for RunMode {
    type Error = &'static str;

    fn try_from(int_mode: u8) -> Result<Self, Self::Error> {
        match int_mode {
            0 => Ok(RunMode::Dual),
            1 => Ok(RunMode::Api),
            2 => Ok(RunMode::Background),
            _ => Err("invalid run mode value selected; allowed: 0, 1, 2"),
        }
    }
}

impl From<RunMode> for u8 {
    fn from(run_mode: RunMode) -> Self {
        match run_mode {
            RunMode::Dual => 0,
            RunMode::Api => 1,
            RunMode::Background => 2,
        }
    }
}
