#![allow(clippy::new_ret_no_self)]
use ramp::prism;
use prism::IS_MOBILE;

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

mod wallet;
use crate::wallet::{WalletService, WalletTx};
mod state;
pub use state::*;

use chk::{
    RootInfo, FormItem, NumberVariant, Flow, Bumper, ChkTheme, //AvatarIconStyle,
    Display, Offset, Context, Screen, PageType, PageBuilder, Icons, //AvatarContent,
    Color, Theme, Form, Root, State, Review, Success, Message, Profile, FormSubmit,
    Timestamp, ListItem, Action, TableItem
};

// use chk::items::{ListItem, Action, TableItem};

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
            RootInfo::icon(ctx, theme, Icons::Wallet, "Bitcoin", BitcoinHome::new(theme, &self.wallet)),
            RootInfo::icon(ctx, theme, Icons::Messages, "Messages", MessagesHome::new(theme)),
            // RootInfo::avatar(ctx, theme, AvatarContent::icon(Icons::Profile, AvatarIconStyle::Secondary), "Profile", MessagesHome::new(theme))
        ]
    }

    fn theme(&self) -> ChkTheme {ChkTheme::Dark(Color::from_hex("#eb343a", 255))}
}

#[derive(Debug, Clone)]
pub struct BitcoinHome;

impl BitcoinHome {
    fn new(theme: &Theme, wallet: &Arc<Mutex<WalletService>>) -> PageType {
        let price = wallet.lock().unwrap().price().unwrap();
        let items = wallet.lock().unwrap().transactions().ok().unwrap().into_iter().map(|t| {
            let title = if t.received { "Received bitcoin" } else { "Sent bitcoin" };
            let subtitle = Timestamp::new(t.timestamp.map(|dt| dt.into())).friendly();

            let usd = t.amount.usd(price);
            let view = vec![Screen::new_builder(theme, ViewTransaction::new(price, t))];
            ListItem::plain(title, &subtitle, Some(&usd), Some(Flow::new(view)))
        }).collect::<Vec<_>>();

        let send = SendFlow::new(theme, wallet);
        let receive = vec![Screen::new_builder(theme, Receive::new(wallet))];
        

        let mut wallet = wallet.lock().unwrap();
        let price = wallet.price().unwrap();
        let balance = wallet.balance().unwrap();
        Root::new("Wallet",
            vec![
                Display::currency(balance.usd_f32(price), &balance.btc()),
                Display::list(None, items, None),
            ], //Some(Flow::new(vec![Screen::new_builder(builder, TaskDetails)])))],
            None, ("Receive".into(), Flow::new(receive)), Some(("Send".into(), Flow::from_form(send))),
        )
    }
}

pub struct Receive;
impl Receive {
    pub fn new(wallet: &Arc<Mutex<WalletService>>) -> Box<dyn PageBuilder> {
        let wallet = wallet.clone();
        Box::new(move |_: &Theme| {
            let address = wallet.lock().unwrap().next_address().expect("Could not next address").to_qr_uri();
            PageType::display(
                "Receive bitcoin",
                vec![Display::qr_code(&address, "Scan to receive bitcoin.")],
                None,
                Bumper::custom(
                    if IS_MOBILE {"Share Address"} else {"Copy Address"}, 
                    if IS_MOBILE {Action::share(&address)} else {Action::copy(&address)}
                ),
                Offset::Center,
            )
        })
    }
}

pub struct ViewTransaction;
impl ViewTransaction {
    pub fn new(price: f64, transaction: WalletTx) -> Box<dyn PageBuilder> {
        Box::new(move |_: &Theme| {
            let transaction = transaction.clone();
            let timestamp = Timestamp::new(transaction.timestamp.map(|dt| dt.into()));

            let items = match transaction.received {
                true => vec![
                    TableItem::new("Date", &timestamp.date()),
                    TableItem::new("Time", &timestamp.time()),
                    TableItem::new("Received at address", &transaction.address_short.unwrap()),
                    TableItem::new("Bitcoin received", &transaction.amount.btc()),
                    TableItem::new("Bitcoin price", &wallet::Amount::usd_from_f32(transaction.btc_price_usd.unwrap() as f32)),
                    TableItem::new("Amount received", &transaction.amount.usd(price))
                ],
                false => vec![
                    TableItem::new("Date", &timestamp.date()),
                    TableItem::new("Time", &timestamp.time()),
                    TableItem::new("Sent to address", &transaction.address_short.unwrap_or_default()),
                    TableItem::new("Bitcoin sent", &transaction.amount.btc()),
                    TableItem::new("Bitcoin price", &wallet::Amount::usd_from_f32(transaction.btc_price_usd.unwrap() as f32)),
                    TableItem::new("Amount sent", &transaction.amount.usd(price)),
                    TableItem::new("Network fee", &transaction.fee.unwrap().usd(price)),
                    TableItem::new("Total", &(transaction.fee.unwrap() + transaction.amount).usd(price))
                ]
            };

            PageType::display(
                &format!("{} bitcoin", if transaction.received {"Received"} else {"Sent"}),
                vec![
                    Display::currency(transaction.amount.usd_f32(price), &transaction.amount.btc()),
                    Display::table("Transaction details", items),
                ],
                None,
                Bumper::Done,
                Offset::Start,
            )
        })
    }
}

