use std::fmt::format;

use serde::{Serialize, Deserialize};
use rusqlite::{params, Connection, Result, Error};


#[derive(Debug, Serialize, Deserialize)]
pub struct Objects {
    pub objects: Vec<Attack>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attack {
    #[serde(rename = "type")]
    pub obj_type: String,
    pub name: Option<String>,
    pub kill_chain_phases: Option<Vec<Phases>>,
    pub external_references: Option<Vec<References>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mitre {
    pub id: String,
    pub name: String,
    pub phases: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct References {
    pub external_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Phases {
    pub phase_name: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Technique{
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tactics{
    pub name: String,
}

pub struct Techniques{
    pub name: String,
}

impl Mitre{
    pub fn new(id:String, name:String, phases: Vec<String>) -> Self {
        Mitre{
            id: id,
            name: name,
            phases: phases.into_iter().map(|p| p.into()).collect()
        }
    }

}

pub fn save_to_db(conn: &Connection, mitre_instances: Vec<Mitre>) -> Result<(), Error> {
    
    for mitre in mitre_instances {    
       
        let mitre_name = format!("{} - {}", mitre.id, mitre.name);

        conn.execute(
            "INSERT INTO techniques (name) VALUES (?1)",
            params![mitre_name],
        )?;
       
        let technique_id = conn.last_insert_rowid();

        for phase in mitre.phases {
            conn.execute(
                "INSERT INTO tactics (name) VALUES (?1)
                 ON CONFLICT(name) DO NOTHING",
                params![phase],
            )?;

            // Get the tactic ID
            let tactic_id: i64 = conn.query_row(
                "SELECT id FROM tactics WHERE name = ?1",
                params![phase],
                |row| row.get(0),
            )?;

            // Insert into technique_tactics (the join table)
            conn.execute(
                "INSERT INTO technique_tactics (technique_id, tactic_id) VALUES (?1, ?2)",
                params![technique_id, tactic_id],
            )?;
        }
    }

    Ok(())
}