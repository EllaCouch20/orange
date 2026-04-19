use ramp::prism;

use std::sync::{Arc, Mutex};

mod wallet;
use crate::wallet::WalletService;
mod state;
pub use state::*;

mod ui;

use chk::{ RootInfo, ChkTheme, AvatarIconStyle, Context, Icons, AvatarContent, Color, Theme, Profile };

chk::run! { |_ctx: &mut Context| Orange::new() }

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
        vec![
            RootInfo::icon(ctx, theme, Icons::Wallet, "Bitcoin", ui::BitcoinHome::new(theme, &self.wallet)),
            RootInfo::icon(ctx, theme, Icons::Messages, "Messages", ui::MessagesHome::new(theme)),
            RootInfo::avatar(ctx, theme, AvatarContent::icon(Icons::Profile, AvatarIconStyle::Secondary), "Profile", ui::ProfileHome::new(theme, Profile::me()))
        ]
    }

    fn theme(&self) -> ChkTheme {ChkTheme::Dark(Color::from_hex("#eb343a", 255))}
}
