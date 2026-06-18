#![allow(clippy::new_ret_no_self)]
use ramp::prism;
use prism::IS_MOBILE;

use std::sync::{Arc, Mutex};

use crate::wallet::{self, WalletService, WalletTx};

use chk::{
    FormItem, NumberVariant, Flow, Bumper, FormComplete,
    Display, Offset, Context, PageType, PageBuilder, Icons,
    Theme, Form, Root, State, Review, Success, FormSubmit,
    Timestamp, Action, TableItem, FormValidState,
};

#[derive(Debug, Clone)]
pub struct BitcoinHome;

impl BitcoinHome {
    pub fn new(theme: &Theme, wallet: &Arc<Mutex<WalletService>>) -> Root {
        let _price = wallet.lock().unwrap().price().unwrap();

        let sw = wallet.clone();
        let rw = wallet.clone();
        let send = Box::new(move |_: &mut Context, theme: &Theme| Flow::from_form(SendForm::new(theme, &sw)));
        let receive = Box::new(move |_: &mut Context, theme: &Theme| Flow::new(theme, vec![Receive::new(&rw)]));

        let mut w = wallet.lock().unwrap();
        let price = w.price().unwrap();
        let balance = w.balance().unwrap();

        let _wallet = wallet.clone();
        let _theme = theme.clone();
        Root::new("Wallet",
            vec![
                Display::currency(balance.usd_f32(price), &balance.btc()),
                // Display::list(None, Arc::new(Box::new(move |ctx: &mut Context| {
                //     wallet.lock().unwrap().transactions().ok().unwrap().into_iter().map(|t| {
                //         let title = if t.received { "Received bitcoin" } else { "Sent bitcoin" };
                //         let subtitle = Timestamp::new(t.timestamp.map(|dt| dt.into())).friendly();

                //         let usd = t.amount.usd(price);
                //         let view = Flow::new(&theme.clone(), vec![ViewTransaction::new(price, t)]);
                //         ListItem::plain(title, &subtitle, Some(&usd), Some(view))
                //     }).collect::<Vec<_>>()
                // })), None),
            ], 
            None, ("Receive".into(), receive), Some(("Send".into(), send)),
        )
    }
}

pub struct Receive;
impl Receive {
    pub fn new(wallet: &Arc<Mutex<WalletService>>) -> Box<dyn PageBuilder> {
        let wallet = wallet.clone();
        Box::new(move || {
            let address = wallet.lock().unwrap().next_address().expect("Could not next address").to_qr_uri();
            PageType::display_qr_code("Receive bitcoin", &address, "Scan to receive bitcoin.")
        })
    }
}

pub struct ViewTransaction;
impl ViewTransaction {
    pub fn new(price: f64, transaction: WalletTx) -> Box<dyn PageBuilder> {
        Box::new(move || {
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

pub struct SendForm;
impl SendForm {
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

            FormComplete::None
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
                Display::cta("Confirm address", Some(&address), "Bitcoin sent to the wrong address can never be recovered.", vec![]),
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
        Form::flow(theme, vec![
            FormItem::text("Bitcoin address", Some(vec![
                ("Paste clipboard".to_string(), Icons::Paste, Action::Paste),
                ("Scan QR code".to_string(), Icons::QrCode, Action::scan_qr(theme, "Scan a bitcoin QR code")),
            ]), |_ctx: &mut Context, a: String| FormValidState::from(WalletService::ui_valid_address(&a))),
            FormItem::number("Bitcoin amount", NumberVariant::Currency, move |_ctx: &mut Context, a: String| FormValidState::from(w.clone().lock().unwrap().ui_can_afford(a))),
            FormItem::enumerator("Transaction speed", vec![
                ("Standard", &format!("Arrives in ~2 hours\n{} bitcoin network fee", low.usd(price))),
                ("Priority", &format!("Arrives in ~30 minutes\n{} bitcoin network fee", high.usd(price))),
            ]),
        ], Some(Review::new("Confirm send", review)), Some(Success::new("Bitcoin sent", success)), closure)
    }
}
