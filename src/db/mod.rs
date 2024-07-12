use std::{collections::HashMap, f32::consts::E, fs::File, io::{Error, Read, Write}, time::SystemTime};
use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, Key, KeyInit, Nonce};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct YaepmDb {
    pub notes:HashMap<String,(u64,Vec<u8>)>
}

pub fn read_db(yaepm_home: &str)-> Result<YaepmDb,()>{
    match File::open(&yaepm_home) {
        Ok(db)=>{
            // Deserilize the db into a YaepmDb struct
            let yaepm_db:YaepmDb=serde_json::from_reader(db).unwrap();
            Ok(yaepm_db)
        },
        Err(_)=>{
            Err(())
        }
    }
}

pub fn setup_db(yaepm_home: &str)-> Result<(),()>{
    match File::create(&yaepm_home) {
        Ok(mut db)=>{
            let yaepm_db=YaepmDb{
                notes:HashMap::new()
            };
            // Serialize yaepm_db and write it to the file
            let serialized=serde_json::to_string(&yaepm_db).unwrap();
            db.write_all(serialized.as_bytes()).unwrap();
            Ok(())
        },
        Err(_)=>{
            Err(())
        }
    }
}