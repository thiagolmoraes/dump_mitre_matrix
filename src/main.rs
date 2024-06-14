use std::{fs::File, io::BufReader};
use std::error::Error;
use rusqlite::Connection;
mod models;
use models::{Mitre, Objects, save_to_db};

fn map_attack() ->  Result<Vec<Mitre>, Box<dyn Error>>{
    let file = File::open("enterprise-attack.json")?;
    let reader = BufReader::new(file);

    let data: Objects = serde_json::from_reader(reader)?;
    
    let mut mitre_instances: Vec<Mitre> = Vec::new();

    for attack in data.objects {
        if attack.obj_type == "attack-pattern" {
            
            if let (Some(name), Some(id_mitre), Some(phases)) = 
                        (attack.name, attack.external_references, attack.kill_chain_phases) {

                if let Some(id) = id_mitre.get(0) {
                    if let Some(id_mitre) = &id.external_id{

                        let tactics: Vec<String> = phases.into_iter().filter_map(|p| p.phase_name).collect();

                        let attack = Mitre::new(id_mitre.to_string(), name, tactics);
                        mitre_instances.push(attack);
                    }
                }
            }
        }
    }
    Ok(mitre_instances)
}

fn main() -> Result<(), Box<dyn Error>> {  
    
    let mitre_instances = map_attack()?;
    
    let conn = Connection::open("database.db")?;

    let tactics = save_to_db(&conn, mitre_instances)?;

    Ok(())
}
