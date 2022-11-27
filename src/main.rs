use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    env,
};

// ------------------------------
// TCP Server
// ------------------------------

fn main() {
    let ping_listen_port = read_env_var();
    let ip = "0.0.0.0"; // Adresse IP
    let listener = TcpListener::bind(format!("{}:{}", ip, ping_listen_port)).unwrap(); // Listener
    println!("[+] Server listening on port {}", ping_listen_port);
    for stream in listener.incoming() { // Pour chaque connexion entrante
        let stream = stream.unwrap(); // On récupère le flux et on le stocke
        handle_connection(stream); // On appelle la fonction handle_connection avec le flux en paramètre
    }
}

// ------------------------------
// Request reader
// ------------------------------

fn handle_connection(mut stream: TcpStream) { // Fonction qui lit la requête pour chaque connexion entrante
    let buf_reader = BufReader::new(&mut stream); // Buffer qui prend la valeur du flux entrant "stream"
    let http_request: Vec<_> = buf_reader // On stocke le buffer précédent dans un vecteur :
        .lines() // Itérable de chaque ligne
        .map(|result| result.unwrap())// sur chaque ligne on récupère le résultat
        .take_while(|line| !line.is_empty())// On prend chaque ligne tant qu'elle n'est pas vide
        .collect(); // On collecte le tout dans le vecteur "http_request"
    if http_request[0] == "GET /ping HTTP/1.1" { // Si la requete pointe vers /ping
        println!("[+] Ping request received");
        stream.write(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\n{}", convert_to_json(http_request)).as_bytes()).unwrap();
    }
}


// ------------------------------
// Env var reader
// ------------------------------
// Fonction qui lit la variable d'environnement "PING_LISTEN_PORT"
fn read_env_var() -> String {
    let vars = env::vars().collect::<Vec<_>>();

    for (key, value) in vars {
        if key == "PING_LISTEN_PORT" {
            let port = value;
            println!("[+] Env var \"PING_LISTEN_PORT\" found");
            println!("[+] port set to = {}", port);
            return port;
        }
    }
    println!("[-] PING_LISTEN_PORT not found");
    let port = "8080".to_string();
    port
}

// ------------------------------
// Json converter
// ------------------------------
//Converti le vecteur de notre requête en json avec comme clé le nom du header.
fn convert_to_json(http_request: Vec<String>) -> String {
    let mut http_request = http_request;
    let mut json :String = String::new();
    http_request.remove(0);
    json.push_str("{");
    for header in http_request {
        let header: Vec<&str> = header.split(": ").collect();
        json.push_str(&format!("\"{}\": \"{}\",", header[0], header[1]));
    }
    json.pop();
    json.push_str("}");
    json
}


