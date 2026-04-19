#![allow(clippy::new_ret_no_self)]

use chk::{Flow, AvatarIconStyle, Display, Icons, Root, Profile, Action, Input, Theme, PageType, Bumper, Offset, PageBuilder, IS_MOBILE};

pub struct ProfileHome;
impl ProfileHome {
    pub fn new(theme: &Theme, profile: Profile) -> Root {
        let profile = profile.clone();
        // let display = vec![
        //     Display::cta(
        //         "Connect to a Computer", None, 
        //         "Connect this device to a laptop or desktop computer to back up accounts or create a savings wallet", 
        //         vec![("Connect Computer".to_string(), Icons::Link, Action::flow(Flow::new(theme, vec![DownloadDesktop::new()])))]
        //     ),
        // ]; //).unwrap_or_default();

        Root::input("My Profile",
            vec![
                Input::avatar(profile.avatar(), Some((Icons::Edit, AvatarIconStyle::Secondary)), None),
                Input::text("Username", true, Some(profile.name.to_string()), None),
                Input::text("About me", true, Some(profile.name), None),
            ],
            // vec![], //display,
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