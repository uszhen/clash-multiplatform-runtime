extern crate embed_resource;

fn main() {
    embed_resource::compile("extra/app.rc", embed_resource::NONE);
}
