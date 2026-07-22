use pulldown_cmark::{Event, Parser, Tag, TagEnd};
/// 从 Markdown 文本中提取真实的文档链接
pub fn extract_markdown_links(content: &str) -> Vec<String> {
    let parser = Parser::new(content);
    let mut links = Vec::new();
    let mut in_code_block = false;
    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(_)) => {
                in_code_block = true;
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                if !in_code_block {
                    links.push(dest_url.to_string());
                }
            }
            _ => {}
        }
    }
    links
}
// --- 单元测试 ---
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ast_ignores_code_block_links() {
        let content = r#"
# Test Document
这是一个真实的文档引用: [布局规范](../../engineering-guide/layout/sizing.md)。
下面的代码块里虽然也有链接格式，但应该被忽略：
```jsx
import { Button } from '@lumtact/ui';
// 下面这段看起来像链接，但在代码里，不应该被提取
[链接到指南](../../engineering-guide/tokens/color-gene.md)
```
结束。
"#;
        let links = extract_markdown_links(content);
        
        // 断言：只提取了一个真实链接
        assert_eq!(links.len(), 1);
        assert_eq!(links[0], "../../engineering-guide/layout/sizing.md");
        
        // 验证没有提取到代码块里的那个链接
        assert!(!links.contains(&"../../engineering-guide/tokens/color-gene.md".to_string()));
    }
}
