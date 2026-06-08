use ramp::prism;

use std::sync::{Arc, Mutex};

mod wallet;
use crate::wallet::WalletService;
mod state;
pub use state::*;
mod bitcoin;
mod messages;
mod contacts;
mod profile;

use chk::{ RootInfo, ChkTheme, AvatarIconStyle, Context, Icons, AvatarContent, Color, Theme };
use chk::air::messages::{ChatRoom, Message};

use std::time::Duration;

chk::run! {|ctx: &mut Context| Orange::new(ctx) }

// pub struct MessagesListener(Vec<Message>);
// impl Service for MessagesListener {
//     async fn run(&mut self, air: &mut crate::maverick_os::air::Air) -> Option<Duration> {
//         let mut new_messages: Vec<Message> = Vec::new();
//         air.list::<ChatRoom>().ok().and_then(|l| l.iter().for_each(|room_id| {
//             if let Some(Substance::Seq(messages)) = air.get_pending::<ChatRoom, _>(room_id, "/messages") {
//                 messages.into_iter().for_each(|m| new_messages.push(Message::from_substance(m)));
//             }
//         }));

//         if new_messages != self.0 {
//             for message in new_messages.iter().filter(|m| !self.messages.contains(m)) {
//                 println!("NEW MESSAGE: {:?}", message);
//                 ctx.push_notification("New Message", "You received a new message.");
//             }

//             self.0 = new_messages;
//         }

//         Some(Duration::from_millis(500))
//     }
// }

pub struct Orange { wallet: Arc<Mutex<WalletService>> }
// impl Default for Orange { fn default() -> Self {Orange::new()} }

impl Orange {
    pub fn new(ctx: &mut Context) -> Self {
        // std::thread::spawn(|| {
        //     loop {
        //         println!("APP HEARTBEAT {:?}", std::time::SystemTime::now());
        //         std::thread::sleep(std::time::Duration::from_secs(5));
        //     }
        // });

        let wallet = WalletService::new().expect("failed to create wallet");
        
        Orange { wallet: Arc::new(Mutex::new(wallet)) }
    }
}

impl chk::App for Orange {
    fn roots(&self, ctx: &mut Context, theme: &Theme) -> Vec<RootInfo> {
        let messages_home = messages::MessagesHome::new(ctx, theme);
        let contacts_home = contacts::ContactsHome::new(ctx, theme);
        let profile_home = profile::ProfileHome::new(ctx, theme);
        vec![
            RootInfo::icon(ctx, theme, Icons::Wallet, "Bitcoin", bitcoin::BitcoinHome::new(theme, &self.wallet)),
            RootInfo::icon(ctx, theme, Icons::Messages, "Messages", messages_home),
            RootInfo::icon(ctx, theme, Icons::Group, "Contacts", contacts_home),
            RootInfo::avatar(ctx, theme, AvatarContent::icon(Icons::Profile, AvatarIconStyle::Secondary), "Profile", profile_home)
        ]
    }

    fn theme(&self) -> ChkTheme {ChkTheme::Dark(Color::from_hex("#eb343a", 255))}
    // fn on_event(&mut self, ctx: &mut Context, event: Box<dyn Event>) -> Vec<Box<dyn Event>> {}
}