use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path; // 移除了 PathBuf
use glob::glob;
use serde::{Deserialize, Serialize};

mod parser;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Node {
    path: String,
    layer: String,
    title: String,
    #[serde(default)]
    outbound_links: Vec<String>,
    #[serde(default)]
    inbound_links: Vec<String>,
    #[serde(default)]
    trace_path: Vec<String>,
    #[serde(default)]
    related_nodes: Vec<String>,
    #[serde(default)]
    cited_by: Vec<String>,
}

fn main() {
    println!("🚀 Starting DAG Generation...");
    
    // 🔧 修改1: 指向新位置
    let docs_dir = "../web-viewer/public/docs/**/*.md";
    let mut nodes: Vec<Node> = Vec::new();
    let mut path_to_index: HashMap<String, usize> = HashMap::new();

    // 🔧 修改2: 指向新位置
    let docs_root_abs = fs::canonicalize("../web-viewer/public/docs")
        .expect("Failed to resolve docs root");

    for entry in glob(docs_dir).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if let Some(content) = read_file_content(&path) {
                    if let Ok(file_abs) = fs::canonicalize(&path) {
                        let raw_links = parser::extract_markdown_links(&content);
                        let outbound_links = normalize_links(&file_abs, &docs_root_abs, raw_links);
                        
                        let relative_path = file_abs
                            .strip_prefix(&docs_root_abs)
                            .unwrap_or(&file_abs)
                            .to_str()
                            .unwrap()
                            .replace("\\", "/");

                        let layer = detect_layer(&relative_path);
                        let title = extract_title(&content);

                        let idx = nodes.len();
                        path_to_index.insert(relative_path.clone(), idx);

                        nodes.push(Node {
                            path: relative_path,
                            layer,
                            title,
                            outbound_links,
                            ..Default::default()
                        });
                    }
                }
            }
            Err(e) => println!("Error reading path: {:?}", e),
        }
    }

    for i in 0..nodes.len() {
        let outbound_links = nodes[i].outbound_links.clone();
        let source_path = nodes[i].path.clone();

        for target_path in outbound_links {
            if let Some(&target_idx) = path_to_index.get(&target_path) {
                nodes[target_idx].inbound_links.push(source_path.clone());
            }
        }
    }

    for i in 0..nodes.len() {
        let mut visited = HashSet::new();
        nodes[i].trace_path = calculate_trace(&nodes[i], &nodes, &mut visited);
    }

    let json = serde_json::to_string_pretty(&nodes).expect("Failed to serialize");
    
    // 🔧 修改3: 输出到 public 目录
    fs::write("../web-viewer/public/dag-manifest.json", json)
        .expect("Failed to write manifest");
    println!("✅ Manifest generated: ../web-viewer/public/dag-manifest.json");
}

fn read_file_content(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok()
}

fn normalize_links(current_file_abs: &Path, docs_root_abs: &Path, raw_links: Vec<String>) -> Vec<String> {
    let mut normalized = Vec::new();
    let current_dir = current_file_abs.parent().unwrap_or(current_file_abs);
    let mut unique_links = HashSet::new();

    for link in raw_links {
        if link.starts_with("http") || link.starts_with("#") {
            continue;
        }
        
        let clean_link = link.split('#').next().unwrap_or(&link);
        let target_path = current_dir.join(clean_link);
        
        if let Ok(canonical_target) = fs::canonicalize(&target_path) {
            if canonical_target.extension().map_or(false, |ext| ext == "md") {
                if let Ok(rel_path) = canonical_target.strip_prefix(docs_root_abs) {
                    let path_str = rel_path.to_str().unwrap().replace("\\", "/");
                    if unique_links.insert(path_str.clone()) {
                        normalized.push(path_str);
                    }
                }
            }
        }
    }
    normalized
}

fn detect_layer(path: &str) -> String {
    if path.starts_with("whitepaper") {
        "Whitepaper".to_string()
    } else if path.starts_with("engineering-guide") {
        "Engineering".to_string()
    } else if path.starts_with("implementation") {
        "Implementation".to_string()
    } else {
        "Unknown".to_string()
    }
}

fn extract_title(content: &str) -> String {
    content
        .lines()
        .find(|line| line.starts_with("#"))
        .and_then(|line| {
            line.split('#')
                .nth(1)
                .map(|s| s.trim().to_string())
        })
        .unwrap_or_else(|| "Untitled".to_string())
}

fn calculate_trace(node: &Node, nodes: &[Node], visited: &mut HashSet<String>) -> Vec<String> {
    if visited.contains(&node.path) {
        return Vec::new();
    }
    visited.insert(node.path.clone());
    
    let mut path = vec![node.path.clone()];
    
    if let Some(outbound_path) = node.outbound_links.first() {
        if let Some(target) = nodes.iter().find(|n| &n.path == outbound_path) {
            if is_upper_layer(&node.layer, &target.layer) {
                path.extend(calculate_trace(target, nodes, visited));
            }
        }
    }
    path
}

fn is_upper_layer(current: &str, target: &str) -> bool {
    let level = |l: &str| -> u8 {
        match l {
            "Whitepaper" => 2,
            "Engineering" => 1,
            "Implementation" => 0,
            _ => 0,
        }
    };
    level(target) > level(current)
}

impl Default for Node {
    fn default() -> Self {
        Node {
            path: String::new(),
            layer: String::new(),
            title: String::new(),
            outbound_links: Vec::new(),
            inbound_links: Vec::new(),
            trace_path: Vec::new(),
            related_nodes: Vec::new(),
            cited_by: Vec::new(),
        }
    }
}
