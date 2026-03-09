use pyo3::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;
use walkdir::WalkDir;

// --- BLOCK 1: DATA ---
#[derive(Serialize, Deserialize, Clone)]
struct FileNode {
    path: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct GraphData {
    nodes: Vec<FileNode>,
    edges: Vec<(usize, usize)>,
}

#[pyclass]
struct BranchoRAG {
    data: GraphData,
}

// --- BLOCK 2: METHODS ---
#[pymethods]
impl BranchoRAG {
    #[new]
    fn new() -> Self {
        BranchoRAG {
            data: GraphData { nodes: Vec::new(), edges: Vec::new() },
        }
    }

    fn scan_folder(&mut self, path: String) -> PyResult<()> {
        let ignore_list = ["target", ".git", ".venv", "__pycache__", "env", "node_modules"];

        // Skip files larger than 1MB to avoid bloating memory with minified JS, logs, etc.
        const MAX_FILE_BYTES: u64 = 1_000_000;

        // Track paths we've already seen to avoid duplicates
        let mut seen: HashSet<String> = self.data.nodes.iter().map(|n| n.path.clone()).collect();

        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let is_ignored = entry.path().components().any(|c| {
                ignore_list.contains(&c.as_os_str().to_str().unwrap_or(""))
            });
            if is_ignored {
                continue;
            }

            if entry.file_type().is_file() {
                let path_str = entry.path().display().to_string();

                if !seen.contains(&path_str) {
                    // Skip files that are too large before attempting to read them
                    let size = entry.metadata().map(|m| m.len()).unwrap_or(u64::MAX);
                    if size > MAX_FILE_BYTES {
                        continue;
                    }

                    // read_to_string naturally skips binary files that aren't valid UTF-8
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        self.data.nodes.push(FileNode {
                            path: path_str.clone(),
                            content,
                        });
                        seen.insert(path_str);
                    }
                }
            }
        }
        Ok(())
    }

    fn save_memory(&self, filename: String) -> PyResult<()> {
        let json = serde_json::to_string_pretty(&self.data)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        let mut file = File::create(filename)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    fn node_count(&self) -> usize {
        self.data.nodes.len()
    }
}

// --- BLOCK 3: THE MODULE ---
#[pymodule]
fn branchorag(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BranchoRAG>()?;
    Ok(())
}
