#![allow(clippy::new_ret_no_self)]
use chk::{AvatarContent, FormItem, Flow, Display, Context, PageType, PageBuilder, Theme, Form, Root, State, FormSubmit, ListItem, AvatarIconStyle, Icons};
use chk::air::profiles::{Profile, Contact};
use chk::air::messages::{ChatRoom, AddMember};

use air::names::{Id, Name};
use air::contract::{Substance, Beaker};

use std::sync::Arc;
use std::str::FromStr;


#[derive(Debug, Clone)]
pub struct MessagesHome;
impl MessagesHome {
    pub fn new(_ctx: &mut Context, theme: &Theme) -> Root {
        let new_message = Box::new(|ctx: &mut Context, theme: &Theme| Flow::from_form(NewMessageFlow::new(ctx, theme)));
        let theme = theme.clone();
        Root::new(
            "Messages", vec![
                Display::list(None, Arc::new(Box::new(move |ctx: &mut Context| {
                    let ids = ctx.list::<ChatRoom>();
                    let mut items = ids.iter().map(|id| {
                        let mut ts = 0;
                        let mut chat_name = "New Message".to_string();
                        let mut chat_avatar = AvatarContent::Icon(Icons::Profile, AvatarIconStyle::Secondary);
                        let mut chat_last = "No messages yet.".to_string();

                        let mut members = vec![];

                        if let Some(Substance::Seq(names)) = ctx.get::<ChatRoom, _>(id, "/members") {
                            names.into_iter().for_each(|name| {
                                if let Substance::String(n) = name {
                                    members.push(Name::from_str(&n).unwrap());
                                }
                            })
                        }

                        if let Some(Substance::String(n)) = ctx.get::<ChatRoom, _>(id, "/author") {
                            members.push(Name::from_str(&n).unwrap());
                        }

                        let members = members.into_iter().filter(|n| *n != ctx.me()).collect::<Vec<_>>();

                        if members.len() > 1 {
                            chat_name = "Group Message".to_string();
                            chat_avatar = AvatarContent::Icon(Icons::Group, AvatarIconStyle::Secondary);
                        } else {
                            if let Some(name) = members.first() {
                                let (p, _) = Profile::from_name(ctx, *name);
                                chat_name = p.username.to_string();
                                chat_avatar = p.avatar;
                            } else {println!("Looks like you are the only one here for now.")}
                        }
                        

                        if let Some(Substance::Seq(messages)) = ctx.get::<ChatRoom, _>(id, "/messages")
                        && let Some(last) = messages.last()
                        && let Ok(Substance::String(message)) = last.query("/body") {
                            if let Ok(Substance::Integer(timestamp)) = last.query("/timestamp") { ts = timestamp; }
                            chat_last = message.to_string();
                            if let Some(recent) = Profile::from_substance(ctx, &last.query("/author").unwrap()) {
                                match recent == Profile::me(ctx) {
                                    true => chat_last = format!("You: {}", message),
                                    false => chat_last = format!("{}: {}", recent.0.username, message)
                                }
                            }
                        }

                        let chat = Flow::new(&theme, vec![Chat::new(*id)]);
                        let list_item = ListItem::avatar(chat_avatar, &chat_name, &chat_last, None, Some(chat));
                        (ts, list_item)
                    }).collect::<Vec<(i64, ListItem)>>();
                    
                    items.sort_by(|a, b| b.0.cmp(&a.0));
                    items.into_iter().map(|(_, item)| item).collect::<Vec<ListItem>>()
                })), Some("No messages yet.\nGet started by messaging a friend.")),
            ], None, 
            ("New Message".into(), new_message), None,
        )
    }

    // fn update(&self, ctx: &mut Context) -> bool {
    //     self.0 != ctx.list::<ChatRoom>()
    // }
}

pub struct NewMessageFlow;
impl NewMessageFlow {
    pub fn new(ctx: &mut Context, theme: &Theme) -> Form {
        let closure = Box::new(move |ctx: &mut Context, objects: &Vec<State>| {
            let id = ctx.create(ChatRoom::new()).unwrap();
            
            if let Some(State::Search(result)) = objects.iter().find(|s| matches!(s, State::Search(_))) {
                result.iter().for_each(|recipient| {
                    ctx.send(id, "/members", AddMember(*recipient)).unwrap();
                    ctx.share::<ChatRoom>(id, *recipient);
                    println!("Created room {:?} with members {:?}", id, recipient);
                })
            }

            Some(Chat::new(id))
        }) as Box<dyn FormSubmit>;

        let items = ctx.list::<Contact>().iter().map(|id| {
            let name = if let Some(Substance::String(name)) = ctx.get::<Contact, _>(id, "/name") {name} else {"orange_name".to_string()};
            let username = if let Some(Substance::String(username)) = ctx.get::<Contact, _>(id, "/username") {username} else {"Friend".to_string()};
            let item = ListItem::avatar(AvatarContent::default(), &username, &name, None, None);
            (item, Name::from_str(&name).unwrap())
        }).collect::<Vec<_>>();
        let items = items.into_iter().filter(|(_, n)| *n != ctx.me()).collect::<Vec<_>>();
        Form::flow(theme, vec![FormItem::search("Select recipient", items)], None, None, closure)
    }
}

pub struct Chat;
impl Chat {
    pub fn new(room_id: Id) -> Box<dyn PageBuilder> {
        Box::new(move || PageType::messaging(room_id))
    }
}
