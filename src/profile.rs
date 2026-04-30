#![allow(clippy::new_ret_no_self)]

use chk::{Flow, AvatarIconStyle, AvatarContent, Display, Icons, Root, Action, Input, Theme, PageType, Bumper, Offset, PageBuilder, IS_MOBILE, ActionItem};
use chk::messages::Profile;
use air::names::{Name, Secret};

pub struct ProfileHome;
impl ProfileHome {
    pub fn new(theme: &Theme) -> Root {
        let profile = Profile::new(Secret::new().name());
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

        Root::both("My Profile",
            vec![
                Input::avatar(profile.avatar, Some((Icons::Edit, AvatarIconStyle::Secondary)), None),
                Input::text("Username", true, Some(profile.username.to_string()), None),
                Input::text("About me", true, Some(profile.notes), None),
            ],
            vec![
                Display::cta("Orange name", None, &profile.name.to_string(), vec![("Copy".to_string(), Icons::Copy, Action::copy(&profile.name.to_string()))]),
            ],
            None, ("Save".into(), Flow::default()), None,
        )
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