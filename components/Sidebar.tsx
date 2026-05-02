"use client";

import {
  Home,
  FolderKanban,
  PackageOpen,
  Box,
  Download,
  Settings,
} from "lucide-react";

export type PageId = "home" | "my-projects" | "fab";

const navItems: {
  icon: typeof Home;
  label: string;
  activeOn: PageId | null;
}[] = [
  { icon: Home, label: "首页", activeOn: "home" },
  { icon: FolderKanban, label: "我的工程", activeOn: "my-projects" },
  { icon: PackageOpen, label: "Fab 库存", activeOn: "fab" },
  { icon: Box, label: "引擎管理", activeOn: null },
  { icon: Download, label: "下载队列", activeOn: null },
];

interface SidebarProps {
  activePage: PageId;
  onNavigate: (page: PageId) => void;
}

export default function Sidebar({ activePage, onNavigate }: SidebarProps) {
  return (
    <aside className="w-[260px] h-full shrink-0 rounded-[22px] bg-[#111318B8] backdrop-blur-[20px] border border-launcher-border flex flex-col overflow-hidden">
      {/* Brand */}
      <div className="flex items-center gap-3 px-5 pt-5 pb-3">
        <div className="w-11 h-11 rounded-xl bg-white flex items-center justify-center border border-[#FFFFFF66]">
          <span className="font-mono text-xs font-bold text-launcher-bg leading-none">
            EPIC
          </span>
        </div>
        <div className="flex flex-col gap-0.5">
          <span className="font-sans text-lg font-semibold text-white">
            Epic Games
          </span>
          <span className="font-mono text-[11px] text-launcher-textSecondary">
            Launcher concept
          </span>
        </div>
      </div>

      {/* Section Label */}
      <div className="px-5 pt-3 pb-2">
        <span className="font-mono text-[11px] font-semibold text-launcher-textMuted tracking-wide">
          WORKSPACE
        </span>
      </div>

      {/* Nav Items */}
      <nav className="flex flex-col gap-1 px-5 flex-1">
        {navItems.map((item) => {
          const Icon = item.icon;
          const isActive = item.activeOn !== null && activePage === item.activeOn;
          return (
            <button
              key={item.label}
              onClick={() => {
                if (item.activeOn) onNavigate(item.activeOn);
              }}
              className={`flex items-center gap-3.5 h-12 rounded-2xl px-3 transition-all duration-200 ${
                isActive
                  ? "bg-launcher-active text-white border border-launcher-borderLight"
                  : "text-launcher-textSecondary hover:bg-launcher-hover border border-transparent"
              }`}
            >
              <Icon size={20} />
              <span className="font-sans text-[15px] font-normal">
                {item.label}
              </span>
            </button>
          );
        })}
      </nav>

      {/* Settings pinned to bottom */}
      <div className="px-5 pb-5">
        <button className="flex items-center gap-3.5 h-12 rounded-2xl px-3 text-launcher-textSecondary hover:bg-launcher-hover w-full transition-colors">
          <Settings size={20} />
          <span className="font-sans text-[15px] font-normal">设置</span>
        </button>
      </div>
    </aside>
  );
}
