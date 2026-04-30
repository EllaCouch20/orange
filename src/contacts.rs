#![allow(clippy::new_ret_no_self)]
use chk::{Icons, Action, Bumper, Offset, AvatarContent, FormItem, Flow, Display, Context, PageType, PageBuilder, Theme, Form, Root, State, FormSubmit, ListItem};
use chk::messages::{Profile, ChatRoom, AddMember, Contact, Username};

use air::names::{Secret, Id, Name};
use air::contract::{Contracts, Contract, Substance, Reactants, Reactant, Beaker};

use std::str::FromStr;
use std::collections::BTreeMap;
use std::path::{PathBuf, Path};
use std::convert::Infallible;
use std::sync::{Arc, Mutex};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct ContactsHome;
impl ContactsHome {
    pub fn new(ctx: &mut Context, theme: &Theme) -> Root {
        
        let new_contact = Flow::from_form(NewContactFlow::new(theme));

        let theme = theme.clone();
        Root::display(
            "Contacts", vec![
                Display::list(None, Arc::new(Box::new(move |ctx: &mut Context| {
                    let ids = ctx.list::<Contact>();

                    ids.iter().map(|id| {
                        let name = if let Some(Substance::String(name)) = ctx.get::<Contact, _>(&id, "/name") {
                            println!("Username {:?}", name);
                            name
                        } else {"Friend".to_string()};
                        // let messages = ctx.get::<ChatRoom, _>(&id, "/messages").unwrap_or_default();
                        // println!("Messages {:?}", messages.query("/body"));
                        // println!("Members {:?}", members);
                        let view_contact = Flow::new(&theme, vec![ViewContact::new(id.clone())]);
                        // let author = Profile::new(Secret::new().name());
                        // match members.get(0) {
                        //     Some(author) => ListItem::avatar(author.avatar, &author.username, "No message.", None, Some(chat)),
                        //     None => ListItem::avatar(AvatarContent::default(), "Group Message", "No message.", None, Some(chat)),
                        // }

                        ListItem::avatar(AvatarContent::default(), &name, &id.to_string(), None, Some(view_contact))
                        
                    }).collect::<Vec<ListItem>>()
                })), Some("No contacts yet.\nGet started by adding a friend.")),
            ], None, 
            ("New Contact".into(), new_contact), None,
        )
    }

    // fn update(&self, ctx: &mut Context) -> bool {
    //     self.0 != ctx.list::<ChatRoom>()
    // }
}

pub struct NewContactFlow;
impl NewContactFlow {
    pub fn new(theme: &Theme) -> Form {
        let closure = Box::new(move |ctx: &mut Context, objects: &Vec<State>| {
            if let Some(State::Text(result)) = objects.get(0) {
                let name = Name::from_str(result).unwrap();
                let username = if let Some(State::Text(username)) = objects.get(1) {username.to_string()} else {Username::new()};
                let notes = if let Some(State::Text(notes)) = objects.get(2) {notes.to_string()} else {String::new()};
                let i = ctx.create(Contact::new(name, username, notes)).unwrap();
                println!("Created contact under {i}")
            }
            // let profile = Profile::new();
            // let recipient = Secret::new().name();
            // let id = ctx.create(ChatRoom::new()).unwrap();
            // ctx.send(id, "/members", AddMember(recipient)).unwrap();
            // ctx.share::<ChatRoom>(id, recipient);

            // println!("Created contact for {:?}", objects);
        }) as Box<dyn FormSubmit>;
        // let items = Profile::more_tests().into_iter().map(|proflie| ListItem::avatar(profile.avatar(), &profile.name, "did address here", None, None)).collect::<Vec<_>>();
        // let profiles = vec![Profile::new(Secret::new().name()), Profile::new(Secret::new().name())];
        // let items = profiles.into_iter().map(|profile| ListItem::avatar(profile.avatar, &profile.username, &profile.name.to_string(), None, None)).collect::<Vec<_>>();
        Form::new(theme, vec![
            FormItem::text("Orange name", Some(vec![
                ("Paste clipboard".to_string(), Icons::Paste, Action::Paste),
                ("Scan QR code".to_string(), Icons::QrCode, Action::scan_qr(theme)),
            ]), |a: String| match a.is_empty() {
                true => Err(String::new()),
                false => Name::from_str(&a).map(|_| String::new()).map_err(|e| "Not a valid Orange Name.".to_string()),
            }),
            FormItem::text("Contact name", Some(vec![
                ("Paste clipboard".to_string(), Icons::Paste, Action::Paste),
                ("Scan QR code".to_string(), Icons::QrCode, Action::scan_qr(theme)),
            ]), |a: String| Ok(a)),
            FormItem::text("Notes", Some(vec![
                ("Paste clipboard".to_string(), Icons::Paste, Action::Paste),
            ]), |a: String| Ok(a)),
            FormItem::avatar("Avatar"),
        ], None, None, closure)
    }
}


pub struct ViewContact;
impl ViewContact {
    pub fn new(contact_id: Id) -> Box<dyn PageBuilder> {
        Box::new(move || {
            let profile = Profile::new(Secret::new().name());
            PageType::display(
                &profile.username,
                vec![
                    Display::avatar(profile.avatar.clone()),
                    Display::cta("About me", None, "Bitcoin sent to the wrong address can never be recovered.", vec![]), //copy, edit
                    Display::cta("Decentralized ID", None, &contact_id.to_string(), vec![]), //copy, edit
                    Display::cta("Bitcoin address", None, "12FWmGPUCtFeZECFydRARUzfqt7h2GBqEL", vec![]), //copy, edit
                ],
                None,
                Bumper::Done,
                Offset::Start,
            )
        })
    }
}