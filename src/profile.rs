#![allow(clippy::new_ret_no_self)]

use chk::{Context, State, FormSubmit, FormItem, Flow, AvatarIconStyle, AvatarContent, Display, Icons, Root, Action, Input, Theme, PageType, Bumper, Offset, PageBuilder, IS_MOBILE, ActionItem};
use chk::messages::{Profile, ChangeNotes, ChangeUsername};
use air::names::{Name, Secret, Id};
use crate::contacts::ViewContact;

pub struct ProfileHome;
impl ProfileHome {
    pub fn new(ctx: &mut Context, theme: &Theme) -> Root {
        let (profile, contact_id) = Profile::me(ctx);
        // let display = vec![match IS_MOBILE {
        //     true => Display::cta(
        //         "Connect to a Computer", None, 
        //         "Connect this device to a laptop or desktop computer to back up accounts or create a savings wallet", 
        //         vec![("Connect Computer".to_string(), Icons::Link, Action::flow(Flow::new(theme, vec![DownloadDesktop::new(), Box::new(move || PageType::scan_qr())])))]
        //     ),
        //     false => Display::cta(
        //         "Connect to a Phone", None, 
        //         "Connect this device to a mobile phone to back up accounts or create a savings wallet", 
        //         vec![("Connect Phone".to_string(), Icons::Link, Action::flow(Flow::new(theme, vec![StartMobileApp::new()])))]
        //     ),
        // }];

        // let id: Option<Id> = ctx.list::<Contact>().iter().find(|contact| {
        //     let name = ctx.get::<Contact, _>(&contact, "/name");
        //     name == ctx.me()
        // }).unwrap();

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
            

        Root::custom(PageType::edit_and_display(
            "My profile",
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
        ))
    }
}

// pub struct DownloadDesktop;
// impl DownloadDesktop {
//     pub fn new() -> Box<dyn PageBuilder> {
//         Box::new(|| PageType::display(
//             "Download desktop app",
//             vec![
//                 Display::image("chk", (100.0, 100.0)),
//                 Display::label("Install the orange desktop app on your laptop or desktop computer"),
//                 Display::label("desktop.orange.me"),
//             ],
//             None,
//             Bumper::Default,
//             Offset::Center,
//         ))
//     }
// }


// pub struct StartMobileApp;
// impl StartMobileApp {
//     pub fn new() -> Box<dyn PageBuilder> {
//         Box::new(|| PageType::display(
//             "Connect mobile app",
//             vec![
//                 Display::qr_code("blah blah blah i don't know what to put here in order to generate a qrcode with data", ""),
//                 Display::label("Scan with the orange mobile app"),
//                 Display::instructions("Scan this QR code with your phone to connect your phone with this laptop or desktop computer"),
//                 Display::instructions("or"),
//                 Display::actions(vec![ActionItem::new(Action::None, "Download mobile app", Icons::Link)]),
//             ],
//             None,
//             Bumper::Default,
//             Offset::Center,
//         ))
//     }
// }