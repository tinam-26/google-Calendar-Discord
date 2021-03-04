use rustcord::{Rustcord, EventHandlers, User, RichPresenceBuilder, RichPresence};
extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_calendar3 as calendar3;
//extern crate timer;

use calendar3::{DefaultDelegate, Error};
use std::default::Default;
use oauth2::{read_application_secret, DefaultAuthenticatorDelegate, Authenticator, ApplicationSecret, MemoryStorage};
// use oauth2::authenticator::Authenticator;
use calendar3::{CalendarHub, FreeBusyRequest, FreeBusyRequestItem};
use std::io::{stdin, stdout, Read, Write};
use std::path::Path;
use std::thread::{spawn}; 
use std::sync::mpsc::channel;
use chrono::{Duration as cDuration, Local};
pub struct Handlers;
//use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

impl EventHandlers for Handlers {
    fn ready(_user: User) {
        println!("Welcome {}#{}", _user.username, _user.discriminator);
    }
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

use hyper_rustls::TlsClient;
use hyper::Client;
use yup_oauth2::{InstalledFlowReturnMethod};

pub async fn event_main() {
    
    //let mut secret: ApplicationSecret = Default::default();

   // let timer = timer::Timer::new(); 
    //refresh token : 
    //access token : 
    let now = Local::now();
    let currTime = now.to_rfc3339(); 
    //sleep(Durration::from_millis(10));
    let nextTime = now + cDuration::minutes(15);
    let nextTime2 = nextTime.to_rfc3339();

    let secret = read_client_secret(CLIENT_SECRET_FILE);
    let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
       hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
       <MemoryStorage as Default>::default(), None);
    
    let refresh = auth.force_refreshed_token(todo!()).await.unwrap(); 
 

    let client = hyper::Client::with_connector(hyper::net::HttpsConnector::new(TlsClient::new()));
    let mut hub = CalendarHub::new(client, auth);
    let req = FreeBusyRequest {
        calendar_expansion_max: None,
        time_min: Some(currTime.to_string()),
        group_expansion_max: None,
        time_max: Some(nextTime2.to_string()),
        items: Some(vec![FreeBusyRequestItem{id: Some("email.com".to_string())}]),
        time_zone: Some("EST".to_string()),
    };
    
    //let result = hub.events().watch(req, "https://www.googleapis.com/auth/calendar/")
      //       .doit();
    //pause(); 
    let colors = hub.freebusy().query(req)
        //.param("timeMin", "2002-07-01T13:50:05Z") //needs to be in rfc3339
        .doit();
    
    //println!("{:?}", colors); //print?
    //println!("test");
    
    match colors {
        Err(ref e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
                Error::HttpError(_)
            |Error::MissingAPIKey
            |Error::MissingToken(_)
            |Error::Cancelled
            |Error::UploadSizeLimitExceeded(_, _)
            |Error::Failure(_)
            |Error::BadRequest(_)
            |Error::FieldClash(_)
            |Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(ref res) => println!("Success: {:?}", res),
    }

    //change number for each state
    //so like if blue change to class, yellow meeting, green busy (appointments), other nothing
    
    let isBusy = colors.unwrap();
   // println!("{:#?}", isBusy);
    let amIBusy = isBusy.1.calendars;
    let amIBusy2 = amIBusy.unwrap();
    let amIBusy3 = amIBusy2.get("christinamodica01@gmail.com");
    let amIAcutallyBusy = amIBusy3.unwrap().busy.as_ref().unwrap(); 
    println!("{:#?}", amIBusy2);

 
    let mut discordId = "802374088043266069";
    let mut discordState = "Error";
    let mut discordDetails = "";
    println!("{:?}",amIBusy2);

    if amIAcutallyBusy.is_empty(){
        discordId = "801484516031725568";
        discordState = "Feel free to DM if you need anything";
    }else if !amIAcutallyBusy.is_empty(){
        discordId = "802378932040105984";
        discordState = "Shhh.. Thinking hard over here.";
    }else{
        discordState = "Error at Line 111";
        discordDetails = "is the hashmap not empty?";
    }
    
    println!("{}",discordState);
    
    let discord = Rustcord::init::<Handlers>(discordId, true, None)
            .expect("Could no initialize RPC");
    let presence = RichPresenceBuilder::new()
        .state(discordState)
        .details(discordDetails)
        .large_image_key("rust")
        .large_image_text("Rust")
        .small_image_key("amethyst")
        .small_image_text("Amethyst")
        .build();
    discord.update_presence(presence).expect("Could not update presence");
   let (tx, rx) = channel(); 
    spawn(move|| {
        loop {
            tx.send(discord.run_callbacks());
            //let status = rx.recv();
           // discord.update_status(status).unwrap();
        }
   });
   /* tokio::run({
        discord.run_callbacks()
            .timeout(Duration::from_secs(900))
            .map_err(|e| {
                println!("operation timed out");
            })
    }) */
    



    
}

const CLIENT_SECRET_FILE: &str = "client_secret.json";

// reads the JSON secret file
fn read_client_secret(file: &str) -> ApplicationSecret {
    read_application_secret(Path::new(file))
        .expect("Cannot find credential, did you create client_secret.json?")
}