use cybercore_packs::{PackLock, PackManifest, Registry, RegistryEntry};
use std::{env, fs, path::{Path, PathBuf}};

const DEFAULT_REGISTRY: &str =
    "https://raw.githubusercontent.com/cybercoretech/cybercore-packs/main/registry/index.json";
const DEFAULT_RAW_ROOT: &str =
    "https://raw.githubusercontent.com/cybercoretech/cybercore-packs/main/";

fn usage() -> ! {
    eprintln!(
        "usage: cybercore-pack [pack] <search|install|add|remove|list|lock|verify> [query|pack-id]"
    );
    std::process::exit(2);
}

fn registry_url() -> String {
    env::var("CYBERCORE_PACK_REGISTRY").unwrap_or_else(|_| DEFAULT_REGISTRY.into())
}

fn registry_root() -> String {
    registry_url()
        .strip_suffix("registry/index.json")
        .unwrap_or(DEFAULT_RAW_ROOT)
        .to_string()
}

fn fetch(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(ureq::get(url).call()?.into_string()?)
}

fn load_registry() -> Result<Registry, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(&fetch(&registry_url())?)?)
}

fn split_spec(spec: &str) -> (&str, Option<&str>) {
    match spec.split_once('@') {
        Some((id, version)) => (id, Some(version)),
        None => (spec, None),
    }
}

fn select_entry<'a>(registry: &'a Registry, spec: &str) -> Result<&'a RegistryEntry, Box<dyn std::error::Error>> {
    let (id, version) = split_spec(spec);
    registry
        .packs
        .iter()
        .find(|pack| pack.id == id && version.map_or(true, |v| pack.version == v))
        .ok_or_else(|| format!("pack not found in registry: {spec}").into())
}

fn lock_path() -> PathBuf {
    Path::new(".cybercore").join("packs.lock.json")
}

fn install(entry: &RegistryEntry) -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(".cybercore").join("packs").join(&entry.id);
    fs::create_dir_all(&root)?;
    let manifest_url = format!("{}{}", registry_root(), entry.manifest);
    let manifest_path = root.join("pack.json");
    fs::write(&manifest_path, fetch(&manifest_url)?)?;

    let lock_path = lock_path();
    let mut lock = if lock_path.exists() {
        PackLock::load(&lock_path)?
    } else {
        PackLock { lockfile_version: 1, packs: Vec::new() }
    };
    lock.add(entry);
    if let Some(pack) = lock.packs.iter_mut().find(|pack| pack.id == entry.id) {
        pack.source = Some(manifest_url);
        pack.install_path = Some(root.to_string_lossy().into_owned());
    }
    lock.save(&lock_path)?;
    println!("installed {} {} at {}", entry.id, entry.version, root.display());
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("pack") {
        args.remove(0);
    }
    let command = args.first().map(String::as_str).unwrap_or_else(|| usage());

    match command {
        "search" => {
            let query = args.get(1).map(String::as_str).unwrap_or("");
            for pack in load_registry()?.search(query) {
                println!("{} {} ({:?})", pack.id, pack.version, pack.channel);
            }
        }
        "install" | "add" => {
            let spec = args.get(1).map(String::as_str).unwrap_or_else(|| usage());
            let registry = load_registry()?;
            install(select_entry(&registry, spec)?)?;
        }
        "remove" => {
            let id = args.get(1).map(String::as_str).unwrap_or_else(|| usage());
            let lock_path = lock_path();
            let mut lock = PackLock::load(&lock_path)?;
            let removed = lock.packs.iter().find(|pack| pack.id == id).cloned();
            lock.packs.retain(|pack| pack.id != id);
            if let Some(pack) = removed {
                if let Some(path) = pack.install_path {
                    let path = PathBuf::from(path);
                    if path.exists() {
                        fs::remove_dir_all(path)?;
                    }
                }
                lock.save(&lock_path)?;
                println!("removed {id}");
            } else {
                println!("pack is not installed: {id}");
            }
        }
        "list" => {
            let lock = PackLock::load(lock_path())?;
            for pack in lock.packs {
                println!("{} {}", pack.id, pack.version);
            }
        }
        "lock" => {
            let path = lock_path();
            if !path.exists() {
                PackLock { lockfile_version: 1, packs: Vec::new() }.save(&path)?;
            }
            println!("lockfile ready: {}", path.display());
        }
        "verify" => {
            let lock = PackLock::load(lock_path())?;
            for pack in lock.packs {
                let path = pack.install_path.ok_or_else(|| format!("{} has no install path", pack.id))?;
                PackManifest::from_path(Path::new(&path).join("pack.json"))?;
            }
            println!("pack lock verified");
        }
        _ => usage(),
    }
    Ok(())
}
