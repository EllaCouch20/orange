use ramp::prism;

use std::sync::{Arc, Mutex};

mod wallet;
use crate::wallet::WalletService;
mod state;
pub use state::*;
mod bitcoin;
mod messages;
mod profile;

use chk::{ RootInfo, ChkTheme, AvatarIconStyle, Context, Icons, AvatarContent, Color, Theme };

chk::run! {[messages::ChatRoom]; |_ctx: &mut Context| Orange::new() }

pub struct Orange { wallet: Arc<Mutex<WalletService>> }
impl Default for Orange { fn default() -> Self {Orange::new()} }

impl Orange {
    pub fn new() -> Self {
        let wallet = WalletService::new().expect("failed to create wallet");
        Orange { wallet: Arc::new(Mutex::new(wallet)) }
    }
}

impl chk::App for Orange {
    fn roots(&self, ctx: &mut Context, theme: &Theme) -> Vec<RootInfo> {
        let messages_home = messages::MessagesHome::new(ctx, theme);
        vec![
            RootInfo::icon(ctx, theme, Icons::Wallet, "Bitcoin", bitcoin::BitcoinHome::new(theme, &self.wallet)),
            RootInfo::icon(ctx, theme, Icons::Messages, "Messages", messages_home),
            RootInfo::avatar(ctx, theme, AvatarContent::icon(Icons::Profile, AvatarIconStyle::Secondary), "Profile", profile::ProfileHome::new(theme))
        ]
    }

    fn theme(&self) -> ChkTheme {ChkTheme::Dark(Color::from_hex("#eb343a", 255))}
}
