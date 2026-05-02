"use client";

import { Search, Moon, Sun, ChevronDown } from "lucide-react";

interface TopBarProps {
  kicker: string;
  title: string;
}

export default function TopBar({ kicker, title }: TopBarProps) {
  return (
    <div className="flex items-center justify-between h-[72px] rounded-[22px] bg-[#111318AA] backdrop-blur-[18px] border border-launcher-borderLight px-[18px] py-3 shrink-0">
      {/* Title Area */}
      <div className="flex flex-col gap-[3px]">
        <span className="font-mono text-[11px] font-semibold text-launcher-textMuted">
          {kicker}
        </span>
        <span className="font-sans text-2xl font-bold text-white">{title}</span>
      </div>

      {/* Actions */}
      <div className="flex items-center gap-3">
        {/* Search */}
        <div className="flex items-center gap-2.5 h-11 w-[260px] rounded-[22px] bg-launcher-hover border border-launcher-borderLight px-3.5 py-2.5">
          <Search size={18} className="text-launcher-textDark" />
          <span className="font-sans text-[13px] text-launcher-textDark">
            搜索工程、资产、插件
          </span>
        </div>

        {/* Theme Switch */}
        <div className="flex items-center gap-1.5 h-11 w-[164px] rounded-[22px] bg-launcher-hover border border-launcher-border p-1">
          <div className="flex items-center justify-center gap-1.5 flex-1 h-full rounded-[18px] bg-white shadow-[0_4px_16px_rgba(0,0,0,0.67)]">
            <Moon size={16} className="text-launcher-bg" />
            <span className="font-sans text-xs font-semibold text-launcher-bg">
              Dark
            </span>
          </div>
          <div className="flex items-center justify-center gap-1.5 flex-1 h-full rounded-[18px]">
            <Sun size={16} className="text-launcher-textSecondary" />
            <span className="font-sans text-xs font-medium text-launcher-textSecondary">
              Light
            </span>
          </div>
        </div>

        {/* Account Chip */}
        <div className="flex items-center gap-2.5 h-11 rounded-[22px] bg-launcher-hover border border-launcher-border py-1.5 pl-1.5 pr-3.5">
          <div className="w-8 h-8 rounded-2xl bg-launcher-accent flex items-center justify-center">
            <span className="font-sans text-sm font-bold text-white">L</span>
          </div>
          <span className="font-sans text-[13px] font-semibold text-white">
            登录账户
          </span>
          <ChevronDown size={16} className="text-launcher-textSecondary" />
        </div>
      </div>
    </div>
  );
}
