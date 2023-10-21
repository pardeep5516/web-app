use actix_cors::Cors;

use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use serde::{Deserialize, Serialize};

use reqwest::Client as HttpClient;

use async_trait::async_trait;

use std::sync::Mutex;

use std::collections::HashMap;

use std::fs;

use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u64,
    name: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>,
}

impl Database {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }

    fn insert(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    fn get(&self, id: u64) -> Option<&Task> {
        self.tasks.get(&id)
    }

    fn get_all(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    fn delete(&mut self, id: u64) -> Option<Task> {
        self.tasks.remove(&id)
    }

    fn update(&mut self, id: u64, task: Task) {
        self.tasks.insert(id, task);
    }
}

fn main() {
    println!("Hello, world!");
}