pub struct SendFlow;
impl SendFlow {
    pub fn new(theme: &Theme, wallet: &Arc<Mutex<WalletService>>) -> Form {
        let w = wallet.clone();
        let price = wallet.lock().unwrap().price().unwrap();
        let (low, high) = wallet.lock().unwrap().required().unwrap();

        let closure = Box::new(move |_: &mut Context, objects: &Vec<State>| {
            let State::Text(address) = objects[0].clone() else { panic!("No Address"); };
            let State::Number(amount_input) = objects[1].clone() else { panic!("No Amount"); };
            let State::Enumerator(priority) = objects[2].clone() else { panic!("No Priority"); };

            let usd = amount_input.trim_start_matches('$').parse::<f64>().unwrap_or_default();
            let amount_btc = bitcoin::Amount::from_sat(((usd / price) * 100_000_000.0).round() as u64);

            let fee_rate = match priority { 0 => 2, _ => 5 };
            let result = w.clone().lock().unwrap().send_to_address(&address, amount_btc.to_sat(), fee_rate);

            match result {
                Ok(txid) => println!("broadcasted tx: {txid}"),
                Err(err) => eprintln!("send failed: {err}"),
            }
        }) as Box<dyn FormSubmit>;

        println!("On submit created.");

        let w = wallet.clone();
        let review = move |objects: &Vec<State>| {
            let State::Text(address) = objects[0].clone() else { panic!("No Address"); };
            let State::Number(amount_input) = objects[1].clone() else { panic!("No Amount"); };
            let State::Enumerator(priority) = objects[2].clone() else { panic!("No Priority"); };

            let usd = amount_input.trim_start_matches('$').parse::<f64>().unwrap_or_default();
            let amount = wallet::Amount::new(bitcoin::Amount::from_sat(((usd / price) * 100_000_000.0).round() as u64));

            let (low, high) = w.clone().lock().unwrap().estimate_fees(address.to_string(), amount).unwrap();

            let (speed_label, fee) = match priority {
                1 => ("Priority (~30 minutes)", high),
                _ => ("Standard (~2 hours)", low),
            };

            vec![
                Display::review("Confirm address", &address, "Bitcoin sent to the wrong address can never be recovered."),
                Display::table("Confirm amount", vec![
                    TableItem::new("Bitcoin sent", &amount.btc()),
                    TableItem::new("Send speed", speed_label),
                    TableItem::new("Amount sent", &amount.usd(price)),
                    TableItem::new("Transaction fee", &fee.usd(price)),
                    TableItem::new("Transaction total", &(amount + fee).usd(price)),
                ]),
            ]
        };

        let success = |objects: Vec<State>| {
            let amount = if let State::Number(x) = &objects[1] {x} else {"$0.00"};
            let usd = amount.trim_start_matches('$').parse::<f64>().unwrap_or_default();
            (Icons::Bitcoin, format!("You sent ${:.2}", usd))
        };
        
        let w = wallet.clone();
        Form::new(theme, vec![
            FormItem::text("Bitcoin address", Some(vec![
                ("Paste clipboard".to_string(), Icons::Paste, Action::Paste),
                ("Scan QR code".to_string(), Icons::QrCode, Action::scan_qr(theme)),
            ]), |a: String| WalletService::ui_valid_address(&a)),
            FormItem::number("Bitcoin amount", NumberVariant::Currency, move |a: String| w.clone().lock().unwrap().ui_can_afford(a)),
            FormItem::enumerator("Transaction speed", vec![
                ("Standard", &format!("Arrives in ~2 hours\n{} bitcoin network fee", low.usd(price))),
                ("Priority", &format!("Arrives in ~30 minutes\n{} bitcoin network fee", high.usd(price))),
            ]),
        ], Some(Review::new("Confirm send", review)), Some(Success::new("Bitcoin sent", success)), closure)
    }
}

#[derive(Debug, Clone)]
pub struct MessagesHome;
impl MessagesHome {
    fn new(theme: &Theme) -> PageType {
        let messages = Message::tests();
        let message = messages[0].clone();
        let chat = vec![Screen::new_builder(theme, Chat::new(messages))];
        let items = vec![ListItem::avatar(message.author.avatar(), &message.author.name, &message.message, None, Some(Flow::new(chat)))];

        Root::new(
            "Messages", vec![Display::list(None, items, Some("No messages yet.\nGet started by messaging a friend."))], None, 
            ("New Message".into(), Flow::from_form(NewMessageFlow::new(theme))), None,
        )
    }
}

pub struct NewMessageFlow;
impl NewMessageFlow {
    pub fn new(theme: &Theme) -> Form {
        let closure = Box::new(move |_ctx: &mut Context, objects: &Vec<State>| println!("New Message {:?}", objects)) as Box<dyn FormSubmit>;
        let items = Profile::more_tests().into_iter().map(|profile| ListItem::avatar(profile.avatar(), &profile.name, "did address here", None, None)).collect::<Vec<_>>();
        Form::new(theme, vec![FormItem::search("Select recipient", items)], None, None, closure)
    }
}

pub struct Chat;
impl Chat {
    pub fn new(messages: Vec<Message>) -> Box<dyn PageBuilder> {
        Box::new(move |_builder: &Theme| {
            let profiles = messages.clone().into_iter().map(|m| m.author).collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();
            PageType::messaging(messages.clone(), profiles)
        })
    }
}