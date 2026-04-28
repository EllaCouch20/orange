#![allow(clippy::new_ret_no_self)]
use chk::{FormItem, Flow, Display, Context, PageType, PageBuilder, Theme, Form, Root, State, Message, FormSubmit, ListItem};
use crate::profile::Profile;

use air::names::{Secret, Id, Name};
use air::contract::{Contracts, Contract, Substance, Reactants, Reactant, Beaker};

use std::collections::BTreeMap;
use std::path::{PathBuf, Path};
use std::convert::Infallible;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct MessagesHome(Vec<Id>);
impl MessagesHome {
    pub fn new(ctx: &mut Context, theme: &Theme) -> Root {
        let ids = ctx.list::<ChatRoom>();
        println!("Room ids {:?}", ids);
        if let Some(id) = ids.get(0) {
            let first_name = ctx.get::<ChatRoom, _>(&ids[0], "/name").unwrap_or(Substance::String("Message".to_string()));
            println!("Rooms {:?}", first_name);
        }

        

        Root::display(
            "Messages", vec![
                Display::list(None, |ctx: &mut Context| {
                    let messages = Message::tests();
                    let message = messages[0].clone();
                    let chat = Flow::new(theme, vec![Chat::new(messages)]);
                    vec![ListItem::avatar(message.author.avatar(), &message.author.name, &message.message, None, Some(chat))]
                }, Some("No messages yet.\nGet started by messaging a friend.")),
            ], None, 
            ("New Message".into(), Flow::from_form(NewMessageFlow::new(theme))), None,
        )
    }

    // fn update(&self, ctx: &mut Context) -> bool {
    //     self.0 != ctx.list::<ChatRoom>()
    // }
}

pub struct NewMessageFlow;
impl NewMessageFlow {
    pub fn new(theme: &Theme) -> Form {
        let closure = Box::new(move |ctx: &mut Context, objects: &Vec<State>| {
            // TODO: Fix searchbar to store items by orange name and not by item title
            // if let Some(State::Search(result)) = objects.iter().find(|s| matches!(s, State::Search(_))) {
            //     println!("Search results {:?}", result);
            // }

            let recipient = Secret::new().name();
            let id = ctx.create(ChatRoom).unwrap();
            ctx.send(id, "/members", AddMember(recipient)).unwrap();
            ctx.share::<ChatRoom>(id, recipient);
            println!("Created room {:?} with members {:?}", id, recipient);
        }) as Box<dyn FormSubmit>;
        // let items = Profile::more_tests().into_iter().map(|proflie| ListItem::avatar(profile.avatar(), &profile.name, "did address here", None, None)).collect::<Vec<_>>();
        let profiles = vec![Profile::new(Secret::new().name()), Profile::new(Secret::new().name())];
        let items = profiles.into_iter().map(|profile| ListItem::avatar(profile.avatar, &profile.username, &profile.name.to_string(), None, None)).collect::<Vec<_>>();
        Form::new(theme, vec![FormItem::search("Select recipient", items)], None, None, closure)
    }
}

pub struct Chat;
impl Chat {
    pub fn new(messages: Vec<Message>) -> Box<dyn PageBuilder> {
        Box::new(move || {
            let profiles = messages.clone().into_iter().map(|m| m.author).collect::<std::collections::HashSet<_>>().into_iter().collect::<Vec<_>>();
            PageType::messaging(messages.clone(), profiles)
        })
    }
}

#[derive(Serialize, Deserialize, Hash)]
pub struct ChatRoom;
impl ChatRoom {
    pub fn new(_name: &str) -> Self {ChatRoom}
}
impl Contract for ChatRoom {
    fn id() -> Id {Id::hash("ChatRoom2.5")}

    fn init(self, signer: &Name, _timestamp: u64) -> Substance {Substance::Map(BTreeMap::from([
        ("name".to_string(), Substance::String("myroom".to_string())),
        ("members".to_string(), Substance::map()),
        ("author".to_string(), Substance::String(signer.to_string())),
        ("messages".to_string(), Substance::map())
    ]))}

    fn routes() -> BTreeMap<PathBuf, Reactants> {
        BTreeMap::from([
            (PathBuf::from("/name"), Reactants::new().add::<ChangeName>()),
            (PathBuf::from("/messages"), Reactants::new().add::<SendMessage>()),
            (PathBuf::from("/members"), Reactants::new().add::<AddMember>()),
        ])
    }
}

#[derive(Serialize, Deserialize, Hash)]
pub struct ChangeName(String);
impl Reactant for ChangeName {
    type Error = Infallible;
    type Contract = ChatRoom;

    fn apply<B: Beaker>(self, _path: &Path, signer: &Name, _timestamp: u64, substance: &mut B) -> Result<(), Self::Error> {
        if substance.query("/author") == Ok(Substance::String(signer.to_string())) {
            let _ = substance.insert("/name", Substance::String(self.0));
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Hash)]
pub struct AddMember(Name);
impl Reactant for AddMember {
    type Error = Infallible;
    type Contract = ChatRoom;

    fn apply<B: Beaker>(self, _path: &Path, signer: &Name, _timestamp: u64, substance: &mut B) -> Result<(), Self::Error> {
        if substance.query("/author") == Ok(Substance::String(signer.to_string())) {
            let _ = substance.insert("/members/-", Substance::String(self.0.to_string()));
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Hash)]
pub struct SendMessage(String);
impl Reactant for SendMessage {
    type Error = Infallible;
    type Contract = ChatRoom;

    fn apply<B: Beaker>(self, _path: &Path, signer: &Name, timestamp: u64, substance: &mut B) -> Result<(), Self::Error> {
        let _ = substance.insert("/messages/-", Substance::Map(BTreeMap::from([
            ("author".to_string(), Substance::String(signer.to_string())),
            ("timestamp".to_string(), Substance::Integer(timestamp as i64)),
            ("body".to_string(), Substance::String(self.0)),
        ])));
        Ok(())
    }
}