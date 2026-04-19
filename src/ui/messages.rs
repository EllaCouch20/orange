#![allow(clippy::new_ret_no_self)]
use chk::{FormItem, Flow, Display, Context, PageType, PageBuilder, Theme, Form, Root, State, Message, Profile, FormSubmit, ListItem};

#[derive(Debug, Clone)]
pub struct MessagesHome;
impl MessagesHome {
    pub fn new(theme: &Theme) -> Root {
        let messages = Message::tests();
        let message = messages[0].clone();
        let chat = Flow::new(theme, vec![Chat::new(messages)]);
        let items = vec![ListItem::avatar(message.author.avatar(), &message.author.name, &message.message, None, Some(chat))];

        Root::display(
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
        Box::new(move || {
            let profiles = messages.clone().into_iter().map(|m| m.author).collect::<std::collections::HashSet<_>>().into_iter().collect::<Vec<_>>();
            PageType::messaging(messages.clone(), profiles)
        })
    }
}