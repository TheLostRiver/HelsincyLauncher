"use client";

import {
  FolderOpen,
  RefreshCw,
  Play,
  Trash2,
  MoreHorizontal,
  Plus,
  ChevronDown,
  Plug,
} from "lucide-react";

const engines = [
  {
    version: "UE 5.4.4",
    railColor: "bg-[#8AD7FF]",
    badge: "默认",
    badgeColor: "bg-[#103D30] text-[#B6FFCE] border-[#4DE0A766]",
    path: "D:/Epic/UE_5.4",
    size: "52.6 GB",
    plugins: 18,
    isDefault: true,
  },
  {
    version: "UE 5.3.2",
    railColor: "bg-[#F8C27A]",
    badge: "稳定",
    badgeColor: "bg-[#3D2E10] text-[#F8C27A] border-[#F8C27A66]",
    path: "E:/Unreal/UE_5.3",
    size: "48.1 GB",
    plugins: 11,
    isDefault: false,
  },
  {
    version: "UE 4.27.2",
    railColor: "bg-[#8F7CFF]",
    badge: "旧项目",
    badgeColor: "bg-[#2A1F4E] text-[#8F7CFF] border-[#8F7CFF66]",
    path: "D:/Epic/UE_4.27",
    size: "43.1 GB",
    plugins: 7,
    isDefault: false,
  },
];

const plugins = [
  { name: "Fab Bridge", state: "启用", stateColor: "text-[#B6FFCE]" },
  { name: "Quixel Tools", state: "更新", stateColor: "text-launcher-warning" },
  { name: "MetaHuman Animator", state: "启用", stateColor: "text-[#B6FFCE]" },
  { name: "RiderLink", state: "禁用", stateColor: "text-launcher-textDark" },
];

