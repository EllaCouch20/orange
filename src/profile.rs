#![allow(clippy::new_ret_no_self)]

use chk::{Page, Context, Root};
use chk::air::profiles::Profile;

pub struct ProfileHome;
impl ProfileHome {
    pub fn new(ctx: &mut Context) -> Root {
        let mut me = Profile::me(ctx);
        Root::custom(Page::profile(ctx, &mut me))
    }
}