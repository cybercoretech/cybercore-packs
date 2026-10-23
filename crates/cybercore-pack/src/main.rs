use cybercore_packs::{PackLock, Registry};
use std::{env, path::Path};

fn usage() -> ! {
    eprintln!("usage: cybercore-pack <search|add|lock|verify> [query|pack-id]");
    std::process::exit(2);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let command = args.next().unwrap_or_else(|| usage());
    let registry = Path::new("registry/index.json");
    let lockfile = Path::new(".cybercore/packs.lock.json");

    match command.as_str() {
        "search" => {
            let query = args.next().unwrap_or_default();
            for pack in Registry::from_path(registry)?.search(&query) {
                println!("{} {} ({:?})", pack.id, pack.version, pack.channel);
            }
        }
        "add" => {
            let id = args.next().unwrap_or_else(|| usage());
            let registry = Registry::from_path(registry)?;
            let entry = registry.packs.iter().find(|pack| pack.id == id).ok_or_else(|| format!("pack not found: {id}"))?;
            let mut lock = if lockfile.exists() {
                PackLock::load(lockfile)?
            } else {
                PackLock { lockfile_version: 1, packs: Vec::new() }
            };
            lock.add(entry);
            lock.save(lockfile)?;
            println!("locked {} {}", entry.id, entry.version);
        }
        "lock" => {
            if !lockfile.exists() {
                PackLock { lockfile_version: 1, packs: Vec::new() }.save(lockfile)?;
            }
            println!("lockfile ready: {}", lockfile.display());
        }
        "verify" => {
            PackLock::load(lockfile)?.verify_files(".")?;
            println!("pack lock verified");
        }
        _ => usage(),
    }
    Ok(())
}
