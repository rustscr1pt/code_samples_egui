use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use mysql::{params, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::Mutex;

pub struct LogsBody {
    pub storage : Vec<String>,
    pub log_sender : Sender<String>,
    pub log_receiver: Receiver<String>,
}

pub struct UserInput {
    pub language_name : String,
    pub sample_name : String,
    pub code_to_save : String,
    pub libraries : String
}
#[derive(PartialEq)]
pub enum DisplayPosition {
    Display,
    Add,
    Concrete
}

pub struct SqlSender {
    language_type : String,
    sample_name : String,
    sample : String,
    library_sample : String
}

pub struct DisplayStorage {
    pub language_type : String,
    pub sample_name : String,
    pub sample : String,
    pub library_sample : String,
    pub id : u16
}

pub struct ConcreteStorage {
    pub language_type : String,
    pub sample_name : String,
    pub sample : String,
    pub library_sample : String,
    pub id : u16
}

pub struct StorageBody {
    pub storage_vector : Vec<DisplayStorage>,
    pub storage_sender : Sender<Vec<DisplayStorage>>,
    pub storage_receiver : Receiver<Vec<DisplayStorage>>
}

pub struct TimerStorage {
    pub countdown : u8,
    pub countdown_sender : Sender<u8>,
    pub countdown_receiver : Receiver<u8>,

    pub active : bool,
    pub sender_active : Sender<bool>,
    pub receiver_active : Receiver<bool>
}

pub struct MainBody {
    pub logs_body : LogsBody,
    pub user_input : UserInput,
    pub display_position : DisplayPosition,
    pub sql_connection : Arc<Mutex<PooledConn>>,
    pub display_storage : StorageBody,
    pub update_timer : TimerStorage
}

pub fn new(connection : Arc<Mutex<PooledConn>>) -> MainBody {
    let (tx, rx) = std::sync::mpsc::channel();
    let (tx2, rx2) = std::sync::mpsc::channel();
    let (tx3, rx3) = std::sync::mpsc::channel();
    let (tx4, rx4) = std::sync::mpsc::channel();

    return MainBody {
        logs_body: LogsBody {
            storage: Vec::new(),
            log_sender: tx,
            log_receiver: rx
        },
        user_input: UserInput {
            language_name: "".to_string(),
            sample_name: "".to_string(),
            code_to_save: "".to_string(),
            libraries: "".to_string(),
        },
        display_position: DisplayPosition::Display,
        sql_connection: connection,
        display_storage: StorageBody {
            storage_vector: Vec::new(),
            storage_sender: tx2,
            storage_receiver: rx2,
        },
        update_timer: TimerStorage {
            countdown: 7,
            countdown_sender: tx3,
            countdown_receiver: rx3,

            active: false,
            sender_active: tx4,
            receiver_active: rx4,
        },
    }
}

impl MainBody {
    pub fn clear_fields(&mut self) -> () { // Clear all fields inside the app.
        self.user_input.libraries.clear();
        self.user_input.code_to_save.clear();
        self.user_input.sample_name.clear();
        self.user_input.language_name.clear();
    }

    pub fn analyze_clear(&mut self, log : String) -> () {
        let key = log.split("~").collect::<Vec<&str>>();
        if key.get(0).unwrap().to_owned() == "$".to_owned() {
            self.clear_fields();
        }
    }
}

// pub fn spawn_update_timer(timer : Sender<u8>, bool : Sender<bool>) -> () {
//     tokio::spawn(async move {
//         let _ = bool.send(true);
//         let mut counter = 7u8;
//         loop {
//             if counter != 0 {
//                 tokio::time::sleep(Duration::from_secs(1)).await;
//                 let _ = timer.send(1);
//                 counter -= 1;
//             }
//             else {
//                 return
//             }
//         }
//     });
// }


pub fn send_data(connection : Arc<Mutex<PooledConn>>, sender : Sender<String>, language_name : String, sample_name : String, sample : String, library_sample : String) -> () {
    let work_vec : Vec<SqlSender> = vec![SqlSender {
        language_type: language_name,
        sample_name: sample_name,
        sample: sample,
        library_sample: library_sample,
    }];
    tokio::spawn(async move {
        let mut connection = connection.lock().await;
        match connection.exec_batch(r"INSERT INTO code_samples (language_type, sample_name, sample, library_sample) VALUES (:language_type, :sample_name, :sample, :library_sample)",
                                    work_vec.iter().map(|cc| params! {
                "language_type" => &cc.language_type,
                "sample_name" => &cc.sample_name,
                "sample" => &cc.sample,
                "library_sample" => &cc.library_sample
            })
        ) {
            Ok(_) => {
                let _ = sender.send("Your code snippet is saved in the SQL".to_owned());
            }
            Err(e) => {
                let _ = sender.send(format!("$~ERROR! {}", e));
            }
        }
    });
}

pub fn get_data(connection : Arc<Mutex<PooledConn>>, sender : Sender<String>, storage_sender : Sender<Vec<DisplayStorage>>) -> () {
    tokio::spawn(async move {
        let mut connection = connection.lock().await;
        match connection.query_map("SELECT * FROM code_samples", |(id, language_type, sample_name, sample, library_sample)| {
            DisplayStorage {
                language_type: language_type,
                sample_name: sample_name,
                sample: sample,
                library_sample: library_sample,
                id: id,
            }
        }) {
            Ok(vec) => {
                let _ = sender.send("All samples from SQL were uploaded.".to_string());
                let _ = storage_sender.send(vec);
            }
            Err(e) => {
                let _ = sender.send(format!("$~ERROR! {}", e));
            }
        }
    });
}

pub fn remove_element_sql(connection : Arc<Mutex<PooledConn>>, sender : Sender<String>, id : u16, storage_sender : Sender<Vec<DisplayStorage>>) -> () {
    tokio::spawn(async move {
        let mut connection = connection.lock().await;
        match connection.query_drop(format!(r"DELETE FROM code_samples WHERE id = {}", id)) {
            Ok(_) => {let _ = sender.send(format!("Sample with id : {} was deleted from the database.", id));}
            Err(e) => {let _ = sender.send(format!("$~ERROR! {}", e));}
        };
        match connection.query_map("SELECT * FROM code_samples", |(id, language_type, sample_name, sample, library_sample)| {
            DisplayStorage {
                language_type: language_type,
                sample_name: sample_name,
                sample: sample,
                library_sample: library_sample,
                id: id,
            }
        }) {
            Ok(vec) => {
                let _ = sender.send("All samples from SQL were uploaded.".to_string());
                let _ = storage_sender.send(vec);
            }
            Err(e) => {
                let _ = sender.send(format!("$~ERROR! {}", e));
            }
        }
    });
}