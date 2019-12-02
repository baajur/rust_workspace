use super::{web, AppState, Error, HttpResponse};
use crate::templates;
use clients_db::{fix_encoding::os_str_debug, ClientRecord, CritterInfo};
use futures::Future;
use serde::Serialize;
use std::{borrow::Cow, net::Ipv4Addr, time::Duration};
use tnf_common::defines::{
    fos,
    param::{CritterParam, Param},
};

pub fn clients(data: web::Data<AppState>) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || -> Result<_, ()> { Ok(data.get_ref().critters_db.list_clients()) })
        //.from_err()
        .then(|res| match res {
            Ok(clients) => match ClientsList::new(clients.clients().iter()).render() {
                Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
                Err(err) => {
                    eprintln!("GM Clients error: {:#?}", err);
                    Ok(HttpResponse::InternalServerError().into())
                }
            },
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
}

#[derive(Debug, Serialize)]
struct ClientsList<'a> {
    clients: Vec<ClientRow<'a>>,
}
#[derive(Debug, Serialize)]
struct ClientRow<'a> {
    name: &'a str,
    file: Cow<'a, str>,
    info: Option<ClientRowInfo<'a>>,
    last_seen: Option<(String, bool)>,
}
#[derive(Debug, Serialize)]
struct ClientRowInfo<'a> {
    id: u32,
    lvl: i32,
    hp: i32,
    map_id: u32,
    map_pid: u16,
    cond: &'static str,
    gamemode: &'static str,
    ip: &'a [Ipv4Addr],
}

const GAMEMODS: [&'static str; fos::GAME_MAX as usize] =
    ["START", "ADVENTURE", "SURVIVAL", "ARCADE", "TEST"];

impl<'a> ClientsList<'a> {
    fn new<I: Iterator<Item = (&'a String, &'a ClientRecord)>>(clients: I) -> Self {
        Self {
            clients: clients
                .map(|(name, record)| {
                    let info = record.info.as_ref().map(|info| ClientRowInfo {
                        id: info.id,
                        lvl: info.param(Param::ST_LEVEL),
                        hp: info.param(Param::ST_CURRENT_HP),
                        map_id: info.map_id,
                        map_pid: info.map_pid,
                        cond: info.cond(),
                        gamemode: GAMEMODS
                            [info.uparam(Param::QST_GAMEMODE).min(fos::GAME_MAX - 1) as usize],
                        ip: &info.ip[..],
                    });
                    ClientRow {
                        info,
                        name: &name,
                        file: os_str_debug(&record.filename),
                        last_seen: record
                            .modified
                            .and_then(|time| time.elapsed().ok())
                            .as_ref()
                            .map(ago),
                    }
                })
                .collect(),
        }
    }
    fn render(&self) -> Result<String, templates::TemplatesError> {
        templates::render("gm_clients.html", self)
    }
}

fn ago(duration: &Duration) -> (String, bool) {
    let secs = duration.as_secs();
    (
        if secs < 60 {
            format!("{}s", secs)
        } else if secs < 60 * 60 {
            format!("{}m", secs / 60)
        } else if secs < 24 * 60 * 60 {
            format!("{}h", secs / 60 / 60)
        } else {
            format!("{}d", secs / 60 / 60 / 24)
        },
        secs < 60 * 5,
    )
}
