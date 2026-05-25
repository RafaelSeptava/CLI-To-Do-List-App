use std::env;
use std::fs::File;
use std::io::BufReader;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LinuxTask {
    id: u32,
    command_line: String,
    description: String,
    is_completed: bool,
}

fn main() {
    let args: Vec<String> = env::args()
        .collect();

    if args.len() < 2 {
        println!("===== TODO-LIST CLI TERMINAL =====");
        println!("Gunakan salah satu perintah berikut:");
        println!("cargo run -- list");
        println!("cargo run -- add [nama_perintah] [deskripsi]");
        println!("cargo run -- complete [id_tugas]");
        println!("cargo run -- delete [id_tugas]");

        return;
    }

    let subcommand = &args[1];

    let mut tasks = load_tasks();

    match subcommand.as_str() {

        "list" => {
            if tasks.is_empty() {
                println!("\nBelum ada tugas belajar Linux. Tambahkan tugas baru!");
                println!("Contoh: cargo run -- add [nama_perintah] [deskripsi]");
                return;
            }

            for task in &tasks {
                let status = if task.is_completed { "[✓]" } else { "[ ]" };
                println!("{} {}: {} - {}", status, task.id, task.command_line, task.description);
            }
        }

        "add" => {
            if args.len() < 4 {
                println!("\nError: Masukkan nama perintah dan deskripsinya!");
                return;
            }

            let name = &args[2];
            let desc = args[3..].join(" ");

            let new_id = tasks.iter()
                .map(|task| task.id)
                .max()
                .unwrap_or(0) + 1;

            tasks.push(LinuxTask {
                id: new_id,
                command_line: name.to_string(),
                description: desc,
                is_completed: false,
            });

            save_tasks(&tasks);
            println!("\nBerhasil menambahkan tugas belajar baru: {}", name);
        }

        "complete" => {
            if args.len() < 3 {
                println!("\nError: Masukkan ID tugas yang ingin diselesaikan!");
                return;
            }

            let id_to_find: u32 = args[2]
                .parse()
                .expect("ID harus berupa angka!");

            if let Some(task) = tasks.iter_mut().find(|t| t.id == id_to_find) {
                task.is_completed = true;
                save_tasks(&tasks);
                println!("\nTugas ID {} berhasil diselesaikan.", id_to_find);
            }

            else {
                println!("\nTugas dengan ID {} tidak ditemukan.", id_to_find);
            }
        }

        "delete" => {
            if args.len() < 3 {
                println!("\nError: Masukkan ID tugas yang ingin dihapus!");
                return;
            }

            let id_to_delete: u32 = args[2]
                .parse()
                .expect("ID harus berupa angka!");

            let original_len = tasks.len();

            tasks.retain(|task| task.id != id_to_delete);

            if tasks.len() < original_len {
                save_tasks(&tasks);
                println!("\nTugas ID {} berhasil dihapus.", id_to_delete);
            }

            else {
                println!("\nTugas dengan ID {} tidak ditemukan.", id_to_delete);
            }
        }

        _ => println!("\nPerintah tidak dikenal! Gunakan 'list', 'add', 'complete', atau 'delete'."),
    }
}

fn load_tasks() -> Vec<LinuxTask> {
    let file = match File::open("linux_task.json") {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    let reader = BufReader::new(file);

    serde_json::from_reader(reader)
        .unwrap_or_else(|_| Vec::new())
}

fn save_tasks(tasks: &Vec<LinuxTask>) {
    let file = File::create("linux_task.json")
        .expect("Gagal memuat file JSON!");

    serde_json::to_writer_pretty(file, tasks)
        .expect("Gagal menulis JSON!");
}