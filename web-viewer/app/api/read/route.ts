import { NextResponse } from 'next/server';
import fs from 'fs';
import path from 'path';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const docPath = searchParams.get('path');

  if (!docPath) {
    return NextResponse.json({ error: 'Path missing' }, { status: 400 });
  }

  // 安全检查：确保路径在 docs 目录下
  const absolutePath = path.resolve(process.cwd(), '../docs', docPath);
  
  // 简单的防穿透检查 (实际生产环境需要更严谨)
  if (!absolutePath.startsWith(path.resolve(process.cwd(), '../docs'))) {
     return NextResponse.json({ error: 'Forbidden' }, { status: 403 });
  }

  try {
    const content = fs.readFileSync(absolutePath, 'utf-8');
    // 返回纯文本
    return new NextResponse(content, {
      status: 200,
      headers: { 'Content-Type': 'text/plain; charset=utf-8' },
    });
  } catch (error) {
    return NextResponse.json({ error: 'File not found' }, { status: 404 });
  }
}
