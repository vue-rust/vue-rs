use vue::Vue;

fn main() {}

#[derive(Default, Vue)]
struct MyComponent {
    #[vue(prop)]
    id: String,

    #[vue(sync)]
    b: bool,
}

async fn my_component_update(mut comp: ReactiveMyComponent) {
    loop {
        let is_empty = comp.map(|comp| comp.id.is_empty());
        comp.set_b(!is_empty);
    }
}
