#![allow(clippy::new_ret_no_self)]
use chk::{Page, Action, AvatarContent, FormComplete, FormItem, Flow, Display, Context, PageType, PageBuilder, Theme, Form, Root, State, FormSubmit, ListItem, AvatarIconStyle, Icons};
use chk::air::profiles::Profile;
use chk::air::messages::{ChatRoom, AddMember, Message};

use air::Instance;
use air::names::{Id, Name};
use std::sync::Arc;
use std::str::FromStr;

use crate::contacts::NewContact;


#[derive(Debug, Clone)]
pub struct MessagesHome;
impl MessagesHome {
    pub fn new(ctx: &mut Context, theme: &Theme) -> Root {
        Root::custom(Page::updates_list_changes::<Profile>(ctx, |ctx: &mut Context, theme: &Theme, mut list: Vec<Instance<Profile>>| {
            let new_message = Box::new(|ctx: &mut Context, theme: &Theme| Flow::from_form(NewMessageFlow::new(ctx, theme)));
            let theme = theme.clone();
            let mut items = ctx.list::<ChatRoom>().iter_mut().map(|(_, instance)| {
                let chat = Flow::new(vec![Chat::new(ctx, instance.clone())]);
                let room = instance.load_pending();

                let mut ts = 0;
                let chat_name = room.name(ctx).to_string();
                let mut chat_avatar = AvatarContent::Icon(Icons::Profile, AvatarIconStyle::Secondary);
                let mut chat_last = "No messages yet.".to_string();

                let members = room.members.clone().into_iter().filter(|n| *n != ctx.me()).collect::<Vec<_>>();

                if members.len() > 1 {
                    chat_avatar = AvatarContent::Icon(Icons::Group, AvatarIconStyle::Secondary);
                } else {
                    if let Some(name) = members.first() {
                        let mut p = Profile::from_name(ctx, *name);
                        let p = p.load_pending();
                        chat_avatar = p.avatar.clone();
                    } else {println!("Looks like you are the only one here for now.")}
                }
                
                if let Some(last) = room.messages.last() {
                    ts = last.timestamp;
                    chat_last = last.body.to_string();
                    let mut recent = Profile::from_name(ctx, last.author);
                    let recent = recent.load_pending();
                    match last.author == ctx.me() {
                        true => chat_last = format!("You: {}", chat_last),
                        false => chat_last = format!("{}: {}", recent.username, chat_last)
                    }
                }

                let list_item = ListItem::avatar(chat_avatar, &chat_name, &chat_last, None, Some(chat));
                (ts, list_item)
            }).collect::<Vec<(u64, ListItem)>>();
            
            items.sort_by(|a, b| b.0.cmp(&a.0));
            let items = items.into_iter().map(|(_, item)| item).collect::<Vec<ListItem>>();
            
            PageType::root("Messages", vec![], vec![
                Display::list(None, items, Some("No messages yet.\nGet started by messaging a friend.")),
            ], None, Some(("New Message".into(), new_message)), None,)
        }))
    }
}

pub struct NewMessageFlow;
impl NewMessageFlow {
    pub fn new(ctx: &mut Context, theme: &Theme) -> Form {
        let closure = Box::new(move |ctx: &mut Context, objects: &Vec<State>| {
            let mut instance = ctx.create::<ChatRoom>(Id::random());
            
            if let Some(State::Search(result)) = objects.iter().find(|s| matches!(s, State::Search(_))) {
                result.iter().for_each(|recipient| {
                    instance.apply(AddMember(*recipient));
                    instance.share(*recipient);
                    println!("Created room with members {:?}", recipient);
                })
            }

            FormComplete::Next(Chat::new(ctx, instance))
        }) as Box<dyn FormSubmit>;

        let items = ctx.list::<Profile>().iter_mut().flat_map(|(_, p)| {
            if p.load_pending().name.unwrap() != ctx.me() {
                let profile = p.load_pending();
                Some((ListItem::avatar(AvatarContent::default(), &profile.username, &profile.name(), None, None), profile.name.unwrap()))
            } else {
                None
            }
        }).collect::<Vec<_>>();
        Form::flow(theme, vec![FormItem::search("Select recipient", items, Some(vec![
            ("New contact".to_string(), None, Icons::Add, Action::flow(Flow::from_form(CreateAndAddContact::new(theme)))),
        ]))], None, None, closure)
    }
}

pub struct Chat;
impl Chat {
    pub fn new(ctx: &mut Context, mut instance: Instance<ChatRoom>) -> Page {
        Page::messaging(ctx, &mut instance)
    }
}

use chk::FormValidState;
pub struct CreateAndAddContact;
impl CreateAndAddContact {
    pub fn new(theme: &Theme) -> Form {
        let closure = Box::new(move |ctx: &mut Context, objects: &Vec<State>| {
            let (profile, name) = if let Some(State::Text(result)) = objects.first() {
                let name = Name::from_str(result).unwrap();
                (Profile::create(ctx, name), name)
            } else {todo!()};
            FormComplete::Return(Box::new(move |ctx: &mut Context, theme: &Theme| {
                (Action::choose_search(name.clone()).get())(ctx, theme)
            }))
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
