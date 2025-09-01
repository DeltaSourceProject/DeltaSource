use std::fs;
use std::collections::HashMap;

/// Struktura zasobu chmurowego
#[derive(Debug)]
struct Resource {
    rtype: String,
    name: String,
    config: HashMap<String, String>,
}

/// Struktura projektu chmurowego
#[derive(Debug)]
struct CloudProject {
    name: String,
    region: String,
    resources: Vec<Resource>,
}

impl CloudProject {
    fn new(name: &str, region: &str) -> Self {
        CloudProject {
            name: name.to_string(),
            region: region.to_string(),
            resources: Vec::new(),
        }
    }

    fn add_resource(&mut self, rtype: &str, name: &str, config: HashMap<String, String>) {
        self.resources.push(Resource {
            rtype: rtype.to_string(),
            name: name.to_string(),
            config,
        });
    }

    fn show_summary(&self) {
        println!("Projekt chmurowy: {}", self.name);
        println!("Region: {}", self.region);
        println!("Zasoby:");
        for res in &self.resources {
            println!("  - [{}] {}: {:?}", res.rtype, res.name, res.config);
        }
    }
}

fn main() {
    // Przykład konfiguracji VM
    let mut vm_config = HashMap::new();
    vm_config.insert("image".to_string(), "ubuntu-22.04".to_string());
    vm_config.insert("cpu".to_string(), "2".to_string());
    vm_config.insert("ram".to_string(), "4GB".to_string());
    vm_config.insert("disk".to_string(), "50GB".to_string());
    vm_config.insert("network".to_string(), "public".to_string());

    // Przykład konfiguracji storage
    let mut storage_config = HashMap::new();
    storage_config.insert("size".to_string(), "100GB".to_string());
    storage_config.insert("access".to_string(), "private".to_string());

    // Przykład konfiguracji bazy danych
    let mut db_config = HashMap::new();
    db_config.insert("engine".to_string(), "postgresql".to_string());
    db_config.insert("version".to_string(), "15".to_string());
    db_config.insert("cpu".to_string(), "1".to_string());
    db_config.insert("ram".to_string(), "2GB".to_string());
    db_config.insert("disk".to_string(), "20GB".to_string());
    db_config.insert("network".to_string(), "private".to_string());

    // Tworzenie projektu chmurowego
    let mut project = CloudProject::new("DeltaSource", "europe-west1");
    project.add_resource("vm", "main-server", vm_config);
    project.add_resource("storage", "data-bucket", storage_config);
    project.add_resource("database", "user-db", db_config);

    // Wyświetl podsumowanie
    project.show_summary();

    // Zapisz konfigurację do pliku (opcjonalnie)
    let summary = format!("{:#?}", project);
    fs::write("cloud_summary.txt", summary).expect("Nie można zapisać pliku");
}