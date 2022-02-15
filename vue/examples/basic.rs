use vue::{methods, Vue};

fn main() {}

#[derive(Default, Vue)]
struct MyComponent {
    list: Vec<String>,

    #[vue(prop)]
    id: String,

    #[vue(sync)]
    b: bool,

    query: Option<FutureExec<SearchResp>>,
}

#[methods]
impl ReactiveMyComponent {
    pub fn handle_click(&self) {}
}

fn my_component_update(mut comp: ReactiveMyComponent) {
    let is_empty = comp.map(|c| c.id.is_empty());
    comp.set_b(!is_empty);
}
