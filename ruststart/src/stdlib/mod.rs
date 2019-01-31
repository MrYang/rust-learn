mod cli;
mod threads;

pub fn s() {

    cli::c();
    threads::listen_tcp();
}