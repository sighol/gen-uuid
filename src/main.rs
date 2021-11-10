// Note that this requires the `v4` feature enabled in the uuid crate.

use uuid::Uuid;

fn main() {
    let my_uuid = Uuid::new_v4();
    println!("{}", my_uuid);
}