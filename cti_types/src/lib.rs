pub type GenericResult<T> = color_eyre::Result<T>;
pub type AppResult = GenericResult<()>;
pub type AnyResult = Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;

pub enum RunMode {
    Api,
    Background,
    Dual,
}
