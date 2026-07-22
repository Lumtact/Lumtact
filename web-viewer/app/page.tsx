'use client';

import { useState, useEffect } from 'react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

interface Node {
  path: string;
  layer: string;
  title: string;
  trace_path: string[];
  outbound_links: string[];
}

export default function KnowledgeHub() {
  const [manifest, setManifest] = useState<Node[] | null>(null);
  const [selectedNode, setSelectedNode] = useState<Node | null>(null);
  const [markdownContent, setMarkdownContent] = useState<string>('');

  // 1. 加载 Manifest
  useEffect(() => {
    fetch('/dag-manifest.json')
      .then(res => res.json())
      .then((data: Node[]) => setManifest(data))
      .catch(console.error);
  }, []);

  // 2. 如果有数据但没选中，默认选中白皮书首页
  useEffect(() => {
    if (manifest && !selectedNode) {
      const whitepaper = manifest.find(n => n.layer === 'Whitepaper' && n.path.includes('index'));
      if (whitepaper) {
        setSelectedNode(whitepaper);
      } else {
        const root = manifest.find(n => n.layer === 'Whitepaper');
        if (root) setSelectedNode(root);
      }
    }
  }, [manifest, selectedNode]);

  // 3. 加载内容
  useEffect(() => {
    if (selectedNode) {
      fetch(`/api/read?path=${encodeURIComponent(selectedNode.path)}`)
        .then(res => res.text())
        .then(text => setMarkdownContent(text))
        .catch(console.error);
    }
  }, [selectedNode]);

  // --- 4. 工具函数：根据路径查找节点 ---
  const findNodeByPath = (path: string): Node | undefined => {
    return manifest?.find(n => n.path === path);
  };

  // --- 5. 处理内部链接点击 ---
  const handleLinkClick = (href: string) => {
    // 外部链接或锚点，不拦截
    if (href.startsWith('http') || href.startsWith('#')) {
      window.open(href, '_blank');
      return;
    }

    // 去除可能存在的 .md 扩展名和 # 锚点
    let targetPath = href.replace(/\.md$/, '').split('#')[0];
    
    // 尝试直接匹配
    let target = findNodeByPath(targetPath);
    
    // 如果没找到，尝试添加 .md 后缀
    if (!target) {
      target = findNodeByPath(`${targetPath}.md`);
    }
    
    // 如果还没找到，尝试在 engineering-guide 等子目录中查找
    if (!target) {
      // 尝试常见的 docs 子路径
      const candidates = [
        `engineering-guide/${targetPath}`,
        `implementation/${targetPath}`,
        `whitepaper-archive/${targetPath}`,
        `${targetPath}`,
      ];
      for (const cand of candidates) {
        const found = findNodeByPath(cand);
        if (found) {
          target = found;
          break;
        }
      }
    }

    if (target) {
      setSelectedNode(target);
    } else {
      console.warn(`[Link] 未找到节点: ${href}`);
      // 作为后备，尝试直接打开文件（浏览器会显示 raw）
      // 但为了体验，我们可以提示用户
      alert(`文档 "${href}" 未在知识图谱中找到，是否查看原始文件？`);
      window.open(`/api/read?path=${encodeURIComponent(href)}`, '_blank');
    }
  };

  // --- 渲染逻辑 ---

  if (!manifest) return <div className="h-screen flex items-center justify-center text-indigo-600 dark:text-indigo-400 font-mono">系统初始化中...</div>;

  const layers = ['Whitepaper', 'Engineering', 'Implementation'];

  return (
    <div className="flex h-screen bg-gray-50 text-gray-900 dark:bg-gray-900 dark:text-gray-100 transition-colors duration-200">
      
      {/* 左侧：分层导航 */}
      <aside className="w-80 bg-white/80 backdrop-blur-md border-r border-gray-200 dark:border-gray-800 dark:bg-gray-900/80 overflow-y-auto">
        <div className="p-6 border-b border-gray-200 dark:border-gray-800">
          <h1 className="text-2xl font-bold bg-gradient-to-r from-indigo-600 to-violet-600 bg-clip-text text-transparent">
            Lumtract
          </h1>
          <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">水质波光 · 知识图谱</p>
        </div>

        <div className="p-4 space-y-6">
          {layers.map((layerName) => {
            const layerNodes = manifest.filter(n => n.layer === layerName);
            if (layerNodes.length === 0) return null;

            const layerConfig = {
              'Whitepaper': { color: 'text-purple-600 dark:text-purple-400', icon: '💎' },
              'Engineering': { color: 'text-blue-600 dark:text-blue-400', icon: '🔧' },
              'Implementation': { color: 'text-emerald-600 dark:text-emerald-400', icon: '🧩' },
            };
            const config = layerConfig[layerName as keyof typeof layerConfig];

            return (
              <div key={layerName}>
                <div className={`flex items-center gap-2 text-xs font-bold uppercase tracking-wider mb-3 ${config.color}`}>
                  <span>{config.icon}</span>
                  <span>{layerName}</span>
                </div>
                <div className="space-y-1 pl-1">
                  {layerNodes.map(node => (
                    <div 
                      key={node.path}
                      onClick={() => setSelectedNode(node)}
                      className={`group cursor-pointer p-3 rounded-lg transition-all duration-200 border border-transparent ${
                        selectedNode?.path === node.path 
                          ? 'bg-indigo-50 border-indigo-200 dark:bg-indigo-900/30 dark:border-indigo-700/50' 
                          : 'hover:bg-gray-100 hover:border-gray-200 dark:hover:bg-gray-800 dark:hover:border-gray-700'
                      }`}
                    >
                      <div className={`font-medium text-sm line-clamp-2 ${
                        selectedNode?.path === node.path ? 'text-indigo-700 dark:text-indigo-300' : 'text-gray-700 dark:text-gray-300'
                      }`}>
                        {node.title}
                      </div>
                      {node.trace_path.length > 1 && (
                        <div className="text-xs text-gray-400 mt-1 flex items-center gap-1">
                           <span className="w-1.5 h-1.5 rounded-full bg-gray-300 dark:bg-gray-600"></span>
                           {node.trace_path.length} 步追溯
                        </div>
                      )}
                    </div>
                  ))}
                </div>
              </div>
            );
          })}
        </div>
      </aside>

      {/* 右侧：阅读器 */}
      <main className="flex-1 overflow-y-auto bg-gray-50/50 dark:bg-gray-900/50">
        {selectedNode ? (
          <article className="max-w-3xl mx-auto min-h-screen py-12 px-8">
            
            {/* 面包屑导航 */}
            <nav className="mb-8 flex flex-wrap items-center gap-2 text-sm font-medium text-gray-500 dark:text-gray-400">
              {selectedNode.trace_path.map((p, i) => (
                <span key={i} className="flex items-center gap-2 hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors">
                  {p.split('/').pop()}
                  {i < selectedNode.trace_path.length - 1 && (
                    <svg className="w-4 h-4 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9 5l7 7-7 7"></path></svg>
                  )}
                </span>
              ))}
            </nav>

            {/* 标题区 */}
            <header className="mb-10 pb-8 border-b border-gray-200 dark:border-gray-800">
              <div className="inline-block px-3 py-1 mb-4 text-xs font-semibold tracking-wider text-white rounded-full bg-gradient-to-r from-indigo-500 to-violet-500">
                {selectedNode.layer}
              </div>
              <h1 className="text-4xl font-extrabold tracking-tight text-gray-900 dark:text-white mb-6">
                {selectedNode.title}
              </h1>
            </header>

            {/* 内容渲染区：支持暗黑模式 + 内部链接拦截 */}
            <div className="prose prose-lg prose-indigo dark:prose-invert dark:prose-dark max-w-none prose-headings:font-bold prose-a:text-indigo-600 dark:prose-a:text-indigo-400 prose-a:no-underline hover:prose-a:underline">
              <ReactMarkdown 
                remarkPlugins={[remarkGfm]}
                components={{
                  // 🔧 核心修复：拦截链接点击
                  a: ({ href, children }) => {
                    if (!href) return <>{children}</>;
                    
                    // 外部链接或锚点：正常打开
                    if (href.startsWith('http') || href.startsWith('#')) {
                      return <a href={href} target="_blank" rel="noopener noreferrer">{children}</a>;
                    }

                    // 内部链接：通过 handleLinkClick 处理
                    return (
                      <a
                        href={href}
                        onClick={(e) => {
                          e.preventDefault();
                          handleLinkClick(href);
                        }}
                        className="cursor-pointer text-indigo-600 dark:text-indigo-400 hover:underline"
                        title={`点击加载 "${href}"`}
                      >
                        {children}
                      </a>
                    );
                  },
                }}
              >
                {markdownContent}
              </ReactMarkdown>
            </div>

            {/* 底部依赖链 */}
            {selectedNode.outbound_links.length > 0 && (
              <div className="mt-16 pt-8 border-t border-gray-200 dark:border-gray-800">
                <h3 className="text-sm font-bold uppercase tracking-wider text-gray-500 dark:text-gray-400 mb-4">
                  引用的核心规范
                </h3>
                <div className="flex flex-wrap gap-3">
                  {selectedNode.outbound_links.map(link => {
                     const target = manifest.find(n => n.path === link);
                     return target ? (
                       <button 
                         key={link} 
                         onClick={() => setSelectedNode(target)}
                         className="flex items-center gap-2 px-4 py-2 bg-white border border-gray-200 rounded-lg shadow-sm hover:shadow-md hover:border-indigo-300 dark:bg-gray-800 dark:border-gray-700 dark:hover:border-indigo-500 transition-all text-left group"
                       >
                         <span className="w-2 h-2 rounded-full bg-blue-500 group-hover:scale-125 transition-transform"></span>
                         <div>
                           <div className="text-xs text-gray-500 dark:text-gray-400">{target.layer}</div>
                           <div className="text-sm font-semibold text-gray-800 dark:text-gray-200">{target.title}</div>
                         </div>
                       </button>
                     ) : null;
                  })}
                </div>
              </div>
            )}
          </article>
        ) : (
          <div className="h-full flex flex-col items-center justify-center text-gray-400 dark:text-gray-600">
            <div className="text-6xl mb-6 animate-pulse">📖</div>
            <h2 className="text-2xl font-bold mb-2">欢迎来到 Lumtract</h2>
            <p>正在加载知识图谱...</p>
          </div>
        )}
      </main>
    </div>
  );
}
