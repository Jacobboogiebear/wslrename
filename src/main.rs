extern crate winreg;
use winreg::enums::*;
use winreg::RegKey;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("This command needs 2 arguments: Distro to rename and the new name for the distro");
        exit(-1);
    }
    let reg: RegKey = open_registry();
    let keys: Vec<String> = get_keys_from_registry(&reg);
    let names: Vec<[String; 2]> = get_distros_from_registry(&reg, keys);
    let mut selected: [String; 2] = ["".to_string(), "".to_string()];
    for i in names {
        if &args[1] == &i[1] {
            selected = i;
            break;
        }
    }
    if selected[0] == "" {
        println!("No distro found with that name, exiting");
        exit(-1);
    }
    rename_distro(&reg, &selected[0], &args[2]);
    println!("Renamed distro! Exiting!");
    exit(0);
}

fn open_registry() -> RegKey {
    let hklm: RegKey = RegKey::predef(HKEY_CURRENT_USER);
    return hklm.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Lxss", KEY_ALL_ACCESS).unwrap();
}

fn get_keys_from_registry(reg: &RegKey) -> Vec<String> {
    let mut keys: Vec<String> = vec![];
    for i in reg.enum_keys().map(|x| x.unwrap())
    {
        keys.push(i);
    }
    return keys;
}

fn get_distros_from_registry(reg: &RegKey, keys: Vec<String>) -> Vec<[String; 2]> {
    let mut names: Vec<[String; 2]> = vec![];
    for i in keys {
        let current_distro: RegKey = reg.open_subkey_with_flags(&i, KEY_ALL_ACCESS).unwrap();
        let distro_name: String = current_distro.get_value("DistributionName").unwrap();
        names.push([i, distro_name]);
    }
    return names;
}

fn rename_distro(reg: &RegKey, key: &String, name: &String) {
    let distro: RegKey = reg.open_subkey_with_flags(key, KEY_ALL_ACCESS).unwrap();
    distro.delete_value("DistributionName").unwrap();
    distro.set_value("DistributionName", name).unwrap();
}