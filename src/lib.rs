use pyo3::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use walkdir::WalkDir; // The folder walker

#[derive(Serialize, Deserialize)]
struct GraphData {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>,
}

#[pyclass]
struct BranchoRAG {
    data: GraphData,
}

#[pymethods]
impl BranchoRAG {
    #[new]
    fn new() -> Self {
        BranchoRAG {
            data: GraphData { nodes: Vec::new(), edges: Vec::new() },
        }
    }

    // The "Eyes": Scans a folder and adds every file as a node
    fn scan_folder(&mut self, path: String) -> PyResult<()> {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let name = entry.path().display().to_string();
                if !self.data.nodes.contains(&name) {
                    self.data.nodes.push(name);
                }
            }
        }
        Ok(())
    }

    fn save_memory(&self, filename: String) -> PyResult<()> {
        let json = serde_json::to_string_pretty(&self.data).unwrap();
        let mut file = File::create(filename)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

#[pymodule]
fn branchorag(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<BranchoRAG>()?;
    Ok(())
}
