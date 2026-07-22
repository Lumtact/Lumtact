import { NextResponse } from 'next/server';
import fs from 'fs';
import path from 'path';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const docPath = searchParams.get('path');

  if (!docPath) {
    return NextResponse.json({ error: 'Path missing' }, { status: 400 });
  }

  // 🔧 修复：文档现在在 public/docs/ 下
  const docsRoot = path.join(process.cwd(), 'public', 'docs');
  const absolutePath = path.resolve(docsRoot, docPath);

  // 安全检查：确保路径在 public/docs 目录下，防止目录遍历攻击
  if (!absolutePath.startsWith(docsRoot)) {
    return NextResponse.json({ error: 'Forbidden' }, { status: 403 });
  }

  // 检查文件是否存在
  if (!fs.existsSync(absolutePath)) {
    return NextResponse.json({ 
      error: 'File not found', 
      path: docPath,
      resolved: absolutePath 
    }, { status: 404 });
  }

  try {
    const content = fs.readFileSync(absolutePath, 'utf-8');
    return new NextResponse(content, {
      status: 200,
      headers: { 'Content-Type': 'text/markdown; charset=utf-8' },
    });
  } catch (error) {
    return NextResponse.json({ error: 'Failed to read file' }, { status: 500 });
  }
}
