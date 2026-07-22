import React from 'react';

interface ButtonProps {
  mode?: 'solid' | 'glass' | 'ghost';
  size?: 'sm' | 'md' | 'lg';
  children: React.ReactNode;
  onClick?: () => void;
}

export const LumtractButton = ({ 
  mode = 'solid', 
  size = 'md', 
  children, 
  onClick 
}: ButtonProps) => {
  // 基础骨架：硅基的精确（去除了完美的圆形，改为更自然的圆角）
  const base = "rounded-xl font-semibold transition-all duration-300 active:scale-95 relative overflow-hidden group";
  
  // 样式体系：碳基的流动
  const styles = {
    // 实心水滴：深蓝背景 + 内部微光
    solid: "bg-blue-600 text-white shadow-[0_4px_14px_0_rgba(37,99,235,0.39)] hover:bg-blue-500 hover:shadow-[0_6px_20px_rgba(37,99,235,0.23)] border border-blue-500/20",
    
    // 玻璃态：半透明 + 模糊边框 (这是最像“水”的状态)
    glass: "bg-white/5 backdrop-blur-md text-blue-100 border border-white/10 hover:bg-white/10 hover:border-white/20 shadow-[0_2px_10px_rgba(255,255,255,0.05)]",
    
    // 幽灵：最轻量，仅保留文字光感
    ghost: "text-blue-400 hover:text-blue-200 hover:bg-blue-500/10"
  };

  const sizes = {
    sm: "px-4 py-1.5 text-sm",
    md: "px-6 py-2.5 text-base",
    lg: "px-8 py-3 text-lg"
  };

  // 增加一点“高光”效果：模拟水面反光
  const highlight = mode === 'solid' 
    ? `<span class="absolute top-0 left-0 w-full h-full bg-gradient-to-r from-transparent via-white/20 to-transparent -translate-x-full group-hover:animate-[shimmer_1.5s_infinite]" />` 
    : '';

  return (
    <button 
      className={`${base} ${styles[mode]} ${sizes[size]}`}
      onClick={onClick}
      // 移除了 ring-offset，改用 outline:none 消除默认丑陋的外圈
      style={{ outline: 'none' }}
    >
      {children}
      {mode === 'solid' && (
        <span className="absolute top-0 left-0 w-full h-full bg-gradient-to-r from-transparent via-white/10 to-transparent -translate-x-full group-hover:animate-[shimmer_1.5s_infinite]" />
      )}
    </button>
  );
};
