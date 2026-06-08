#![allow(clippy::new_ret_no_self)]

use chk::{Context, State, FormSubmit, FormItem, Display, Icons, Root, Action, Theme, PageType};
use chk::air::profiles::{Profile, ChangeNotes, ChangeUsername};

pub struct ProfileHome;
impl ProfileHome {
    pub fn new(ctx: &mut Context, _theme: &Theme) -> Root {
        Root::custom(PageType::profile(Profile::me(ctx)))
    }
}