use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// 配置
const DOCS_DIR: &str = "../docs";
const OUTPUT_FILE: &str = "../docs/dag-manifest.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Node {
    path: String,
    layer: Layer,
    title: String,
    outbound_links: Vec<String>,
    inbound_links: Vec<String>,
    trace_path: Vec<String>,
    related_nodes: Vec<String>,
    cited_by: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Layer {
    Whitepaper,
    Engineering,
    Implementation,
    Unknown,
}

impl Layer {
    fn from_path(path: &Path) -> Self {
        let path_str = path.to_string_lossy().to_lowercase();
        if path_str.contains("whitepaper") {
            Layer::Whitepaper
        } else if path_str.contains("engineering-guide") {
            Layer::Engineering
        } else if path_str.contains("implementation") {
            Layer::Implementation
        } else {
            Layer::Unknown
        }
    }
}

#[derive(Debug)]
struct ContentInfo {
    title: String,
    references: Vec<String>,
}

fn parse_file_content(filepath: &Path) -> ContentInfo {
    let content = fs::read_to_string(filepath).unwrap_or_default();
    
    let title_re = Regex::new(r"^#\s+(.+)$").unwrap();
    let title = title_re
        .captures(&content)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
        .unwrap_or_else(|| filepath.file_stem().unwrap().to_string_lossy().to_string());
    
    let link_re = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
    let mut references: Vec<String> = Vec::new();
    
    for line in content.lines() {
        if line.contains("[DERIVATION]") || line.contains("[RULE]") {
            // 修复：直接遍历 String，不进行元组解构
            for link_path in link_re.captures_iter(line).map(|c| {
                c.get(2).unwrap().as_str().to_string()
            }) {
                if link_path.starts_with("http") || link_path.starts_with('/') || !link_path.ends_with(".md") {
                    continue;
                }
                
                let abs_path = filepath.parent().unwrap().join(&link_path);
                let docs_root = Path::new(DOCS_DIR).canonicalize().unwrap();
                
                match abs_path.canonicalize() {
                    Ok(abs_path) => {
                        match abs_path.strip_prefix(&docs_root) {
                            Ok(rel_path) => {
                                references.push(rel_path.to_string_lossy().to_string());
                            }
                            Err(_) => continue,
                        }
                    }
                    Err(_) => continue,
                }
            }
        }
    }
    
    let references_set: HashSet<_> = references.into_iter().collect();
    ContentInfo {
        title,
        references: references_set.into_iter().collect(),
    }
}

fn build_dag() -> Vec<Node> {
    let docs_root = Path::new(DOCS_DIR).canonicalize().unwrap();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    
    println!("🔍 Scanning docs/ directory...");
    
    for entry in WalkDir::new(DOCS_DIR)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
    {
        let path = entry.path();
        if path.file_name().unwrap() == "README.md" {
            continue;
        }
        
        let rel_path = path.strip_prefix(&docs_root).unwrap().to_string_lossy().to_string();
        let layer = Layer::from_path(path);
        
        let content_info = parse_file_content(path);
        
        nodes.insert(
            rel_path.clone(),
            Node {
                path: rel_path.clone(),
                layer,
                title: content_info.title,
                outbound_links: content_info.references.clone(),
                inbound_links: Vec::new(),
                trace_path: Vec::new(),
                related_nodes: Vec::new(),
                cited_by: Vec::new(),
            },
        );
    }
    
    println!("✅ Found {} nodes.", nodes.len());
    
    // 修复：先收集所有需要添加的引用关系
    let mut inbound_additions: Vec<(String, String)> = Vec::new(); // (目标路径, 源路径)
    let mut related_additions: Vec<(String, String)> = Vec::new();
    let mut cited_by_additions: Vec<(String, String)> = Vec::new();
    
    for (path, node) in nodes.iter() {
        for ref_path in &node.outbound_links {
            if nodes.contains_key(ref_path) {
                let target_layer = nodes.get(ref_path).unwrap().layer;
                
                // 记录反向链接
                inbound_additions.push((ref_path.clone(), path.clone()));
                
                // 判断是否为同层关联
                if node.layer as u8 == target_layer as u8 {
                    related_additions.push((path.clone(), ref_path.clone()));
                }
                
                // 判断是否为下层引用上层
                let is_down_to_up = 
                    (matches!(node.layer, Layer::Implementation) && matches!(target_layer, Layer::Engineering)) ||
                    (matches!(node.layer, Layer::Engineering) && matches!(target_layer, Layer::Whitepaper));
                
                if is_down_to_up {
                    cited_by_additions.push((ref_path.clone(), path.clone()));
                }
            }
        }
    }
    
    // 第二遍遍历：应用所有修改
    for (target_path, source_path) in inbound_additions {
        if let Some(node) = nodes.get_mut(&target_path) {
            if !node.inbound_links.contains(&source_path) {
                node.inbound_links.push(source_path);
            }
        }
    }
    
    for (path, ref_path) in related_additions {
        if let Some(node) = nodes.get_mut(&path) {
            if !node.related_nodes.contains(&ref_path) {
                node.related_nodes.push(ref_path);
            }
        }
    }
    
    for (target_path, source_path) in cited_by_additions {
        if let Some(node) = nodes.get_mut(&target_path) {
            if !node.cited_by.contains(&source_path) {
                node.cited_by.push(source_path);
            }
        }
    }
    
    // 计算完整上溯路径 (trace_path)
    fn calculate_trace(node: &Node, nodes: &HashMap<String, Node>, visited: &mut HashSet<String>) -> Vec<String> {
        if visited.contains(&node.path) {
            return Vec::new();
        }
        
        visited.insert(node.path.clone());
        let mut path_list = vec![node.path.clone()];
        
        if let Some(parent_path) = node.inbound_links.first() {
            if let Some(parent_node) = nodes.get(parent_path) {
                path_list.extend(calculate_trace(parent_node, nodes, visited));
            }
        }
        
        path_list
    }
    
    let mut final_nodes: Vec<Node> = Vec::new();
    for (_, node) in nodes.iter() {
        let mut visited = HashSet::new();
        let trace_path = calculate_trace(node, &nodes, &mut visited);
        
        let mut node_with_trace = node.clone();
        node_with_trace.trace_path = trace_path;
        final_nodes.push(node_with_trace);
    }
    
    final_nodes.sort_by(|a, b| {
        let layer_order = |layer: &Layer| -> u8 {
            match layer {
                Layer::Whitepaper => 0,
                Layer::Engineering => 1,
                Layer::Implementation => 2,
                Layer::Unknown => 99,
            }
        };
        
        match layer_order(&a.layer).cmp(&layer_order(&b.layer)) {
            std::cmp::Ordering::Equal => a.path.cmp(&b.path),
            other => other,
        }
    });
    
    final_nodes
}

fn main() {
    let dag = build_dag();
    
    let output_json = serde_json::to_string_pretty(&dag).expect("Failed to serialize JSON");
    fs::write(OUTPUT_FILE, output_json).expect("Failed to write output file");
    
    println!("💾 Generated {}", OUTPUT_FILE);
}
