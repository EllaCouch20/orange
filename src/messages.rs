#![allow(clippy::new_ret_no_self)]
use chk::{AvatarContent, FormItem, Flow, Display, Context, PageType, PageBuilder, Theme, Form, Root, State, FormSubmit, ListItem, AvatarIconStyle, Icons};
use chk::messages::{Profile, ChatRoom, AddMember, Contact};

use air::names::{Secret, Id, Name};
use air::contract::{Contracts, Contract, Substance, Reactants, Reactant, Beaker};

use std::collections::BTreeMap;
use std::path::{PathBuf, Path};
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use std::str::FromStr;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct MessagesHome;
impl MessagesHome {
    pub fn new(ctx: &mut Context, theme: &Theme) -> Root {
        let new_message = Box::new(|ctx: &mut Context, theme: &Theme| Flow::from_form(NewMessageFlow::new(ctx, theme)));
        let theme = theme.clone();
        Root::new(
            "Messages", vec![
                Display::list(None, Arc::new(Box::new(move |ctx: &mut Context| {
                    let ids = ctx.list::<ChatRoom>();
                    ids.iter().map(|id| {
                        let messages = ctx.get::<ChatRoom, _>(&id, "/messages").unwrap_or_default();
                        let mut chat_name = "New Message".to_string();
                        let mut chat_avatar = AvatarContent::Icon(Icons::Profile, AvatarIconStyle::Secondary);
                        let mut chat_last = "No messages yet.".to_string();

                        if let Some(Substance::Seq(names)) = ctx.get::<ChatRoom, _>(&id, "/members") {
                            match names.len() > 1 {
                                true => {
                                    chat_name = "Group Message".to_string();
                                    chat_avatar = AvatarContent::Icon(Icons::Profile, AvatarIconStyle::Secondary);
                                }
                                false => if let Some(first) = names.get(0) {
                                    ctx.list::<Contact>().iter().for_each(|contact| {
                                        if Some(first.clone()) == ctx.get::<Contact, _>(&contact, "/name") {
                                            if let Some(Substance::String(name)) = ctx.get::<Contact, _>(&contact, "/username") {
                                                chat_name = name;
                                            }
                                        }
                                    })
                                }
                            }
                        }

                        if let Some(Substance::Seq(messages)) = ctx.get::<ChatRoom, _>(&id, "/messages") {
                            if let Some(last) = messages.last() {
                                if let Ok(Substance::String(message)) = last.query("/body") {
                                    chat_last = format!("You: {}", message);
                                    ctx.list::<Contact>().iter().for_each(|contact| {
                                        // println!("{:?}, {:?}", ctx.get::<Contact, _>(&contact, "/name"), last.query("/author").ok());
                                        let contact_name = ctx.get::<Contact, _>(&contact, "/name");
                                        if contact_name == last.query("/author").ok() {
                                            if contact_name != Some(Substance::String(ctx.me().to_string())) {
                                                if let Some(Substance::String(username)) = ctx.get::<Contact, _>(&contact, "/username") {
                                                    chat_last = format!("{}: {}", username, message);
                                                }
                                            }
                                        }
                                    })
                                }
                            }
                        }

                        // println!("Messages {:?}", messages.query("/body"));
                        // println!("Members {:?}", members);
                        let chat = Flow::new(&theme, vec![Chat::new(*id)]);
                        // let author = Profile::new(Secret::new().name());
                        // match members.get(0) {
                        //     Some(author) => ListItem::avatar(author.avatar, &author.username, "No message.", None, Some(chat)),
                        //     None => ListItem::avatar(AvatarContent::default(), "Group Message", "No message.", None, Some(chat)),
                        // }

                        ListItem::avatar(chat_avatar, &chat_name, &chat_last, None, Some(chat))
                        
                    }).collect::<Vec<ListItem>>()
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

            None
        }) as Box<dyn FormSubmit>;

        let items = ctx.list::<Contact>().iter().map(|id| {
            let name = if let Some(Substance::String(name)) = ctx.get::<Contact, _>(&id, "/name") {name} else {"orange_name".to_string()};
            let username = if let Some(Substance::String(username)) = ctx.get::<Contact, _>(&id, "/username") {username} else {"Friend".to_string()};
            let item = ListItem::avatar(AvatarContent::default(), &username, &name, None, None);
            (item, Name::from_str(&name).unwrap())
        }).collect::<Vec<_>>();
        Form::flow(theme, vec![FormItem::search("Select recipient", items)], None, None, closure)
    }
}

pub struct Chat;
impl Chat {
    pub fn new(room_id: Id) -> Box<dyn PageBuilder> {
        Box::new(move || PageType::messaging(room_id.clone()))
    }
}
