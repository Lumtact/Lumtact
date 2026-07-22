use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, Router},
    Json,
};
use serde::{Deserialize, Serialize};
use std::{collections::{HashMap, HashSet}, fs, path::PathBuf, sync::Arc};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber;

// 配置常量
const MANIFEST_PATH: &str = "../docs/dag-manifest.json";
const DOCS_DIR: &str = "../docs";

// 复用生成器中的 Node 结构定义（用于反序列化）
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

// API 响应结构
#[derive(Serialize)]
struct TraceResponse {
    query_node: String,
    trace_path: Vec<String>,
    documents: Vec<Document>,
}

#[derive(Serialize)]
struct Document {
    path: String,
    title: String,
    content: String,
}

#[derive(Clone)]
struct AppState {
    manifest: HashMap<String, Node>,
    docs_root: PathBuf,
}

// 辅助函数：获取层级值
fn layer_value(layer: &str) -> u8 {
    match layer {
        "Whitepaper" => 2,
        "Engineering" => 1,
        "Implementation" => 0,
        _ => 0,
    }
}

// 辅助函数：检查是否是向上一层
fn is_upper_layer(current: &str, target: &str) -> bool {
    layer_value(target) > layer_value(current)
}

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt().init();

    // 1. 加载 Manifest
    let manifest_content = fs::read_to_string(MANIFEST_PATH).expect("Failed to read manifest");
    let nodes: Vec<Node> = serde_json::from_str(&manifest_content).expect("Failed to parse manifest");
    
    let mut manifest_map: HashMap<String, Node> = HashMap::new();
    for node in nodes {
        manifest_map.insert(node.path.clone(), node);
    }
    
    println!("🚀 Loaded {} nodes from manifest.", manifest_map.len());

    // 2. 解析文档根目录绝对路径
    let docs_root = fs::canonicalize(DOCS_DIR).expect("Failed to resolve docs root");

    let state = Arc::new(AppState {
        manifest: manifest_map,
        docs_root,
    });

    // 3. 构建路由
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/trace/*node_path", get(get_trace))
        .route("/api/v1/node/*node_path", get(get_node))
        .route("/api/v1/graph", get(get_graph))
        .layer(
            TraceLayer::new_for_http()
        )
        .layer(CorsLayer::permissive())
        .with_state(state);

    // 4. 启动服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🔥 Server listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

// 健康检查
async fn health_check() -> &'static str {
    "OK"
}

// 修复：正确的追溯逻辑 - 沿着 outbound_links 向上追溯
fn calculate_trace(node: &Node, nodes: &HashMap<String, Node>, visited: &mut HashSet<String>) -> Vec<String> {
    if visited.contains(&node.path) {
        return Vec::new();
    }
    
    visited.insert(node.path.clone());
    let mut path_list = vec![node.path.clone()];
    
    // 沿着 outbound_links 追溯，且只向上一层追溯
    for outbound_path in &node.outbound_links {
        if let Some(target_node) = nodes.get(outbound_path) {
            if is_upper_layer(&node.layer, &target_node.layer) {
                path_list.extend(calculate_trace(target_node, nodes, visited));
                break; // 只取第一个上级节点，避免多路径复杂化
            }
        }
    }
    
    path_list
}

// 获取完整追溯路径 + 文档内容
async fn get_trace(
    State(state): State<Arc<AppState>>,
    Path(node_path): Path<String>,
) -> impl IntoResponse {
    let clean_path = node_path.trim_start_matches('/').to_string();
    let clean_path = urlencoding::decode(&clean_path).unwrap_or(clean_path.as_str().into()).to_string();

    println!("🔍 Tracing: {}", clean_path);

    let node = match state.manifest.get(&clean_path) {
        Some(n) => n,
        None => {
            eprintln!("❌ Node not found: {}", clean_path);
            return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Node not found", "path": clean_path}))).into_response();
        }
    };

    // 修复：使用正确的追溯逻辑
    let mut visited = HashSet::new();
    let trace_path = calculate_trace(node, &state.manifest, &mut visited);

    println!("📊 Trace path: {:?}", trace_path);

    // 收集所有 trace_path 中的文档内容
    let mut documents = Vec::new();
    for path in &trace_path {
        let target_node = match state.manifest.get(path) {
            Some(n) => n,
            None => continue,
        };

        let file_path = state.docs_root.join(path);
        match fs::read_to_string(&file_path) {
            Ok(content) => {
                documents.push(Document {
                    path: path.clone(),
                    title: target_node.title.clone(),
                    content,
                });
            }
            Err(e) => {
                tracing::warn!("Failed to read file {}: {}", file_path.display(), e);
            }
        }
    }

    let response = TraceResponse {
        query_node: clean_path,
        trace_path,
        documents,
    };

    (StatusCode::OK, Json(response)).into_response()
}

// 获取单个节点信息（不含内容）
async fn get_node(
    State(state): State<Arc<AppState>>,
    Path(node_path): Path<String>,
) -> impl IntoResponse {
    let clean_path = node_path.trim_start_matches('/').to_string();
    let clean_path = urlencoding::decode(&clean_path).unwrap_or(clean_path.as_str().into()).to_string();
    
    println!("🔍 Getting node: {}", clean_path);
    
    match state.manifest.get(&clean_path) {
        Some(node) => (StatusCode::OK, Json(node.clone())).into_response(),
        None => {
            eprintln!("❌ Node not found: {}", clean_path);
            (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Node not found", "path": clean_path}))).into_response()
        }
    }
}

// 获取完整图数据
async fn get_graph(State(state): State<Arc<AppState>>) -> Json<HashMap<String, Node>> {
    Json(state.manifest.clone())
}
