use ic_kit::{Principal, ic};

pub struct Admins(pub Vec<Principal>);

impl Default for Admins {
    fn default() -> Self {
        panic!()
    }
}

pub fn is_admin(id: Principal) -> bool {
    let admins = ic::get::<Admins>().0;
    admins.contains(&id)
}