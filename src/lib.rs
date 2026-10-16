use ramp::prism;

// mod wallet;
// use crate::wallet::WalletService;
mod state;
pub use state::*;
// mod bitcoin;
mod messages;
mod contacts;
mod profile;

use chk::{ RootInfo, ChkTheme, AvatarIconStyle, Context, Icons, AvatarContent, Color, Theme };

chk::run! {[], |_ctx: &mut Context| {
    chk::set_giphy_api_key("RL2FFrfnJ8J6jcb8XRZ4ImqkFLrpW5Tt");
    Orange::new() 
}}

#[derive(Default)]
pub struct Orange;
impl Orange { pub fn new() -> Self {Orange} }

impl chk::App for Orange {
    fn roots(&self, ctx: &mut Context, theme: &Theme) -> Vec<RootInfo> {
        let messages_home = messages::MessagesHome::new(ctx);
        let contacts_home = contacts::ContactsHome::new(ctx);
        let profile_home = profile::ProfileHome::new(ctx);
        vec![
            // RootInfo::icon(ctx, theme, Icons::Wallet, "Bitcoin", bitcoin::BitcoinHome::new(theme, &self.wallet)),
            RootInfo::icon(ctx, theme, Icons::Messages, "Messages", messages_home),
            RootInfo::icon(ctx, theme, Icons::Group, "Contacts", contacts_home),
            RootInfo::avatar(ctx, theme, AvatarContent::icon(Icons::Profile, AvatarIconStyle::Secondary), "Profile", profile_home)
        ]
    }

    fn theme(&self) -> ChkTheme {ChkTheme::Dark(Color::from_hex("#eb343a", 255))}
    // fn on_event(&mut self, ctx: &mut Context, event: Box<dyn Event>) -> Vec<Box<dyn Event>> {}
}