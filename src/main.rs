extern crate kubecfg;

use kubecfg::Config;

fn main() {
    if let Ok(c) = Config::from_std_path() {
        println!("{:#?}", c.clusters[&c.current_context]);
    }
}
