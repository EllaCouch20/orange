#![allow(clippy::new_ret_no_self)]
use chk::{FormValidState, Icons, Action, FormComplete, AvatarContent, FormItem, Flow, Display, Context, PageType, PageBuilder, Theme, Form, Root, State, FormSubmit, ListItem};
use chk::air::profiles::Profile;
use air::names::{Id, Name};
use air::Instance;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ContactsHome;
impl ContactsHome {
    pub fn new(_ctx: &mut Context, theme: &Theme) -> Root {
        
        let new_contact = Box::new(|_ctx: &mut Context, theme: &Theme| Flow::from_form(NewContact::new(theme)));
        // let new_contact = Flow::from_form(NewContact::new(theme));
        // let new_contact = Flow::new(&theme, vec![NewContact::new(theme)]);

        let theme = theme.clone();
        Root::new(
            "Contacts", vec![
                Display::list(None, Arc::new(Box::new(move |ctx: &mut Context| {
                    let _me = ctx.me();
                    let mut list = ctx.list::<Profile>();
                    let mut items = list.iter_mut().flat_map(|instance| {
                        let instance_clone = instance.clone();
                        let profile = instance.pending();
                        if profile.name.unwrap() != ctx.me() {
                            let view_contact = Flow::new(&theme, vec![ViewContact::new(ctx, instance_clone)]);
                            Some(ListItem::avatar(profile.avatar.clone(), &profile.username, &profile.name(), None, Some(view_contact)))
                        } else {None}
                    }).collect::<Vec<ListItem>>();
                    items.sort_by(|a, b| a.title.cmp(&b.title));
                    items
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
            let profile = if let Some(State::Text(result)) = objects.first() {
                let name = Name::from_str(result).unwrap();
                Profile::create(ctx, name)
            } else {todo!()};
            FormComplete::Next(ViewContact::new(ctx, profile))
        }) as Box<dyn FormSubmit>;

        Form::flow(theme, vec![
            FormItem::text("Create contact", Some(vec![
                ("Paste clipboard".to_string(), Some("Pasted".to_string()), Icons::Paste, Action::Paste),
                ("Scan QR code".to_string(), None, Icons::QrCode, Action::scan_qr(theme, "Scan a profile QR code")),
            ]), |_ctx: &mut Context, a: String| match a.is_empty() {
                true => FormValidState::Invalid,
                false => match Name::from_str(&a) {
                    Ok(_) => FormValidState::Valid,
                    Err(_) => FormValidState::InvalidWithData("Not a valid Orange Name.".to_string())
                }
            }),
        ], None, None, closure)
    }
}

pub struct ViewContact;
impl ViewContact {
    pub fn new(_ctx: &mut Context, profile: Instance<Profile>) -> Box<dyn PageBuilder> {
        Box::new(move || PageType::profile(profile.clone()))
    }
}
