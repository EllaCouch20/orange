#![allow(clippy::new_ret_no_self)]
use chk::{Icons, Input, Action, Bumper, Offset, AvatarContent, AvatarIconStyle, FormItem, Flow, Display, Context, PageType, PageBuilder, Theme, Form, Root, State, FormSubmit, ListItem};
use chk::messages::{Profile, ChatRoom, AddMember, Contact, Username, ChangeUsername, ChangeNotes};

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
        
        let new_contact = Box::new(|ctx: &mut Context, theme: &Theme| Flow::from_form(NewContact::new(theme)));
        // let new_contact = Flow::from_form(NewContact::new(theme));
        // let new_contact = Flow::new(&theme, vec![NewContact::new(theme)]);

        let theme = theme.clone();
        Root::new(
            "Contacts", vec![
                Display::list(None, Arc::new(Box::new(move |ctx: &mut Context| {
                    let ids = ctx.list::<Contact>();
                    let me = ctx.me();
                    ids.iter().flat_map(|id| {
                        let profile = Profile::from_id(ctx, *id);
                        if profile.name.unwrap() != ctx.me() {
                            let view_contact = Flow::new(&theme, vec![ViewContact::new(ctx, profile.clone(), id.clone())]);
                            Some(ListItem::avatar(AvatarContent::default(), &profile.username, &profile.name(), None, Some(view_contact)))
                        } else {None}
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

pub struct NewContact;
impl NewContact {
    pub fn new(theme: &Theme) -> Form {
        let closure = Box::new(move |ctx: &mut Context, objects: &Vec<State>| {
            let (profile, id) = if let Some(State::Text(result)) = objects.get(0) {
                let name = Name::from_str(result).unwrap();
                let profile = Profile::create(name);
                let id = ctx.create(Contact::new(name, profile.username.to_string(), profile.notes.to_string())).unwrap();
                (profile, id)
            } else {todo!()};
            Some(ViewContact::new(ctx, profile, id))
        }) as Box<dyn FormSubmit>;

        Form::flow(theme, vec![
            FormItem::text("Orange name", Some(vec![
                ("Paste clipboard".to_string(), Icons::Paste, Action::Paste),
                ("Scan QR code".to_string(), Icons::QrCode, Action::scan_qr(theme, "Scan a profile QR code")),
            ]), |ctx: &mut Context, a: String| match a.is_empty() {
                true => Err(String::new()),
                false => Name::from_str(&a).map(|_| String::new()).map_err(|e| "Not a valid Orange Name.".to_string()),
            }),
        ], None, None, closure)
    }
}

pub struct ViewContact;
impl ViewContact {
    pub fn new(ctx: &mut Context, profile: Profile, contact_id: Id) -> Box<dyn PageBuilder> {
        Box::new(move || {
            let profile = profile.clone();
            let closure = Box::new(move |ctx: &mut Context, objects: &Vec<State>| {
                if let Some(State::Text(result)) = objects.get(1) {
                    let _ = ctx.send(contact_id, "/username", ChangeUsername(result.to_string()));
                }
                if let Some(State::Text(result)) = objects.get(2) {
                    let _ = ctx.send(contact_id, "/notes", ChangeNotes(result.to_string()));
                }
                None
            }) as Box<dyn FormSubmit>;
            let name = profile.name.unwrap().clone();
            PageType::edit_and_display(
                "View contact",
                vec![
                    FormItem::avatar_with_preset("Avatar", profile.avatar),
                    FormItem::text_with_preset("Username", &profile.username.clone(), None, move |ctx: &mut Context, a: String| {
                        match a.is_empty() {
                            true => Err("Username cannot be empty".to_string()),
                            false => {
                                if let Some(current) = Profile::from_name(ctx, name.clone()) {
                                    match current.username == a {
                                        true => Err(String::new()),
                                        false => Ok(a.to_string())
                                    }
                                } else {Ok(a.to_string())}
                            }
                        }
                    }),
                    FormItem::text_with_preset("About me", &profile.notes, None, |ctx: &mut Context, a: String| Ok(a.to_string())),
                ],
                vec![
                    Display::cta("Orange name", None, &profile.name.unwrap().to_string(), vec![("Copy".to_string(), Icons::Copy, Action::copy(&profile.name.unwrap().to_string()))]),
                ],
                closure
            )
        })
    }
}