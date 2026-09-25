use antora_cli::{DefaultTemplateResolver, run_cli};

fn main() {
    let template_resolver = Box::new(DefaultTemplateResolver::default());
    run_cli!(template_resolver)
}
