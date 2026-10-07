#![allow(clippy::new_ret_no_self)]

use chk::{Page, Context, State, FormSubmit, FormItem, Display, Icons, Root, Action, Theme, PageType};
use chk::air::profiles::{Profile, ProfileAction};

pub struct ProfileHome;
impl ProfileHome {
    pub fn new(ctx: &mut Context, theme: &Theme) -> Root {
        let mut me = Profile::me(ctx);
        Root::custom(Page::profile(ctx, theme, &mut me))
    }
}