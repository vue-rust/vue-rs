use serde::Serialize;

#[derive(Serialize)]
pub struct CreateConfig {
    pub props: Vec<&'static str>,
}
