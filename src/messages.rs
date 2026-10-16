#![allow(clippy::new_ret_no_self)]
use chk::{FormAction, Page, Action, AvatarContent, FormComplete, FormItem, Flow, Display, Context, PageType, Theme, Form, Root, State, FormSubmit, ListItem, AvatarIconStyle, Icons};
use chk::air::profiles::Profile;
use chk::air::messages::{ChatRoom, MessageContent};

use maverick_os::air::{Instance, Name};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MessagesHome;
impl MessagesHome {
    pub fn new(ctx: &mut Context) -> Root {
        Root::custom(Page::updates_list_changes::<ChatRoom>(ctx, |ctx: &mut Context, _theme: &Theme, mut list: Vec<Instance<ChatRoom>>| {
            chk::air::messages::VideoClip::listen(ctx);

            let new_message = Box::new(|ctx: &mut Context, theme: &Theme| Flow::from_form(NewMessageFlow::new(ctx, theme)));
            let mut items = list.iter_mut().map(|instance| {
                let chat = Flow::new(vec![Chat::new(ctx, instance.clone())]);
                let room = instance.pending();

                let mut ts = 0;
                let chat_name = room.name(ctx).to_string();
                let mut chat_avatar = AvatarContent::Icon(Icons::Profile, AvatarIconStyle::Secondary);
                let mut chat_last = "No messages yet.".to_string();

                let members = room.members.clone().into_iter().filter(|n| *n != ctx.me()).collect::<Vec<_>>();

                if members.len() > 1 {
                    chat_avatar = AvatarContent::Icon(Icons::Group, AvatarIconStyle::Secondary);
                } else {
                    if let Some(name) = members.first() {
                        let p = Profile::from_name(ctx, *name);
                        let p = p.pending();
                        chat_avatar = p.avatar.clone();
                    } else {println!("Looks like you are the only one here for now.")}
                }
                
                if let Some(last) = room.messages(ctx.me()).last() {
                    ts = last.timestamp;
                    chat_last = match &last.body {
                        MessageContent::Text(message) => message.to_string(),
                        MessageContent::Sticker(_) => "Sticker".to_string(),
                        MessageContent::Gif(_) => "GIF".to_string(),
                        MessageContent::Photo(_) => "Photo".to_string(),
                        MessageContent::Video(_) => "Video".to_string(),
                    };

                    let recent = Profile::from_name(ctx, last.author);
                    let recent = recent.pending();
                    match last.author == ctx.me() {
                        true => chat_last = format!("You: {}", chat_last),
                        false => chat_last = format!("{}: {}", recent.username, chat_last)
                    }
                }

                let list_item = ListItem::avatar(chat_avatar, &chat_name, &chat_last, None, Some(chat));
                (ts, list_item)
            }).collect::<Vec<(u64, ListItem)>>();
            
            items.sort_by_key(|b| std::cmp::Reverse(b.0));
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
            println!("Creating chatroom");
            
            if let Some(State::Search(recipients)) = objects.iter().find(|s| matches!(s, State::Search(_))) {
                let instance = ChatRoom::create(ctx, recipients.to_vec());
                FormComplete::Next(Page::messaging(ctx, &mut instance.clone()))
            } else {FormComplete::None}

        }) as Box<dyn FormSubmit>;

        let items = ctx.list::<Profile>().iter_mut().flat_map(|(_, p)| {
            let profile = p.pending();
            let username = match profile.name == Some(ctx.me()) {
                true => format!("{} (You)", profile.username),
                false => profile.username.to_string()
            };

            Some((ListItem::avatar(AvatarContent::default(), &username, &profile.name(), None, None), profile.name.unwrap()))
        }).collect::<Vec<_>>();
        Form::flow(theme, vec![FormItem::search("New message", "profile name", items, Some(vec![
            FormAction::Custom("New contact".to_string(), None, Icons::Add, Action::flow(Flow::from_form(CreateAndAddContact::new(theme)))),
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
            let (_, name) = if let Some(State::Text(result)) = objects.first() {
                let name = Name::from_str(result).unwrap();
                (Profile::create(ctx, name), name)
            } else {todo!()};
            FormComplete::Return(Box::new(move |ctx: &mut Context, theme: &Theme| {
                (Action::choose_search(name).get())(ctx, theme)
            }))
        }) as Box<dyn FormSubmit>;

        Form::flow(theme, vec![
            FormItem::text("New contact", "orange name", 
            Some(vec![FormAction::Paste, FormAction::scan_qr_code("Scan a profile QR code")]), 
            |_ctx: &mut Context, a: String| match a.is_empty() {
                true => FormValidState::Invalid,
                false => match Name::from_str(&a) {
                    Ok(_) => FormValidState::Valid,
                    Err(_) => FormValidState::InvalidWithData("Not a valid Orange Name.".to_string())
                }
            }),
        ], None, None, closure)
    }
}
