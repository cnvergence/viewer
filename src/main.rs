#![deny(warnings)]
use warp::Filter;
use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let pdf = get_pdf_path();
    let route = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(pdf));

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}

fn get_pdf_path() -> impl Into<PathBuf>{
let filepath = format!("./{}.pdf", get_env_var());
PathBuf::from(filepath)
}

fn get_env_var() -> std::string::String {
    let key = "REQUEST_HASH";
    match env::var(key) {
        Ok(val) => {return val},
        Err(e) => panic!("couldn't interpret {}: {}", key, e),
};
}
