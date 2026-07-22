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
  // 硅基骨架：碳基的有机圆角
  const base = "rounded-xl font-semibold transition-all duration-300 active:scale-95 relative overflow-hidden group";
  
  // 基于公理推导的样式体系
  const styles = {
    // Solid: 高能凝聚态
    solid: "bg-blue-500 text-white shadow-[0_0_15px_rgba(59,130,246,0.5)] hover:bg-blue-400 hover:shadow-[0_0_25px_rgba(59,130,246,0.6)] border border-blue-400",
    
    // Glass: 丁达尔效应 - 模拟光束穿过水体
    // 推导点：text-white (光源) + shadow (散射) + backdrop-blur (水体密度)
    glass: "bg-blue-500/10 backdrop-blur-md text-white border border-white/10 shadow-[0_0_10px_rgba(255,255,255,0.1)] hover:bg-blue-500/20 hover:shadow-[0_0_20px_rgba(59,130,246,0.4)] hover:border-white/30",
    
    // Ghost: 能量离散态
    // 推导点：默认仅微亮，悬浮时能量聚集产生光晕，而非生硬色块
    ghost: "text-blue-50 hover:text-white hover:bg-blue-500/15 hover:shadow-[0_0_15px_rgba(59,130,246,0.2)]"
  };

  const sizes = {
    sm: "px-4 py-1.5 text-sm",
    md: "px-6 py-2.5 text-base",
    lg: "px-8 py-3 text-lg"
  };

  return (
    <button 
      className={`${base} ${styles[mode]} ${sizes[size]}`}
      onClick={onClick}
      style={{ outline: 'none' }}
    >
      {children}
      {/* 仅在 Solid 模式保留高光流过，模拟液面极速掠过 */}
      {mode === 'solid' && (
        <span className="absolute top-0 left-0 w-full h-full bg-gradient-to-r from-transparent via-white/20 to-transparent -translate-x-full group-hover:animate-[shimmer_1.5s_infinite]" />
      )}
    </button>
  );
};
