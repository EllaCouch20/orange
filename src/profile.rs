#![allow(clippy::new_ret_no_self)]

use chk::{Page, Context, State, FormSubmit, FormItem, Display, Icons, Root, Action, Theme, PageType};
use chk::air::profiles::{Profile, ChangeNotes, ChangeUsername};

pub struct ProfileHome;
impl ProfileHome {
    pub fn new(ctx: &mut Context, _theme: &Theme) -> Root {
        Root::custom(Page::profile(&mut Profile::me(ctx)))
    }
}