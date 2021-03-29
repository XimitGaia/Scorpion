pub mod domain;
pub mod adapters;
use adapters::db::sqlite;

fn main() {
    let sqlite = sqlite::Db{
        hash: String::from("sa768dvb6eriho4588945g-929034994")
    };
}
