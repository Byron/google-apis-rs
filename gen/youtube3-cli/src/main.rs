// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate docopt;
extern crate rustc_serialize;

docopt!(Args derive Debug, "
Usage: 
    youtube3 activities (insert|list)
    youtube3 captions (delete|download|insert|list|update)
    youtube3 channel-banners insert
    youtube3 channel-sections (delete|insert|list|update)
    youtube3 channels (list|update)
    youtube3 guide-categories list
    youtube3 i18n-languages list
    youtube3 i18n-regions list
    youtube3 live-broadcasts (bind|control|delete|insert|list|transition|update)
    youtube3 live-streams (delete|insert|list|update)
    youtube3 playlist-items (delete|insert|list|update)
    youtube3 playlists (delete|insert|list|update)
    youtube3 search list
    youtube3 subscriptions (delete|insert|list)
    youtube3 thumbnails set
    youtube3 video-categories list
    youtube3 videos (delete|get-rating|insert|list|rate|update)
    youtube3 watermarks (set|unset)
    youtube3 --help
");

fn main() {
    let _: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    println!("Hello, youtube:v3 !");
}