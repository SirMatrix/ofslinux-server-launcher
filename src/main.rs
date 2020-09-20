
use std::fs;
use std::path::Path;
mod utils;


pub fn check_existance(){
    let remotep = "ofslinux-server-launcher/remote";
    let localp =  "ofslinux-server-launcher/local";
    let remote = Path::new(&remotep).exists();
    let local = Path::new(&localp).exists();

    if remote != true{
        fs::create_dir_all(remotep).unwrap();
    }
    if local != true{
        fs::create_dir_all(localp).unwrap();
    }

}




fn main() {

    let ofp = "open_fortress";
    let ofd = Path::new(&ofp).exists();
    if ofd != true {
        fs::create_dir_all(ofp).unwrap();

    }
    utils::netcode::reader("https://svn.openfortress.fun/files/");



}