export default function EngineManagementContent() {
  return (
    <div className="flex gap-[18px] flex-1 min-h-0">
      {/* Installed Engines Work Area */}
      <div className="flex flex-col gap-4 flex-1 min-w-0 rounded-3xl bg-launcher-panelAlt backdrop-blur-[18px] border border-launcher-border p-5">
        {/* Header */}
        <div className="flex items-start justify-between w-full">
          <div className="flex flex-col gap-1">
            <h2 className="font-sans text-2xl font-bold text-white">
              已安装引擎
            </h2>
            <p className="font-sans text-[13px] text-launcher-textDark">
              管理 Unreal Engine 版本、插件和本地缓存。
            </p>
          </div>
          <div className="flex gap-2.5 shrink-0">
            <button className="flex items-center justify-center gap-2 h-11 rounded-[22px] bg-[#FFFFFF10] border border-launcher-borderLight px-[18px] hover:bg-launcher-hover transition-colors">
              <FolderOpen size={18} className="text-[#D9DEE7]" />
              <span className="font-sans text-sm font-semibold text-[#D9DEE7]">
                手动选择
              </span>
            </button>
            <button className="flex items-center justify-center gap-2 h-11 rounded-[22px] bg-white px-[18px] hover:bg-gray-100 transition-colors">
              <RefreshCw size={18} className="text-launcher-bg" />
              <span className="font-sans text-sm font-bold text-launcher-bg">
                刷新
              </span>
            </button>
          </div>
        </div>

        {/* Summary Chips */}
        <div className="flex items-center gap-2.5 w-full">
          <div className="flex items-center h-9 rounded-[18px] bg-[#FFFFFF10] border border-launcher-borderLight px-3.5">
            <span className="font-mono text-[12px] text-[#D9DEE7]">
              3 个版本
            </span>
          </div>
          <div className="flex items-center h-9 rounded-[18px] bg-[#FFFFFF10] border border-launcher-borderLight px-3.5">
            <span className="font-mono text-[12px] text-[#D9DEE7]">
              UE 5.4.4 默认
            </span>
          </div>
          <div className="flex items-center h-9 rounded-[18px] bg-[#FFFFFF10] border border-launcher-borderLight px-3.5">
            <span className="font-mono text-[12px] text-[#D9DEE7]">
              143.8 GB
            </span>
          </div>
        </div>

        {/* Engine List */}
        <div className="flex flex-col gap-3 flex-1 min-h-0 overflow-y-auto">
          {engines.map((eng) => (
            <div
              key={eng.version}
              className="flex items-center gap-4 w-full rounded-[18px] bg-[#FFFFFF0B] border border-[#FFFFFF10] p-4"
            >
              {/* Color Rail */}
              <div className={`w-1 h-[72px] rounded-full ${eng.railColor} shrink-0`} />

              {/* Info */}
              <div className="flex flex-col gap-1 flex-1 min-w-0">
                <div className="flex items-center gap-2.5">
                  <span className="font-mono text-[15px] font-bold text-white">
                    {eng.version}
                  </span>
                  <span
                    className={`flex items-center h-[22px] rounded-[11px] border px-2.5 font-sans text-[11px] font-bold ${eng.badgeColor}`}
                  >
                    {eng.badge}
                  </span>
                </div>
                <div className="flex items-center gap-3">
                  <span className="font-mono text-[11px] text-launcher-textDark">
                    {eng.path}
                  </span>
                  <span className="font-mono text-[11px] text-launcher-textDark">
                    {eng.size}
                  </span>
                  <span className="font-mono text-[11px] text-launcher-textDark">
                    {eng.plugins} 插件
                  </span>
                </div>
              </div>

              {/* Actions */}
              <div className="flex items-center gap-2 shrink-0">
                <button className="flex items-center justify-center w-9 h-9 rounded-[18px] bg-white hover:bg-gray-100 transition-colors">
                  <Play size={16} className="text-launcher-bg" />
                </button>
                <button className="flex items-center justify-center w-9 h-9 rounded-[18px] bg-[#FFFFFF10] border border-launcher-borderLight hover:bg-launcher-hover transition-colors">
                  <Trash2 size={16} className="text-launcher-textSecondary" />
                </button>
                <button className="flex items-center justify-center w-9 h-9 rounded-[18px] bg-[#FFFFFF10] border border-launcher-borderLight hover:bg-launcher-hover transition-colors">
                  <MoreHorizontal size={16} className="text-launcher-textSecondary" />
                </button>
                <button className="flex items-center justify-center w-9 h-9 rounded-[18px] bg-[#FFFFFF10] border border-launcher-borderLight hover:bg-launcher-hover transition-colors">
                  <Plug size={16} className="text-launcher-textSecondary" />
                </button>
              </div>
            </div>
          ))}
        </div>

        {/* Footer */}
        <div className="flex items-center justify-between w-full h-[42px] shrink-0">
          <span className="font-sans text-xs text-launcher-textDark">
            本地缓存 143.8 GB · 36 个插件
          </span>
          <button className="flex items-center gap-2 h-[34px] rounded-[17px] bg-[#FFFFFF0D] border border-[#FFFFFF20] px-3 hover:bg-launcher-hover transition-colors">
            <Plus size={16} className="text-[#D9DEE7]" />
            <span className="font-sans text-xs font-bold text-[#D9DEE7]">
              安装新版本
            </span>
          </button>
        </div>
      </div>

      {/* Right Panel: Current Engine */}
      <div className="flex flex-col gap-4 w-[340px] shrink-0 rounded-3xl bg-launcher-panelAlt backdrop-blur-[18px] border border-launcher-border p-5">
        {/* Preview */}
        <div className="relative w-full h-[170px] rounded-[20px] overflow-hidden bg-gradient-to-br from-[#1A2A3A] to-[#0D1117] border border-[#FFFFFF22]">
          <div className="absolute top-5 left-[96px] w-[144px] h-[144px] rounded-full bg-[#4A9FD833] blur-[26px]" />
          <div className="absolute inset-0 flex items-center justify-center">
            <span className="font-mono text-[48px] font-bold text-[#4A9FD8CC]">
              UE
            </span>
          </div>
        </div>

        {/* Title */}
        <div className="flex flex-col gap-1">
          <span className="font-sans text-[21px] font-bold text-white">
            Unreal Engine
          </span>
          <span className="font-mono text-[11px] text-launcher-textDark">
            Epic Games · 游戏引擎
          </span>
        </div>

        {/* Version Selector */}
        <div className="flex items-center justify-between w-full h-11 rounded-[18px] bg-[#FFFFFF10] border border-launcher-borderLight px-3.5">
          <span className="font-mono text-[13px] text-white">
            UE 5.4.4
          </span>
          <ChevronDown size={18} className="text-launcher-textSecondary" />
        </div>

        {/* Launch Button */}
        <button className="flex items-center justify-center gap-2 w-full h-[42px] rounded-[21px] bg-white hover:bg-gray-100 transition-colors">
          <Play size={17} className="text-launcher-bg" />
          <span className="font-sans text-[13px] font-bold text-launcher-bg">
            启动编辑器
          </span>
        </button>

        {/* Status */}
        <div className="flex flex-col gap-3 rounded-[18px] bg-[#FFFFFF0B] p-3.5">
          <div className="flex items-center justify-between w-full">
            <span className="font-sans text-xs text-launcher-textDark">
              状态
            </span>
            <div className="flex items-center gap-2">
              <div className="w-2 h-2 rounded-full bg-launcher-success" />
              <span className="font-mono text-xs font-bold text-[#B6FFCE]">
                已就绪
              </span>
            </div>
          </div>
          <div className="flex items-center justify-between w-full">
            <span className="font-sans text-xs text-launcher-textDark">
              安装路径
            </span>
            <span className="font-mono text-xs text-[#D9DEE7]">
              D:/Epic/UE_5.4
            </span>
          </div>
          <div className="flex items-center justify-between w-full">
            <span className="font-sans text-xs text-launcher-textDark">
              占用空间
            </span>
            <span className="font-mono text-xs text-[#D9DEE7]">
              52.6 GB
            </span>
          </div>
        </div>

        {/* Options Trigger */}
        <button className="flex items-center justify-between w-full h-11 rounded-[18px] bg-[#FFFFFF10] border border-launcher-borderLight px-3.5 hover:bg-launcher-hover transition-colors">
          <span className="font-sans text-[13px] text-[#D9DEE7]">
            引擎选项
          </span>
          <ChevronDown size={18} className="text-launcher-textSecondary" />
        </button>

        {/* Plugins */}
        <div className="flex flex-col gap-2.5">
          <div className="flex items-center justify-between w-full">
            <span className="font-sans text-sm font-bold text-white">
              插件
            </span>
            <span className="font-mono text-[11px] text-launcher-textDark">
              12 已安装
            </span>
          </div>
          <div className="flex flex-col gap-1">
            {plugins.map((p) => (
              <div
                key={p.name}
                className="flex items-center justify-between h-9 rounded-[14px] bg-[#FFFFFF0B] px-3"
              >
                <span className="font-sans text-xs text-[#D9DEE7]">
                  {p.name}
                </span>
                <span className={`font-mono text-[11px] font-bold ${p.stateColor}`}>
                  {p.state}
                </span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
