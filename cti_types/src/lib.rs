pub type GenericResult<T> = color_eyre::Result<T>;
pub type AppResult = GenericResult<()>;
pub type AnyResult = Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;

#[repr(C)]
pub enum RunMode {
    Dual = 0,
    Api = 1,
    Background = 2,
}

impl From<u8> for RunMode {
    fn from(int_mode: u8) -> Self {
        match int_mode {
            0 => RunMode::Dual,
            1 => RunMode::Api,
            2 => RunMode::Background,
            _ => panic!("invalid run mode value selected; allowed: 0, 1, 2"),
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
