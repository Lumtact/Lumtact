use std::fs::File;
use std::io::Write;
use lumtract_dag::core::DAG;

fn main() {
    println!("🚀 Starting DAG Generation...");

    // 创建一个简单的 DAG
    let mut dag = DAG::new("main");
    
    // 构建结构 (示例)
    let a = dag.add_node("A", "Root");
    let b = dag.add_node("B", "Process");
    let c = dag.add_node("C", "Output");
    
    dag.add_edge(a, b);
    dag.add_edge(b, c);

    // 序列化
    let json_str = serde_json::to_string_pretty(&dag).expect("Failed to serialize");

    // 🎯 关键修改：直接输出到前端的公共目录
    // 这符合 [PURPOSE]：生成数据就是为了被前端直接消费，减少中间搬运环节
    let output_path = "../web-viewer/public/dag-data.json";
    
    let mut file = File::create(output_path).expect("Failed to create file");
    file.write_all(json_str.as_bytes()).expect("Failed to write");

    println!("✅ Manifest generated: {}", output_path);
}
