use harmonius_primitives::{Handle, HandleMap};

struct Mesh;
struct Texture;

fn main() {
    let mut map = HandleMap::<Texture>::new();
    let h: Handle<Mesh> = Handle::NULL;
    let _ = map.get(h);
}
