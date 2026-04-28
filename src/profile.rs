#![allow(clippy::new_ret_no_self)]

use chk::{Flow, AvatarIconStyle, AvatarContent, Display, Icons, Root, Action, Input, Theme, PageType, Bumper, Offset, PageBuilder, IS_MOBILE, ActionItem};

use air::names::{Name, Secret};
use std::sync::Arc;
use rand::{seq::SliceRandom, Rng};
use std::fs;

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

        Root::input("My Profile",
            vec![
                Input::avatar(profile.avatar, Some((Icons::Edit, AvatarIconStyle::Secondary)), None),
                Input::text("Username", true, Some(profile.username.to_string()), None),
                Input::text("About me", true, Some(profile.username), None),
            ],
            // display,
            None, ("Save".into(), Flow::default()), None,
        )
    }
}

#[derive(Clone, Debug)]
pub struct Profile {
    pub name: Name,
    pub username: String,
    pub avatar: AvatarContent,
}

impl Profile {
    pub fn new(name: Name) -> Self { Profile { name, username: Username::new(), avatar: AvatarContent::icon(Icons::Profile, AvatarIconStyle::Secondary) } }
    // pub fn avatar(&self) -> AvatarContent {
    //     match &self.pfp {
    //         Some(img) => AvatarContent::image(img.clone()),
    //         None => AvatarContent::icon(Icons::Profile, AvatarIconStyle::Secondary)
    //     }
    // }
}

pub struct Username;
impl Username {
    pub fn new() -> String {        
        let mut rng = rand::thread_rng();

        let read = |p: &str| -> Vec<String> {fs::read_to_string(p).unwrap().lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect()};

        let cap = |s: &str| {
            let s = s.to_lowercase();
            let mut c = s.chars();
            c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
        };

        let animals = read("usernames/animals.txt");
        let foods = read("usernames/foods.txt");
        let adjectives = read("usernames/adjectives.txt");

        let noun_list = if rng.gen_bool(0.5) { &animals } else { &foods };
        format!("{}{}", cap(adjectives.choose(&mut rng).unwrap()), cap(noun_list.choose(&mut rng).unwrap()))
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