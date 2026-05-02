"use client";

import {
  Plus,
  Search,
  MountainSnow,
  Building2,
  Trees,
  Archive,
  Play,
  FolderOpen,
  Copy,
  RefreshCw,
  ArchiveRestore,
} from "lucide-react";

const projects = [
  {
    icon: MountainSnow,
    iconBg: "bg-launcher-accentDim",
    iconColor: "text-[#9ED8FF]",
    name: "Aurora Facility",
    path: "D:/Unreal/AuroraFacility/AuroraFacility.uproject",
    engine: "UE 5.4.4",
    lastOpen: "今天 14:20",
    selected: true,
    disabled: false,
  },
  {
    icon: Building2,
    iconBg: "bg-[#FFFFFF10]",
    iconColor: "text-launcher-textSecondary",
    name: "Metropolis Tools",
    path: "E:/Client/Metropolis/MetropolisTools.uproject",
    engine: "UE 5.3.2",
    lastOpen: "昨天",
    selected: false,
    disabled: false,
  },
  {
    icon: Trees,
    iconBg: "bg-[#FFFFFF10]",
    iconColor: "text-launcher-textSecondary",
    name: "Forest Lighting Lab",
    path: "D:/Research/LightingLab/ForestLighting.uproject",
    engine: "UE 5.4.4",
    lastOpen: "4 天前",
    selected: false,
    disabled: false,
  },
  {
    icon: Archive,
    iconBg: "bg-[#FFFFFF0B]",
    iconColor: "text-launcher-textMuted",
    name: "Legacy Cinematic Test",
    path: "F:/Archive/CinematicTest/CinematicTest.uproject",
    engine: "UE 4.27",
    lastOpen: "需迁移",
    lastOpenColor: "text-launcher-warning",
    selected: false,
    disabled: true,
  },
];

const metaRows = [
  { label: "引擎版本", value: "UE 5.4.4", valueColor: "text-[#D9DEE7]" },
  { label: "工程大小", value: "18.6 GB", valueColor: "text-[#D9DEE7]" },
  { label: "Fab 依赖", value: "7 个资产", valueColor: "text-[#9ED8FF]" },
];

const ops = [
  { icon: Copy, label: "复制工程路径" },
  { icon: RefreshCw, label: "迁移到新版引擎" },
  { icon: ArchiveRestore, label: "归档工程" },
];

export default function MyProjectsContent() {
  return (
    <div className="flex gap-[18px] flex-1 min-h-0">
      {/* Project Library List */}
      <div className="flex flex-col gap-4 flex-1 min-w-0 rounded-3xl bg-launcher-panelAlt backdrop-blur-[18px] border border-launcher-border p-5">
        {/* Header */}
        <div className="flex items-start justify-between w-full">
          <div className="flex flex-col gap-1">
            <h2 className="font-sans text-2xl font-bold text-white">
              项目工程库
            </h2>
            <p className="font-sans text-[13px] text-launcher-textDark">
              只展示并管理 Unreal Engine 项目工程，不混入商城与资产库存。
            </p>
          </div>
          <button className="flex items-center justify-center gap-2 h-11 rounded-[22px] bg-white px-[18px] hover:bg-gray-100 transition-colors shrink-0">
            <Plus size={18} className="text-launcher-bg" />
            <span className="font-sans text-sm font-bold text-launcher-bg">
              新建工程
            </span>
          </button>
        </div>

        {/* Toolbar */}
        <div className="flex items-center gap-3 h-12 w-full">
          <div className="flex items-center gap-2.5 flex-1 h-full rounded-3xl bg-[#FFFFFF10] border border-launcher-borderLight px-3.5">
            <Search size={18} className="text-launcher-textDark" />
            <span className="font-sans text-[13px] text-launcher-textDark">
              搜索工程名称、路径、版本
            </span>
          </div>
          <button className="flex items-center justify-center h-full rounded-3xl bg-white px-3.5">
            <span className="font-sans text-[13px] font-bold text-launcher-bg">
              全部
            </span>
          </button>
          <button className="flex items-center justify-center h-full rounded-3xl bg-[#FFFFFF10] border border-launcher-borderLight px-3.5">
            <span className="font-sans text-[13px] font-semibold text-[#D9DEE7]">
              最近
            </span>
          </button>
        </div>

        {/* Metrics */}
        <div className="flex gap-3 h-[76px] w-full">
          <div className="flex flex-col items-center justify-center gap-1 flex-1 rounded-[18px] bg-[#FFFFFF0B] px-3.5 py-2.5">
            <span className="font-mono text-[22px] font-bold text-white">
              12
            </span>
            <span className="font-sans text-xs text-launcher-textDark">
              本地工程
            </span>
          </div>
          <div className="flex flex-col items-center justify-center gap-1 flex-1 rounded-[18px] bg-[#FFFFFF0B] px-3.5 py-2.5">
            <span className="font-mono text-[22px] font-bold text-[#9ED8FF]">
              5.4
            </span>
            <span className="font-sans text-xs text-launcher-textDark">
              主力引擎版本
            </span>
          </div>
        </div>

        {/* Column Headers */}
        <div className="flex items-center gap-3 h-[34px] w-full px-3.5">
          <span className="font-mono text-[11px] font-semibold text-launcher-textMuted flex-1">
            工程
          </span>
          <span className="font-mono text-[11px] font-semibold text-launcher-textMuted w-[86px]">
            引擎
          </span>
          <span className="font-mono text-[11px] font-semibold text-launcher-textMuted w-[110px]">
            最近打开
          </span>
        </div>

        {/* Project Rows */}
        <div className="flex flex-col gap-2 flex-1 min-h-0 overflow-y-auto">
          {projects.map((p) => {
            const Icon = p.icon;
            return (
              <div
                key={p.name}
                className={`flex items-center gap-3 rounded-[18px] p-3 shrink-0 ${
                  p.selected
                    ? "h-[76px] bg-[#FFFFFF16] border border-[#4A9FD866]"
                    : p.disabled
                      ? "h-[68px] bg-[#FFFFFF07]"
                      : "h-[68px] bg-[#FFFFFF0B]"
                }`}
              >
                <div
                  className={`flex items-center justify-center w-10 h-10 rounded-[14px] ${p.iconBg} shrink-0`}
                >
                  <Icon size={p.selected ? 24 : 22} className={p.iconColor} />
                </div>
                <div className="flex flex-col gap-1 flex-1 min-w-0">
                  <span
                    className={`font-sans text-[15px] font-bold ${
                      p.disabled
                        ? "text-launcher-textSecondary"
                        : "text-white"
                    }`}
                  >
                    {p.name}
                  </span>
                  <span
                    className={`font-mono text-[11px] truncate ${
                      p.disabled
                        ? "text-launcher-textMuted"
                        : "text-launcher-textDark"
                    }`}
                  >
                    {p.path}
                  </span>
                </div>
                <span className="font-mono text-xs text-[#D9DEE7] w-[86px] shrink-0">
                  {p.engine}
                </span>
                <span
                  className={`font-sans text-xs w-[110px] shrink-0 ${
                    p.lastOpenColor || "text-launcher-textDark"
                  }`}
                >
                  {p.lastOpen}
                </span>
                {p.selected && (
                  <button className="flex items-center justify-center w-[38px] h-[38px] rounded-[19px] bg-white shrink-0 hover:bg-gray-100 transition-colors">
                    <Play size={17} className="text-launcher-bg" />
                  </button>
                )}
              </div>
            );
          })}
        </div>

        {/* Footer */}
        <div className="flex items-center justify-between w-full h-[42px] shrink-0">
          <span className="font-sans text-xs text-launcher-textDark">
            显示 4 / 12 个工程 · 按最近打开排序
          </span>
          <button className="flex items-center gap-2 h-[34px] rounded-[17px] bg-[#FFFFFF0D] border border-[#FFFFFF20] px-3 hover:bg-launcher-hover transition-colors">
            <FolderOpen size={16} className="text-[#D9DEE7]" />
            <span className="font-sans text-xs font-bold text-[#D9DEE7]">
              扫描本地工程
            </span>
          </button>
        </div>
      </div>

      {/* Selected Project Detail */}
      <div className="flex flex-col gap-2 w-[340px] shrink-0 rounded-3xl bg-launcher-panelAlt backdrop-blur-[18px] border border-launcher-border py-3 px-3.5">
        {/* Preview */}
        <div className="relative w-full h-[120px] rounded-[20px] overflow-hidden bg-gradient-to-br from-[#244A60] to-[#0C1016] border border-[#FFFFFF22]">
          <div className="absolute top-5 left-[96px] w-[144px] h-[144px] rounded-full bg-[#4A9FD84D] blur-[32px]" />
          <div className="absolute inset-0 flex items-center justify-center">
            <MountainSnow size={64} className="text-[#FFFFFFCC]" />
          </div>
        </div>

        {/* Detail Title */}
        <div className="flex flex-col gap-1">
          <span className="font-sans text-[22px] font-bold text-white">
            Aurora Facility
          </span>
          <span className="font-mono text-[11px] text-launcher-textDark truncate">
            D:/Unreal/AuroraFacility
          </span>
        </div>

        {/* Action Buttons */}
        <div className="flex gap-2 w-full">
          <button className="flex items-center justify-center gap-2 flex-1 h-[42px] rounded-[21px] bg-white hover:bg-gray-100 transition-colors">
            <Play size={17} className="text-launcher-bg" />
            <span className="font-sans text-[13px] font-bold text-launcher-bg">
              打开
            </span>
          </button>
          <button className="flex items-center justify-center w-[42px] h-[42px] rounded-[21px] bg-launcher-hover border border-launcher-border hover:bg-launcher-hoverLight transition-colors">
            <FolderOpen size={18} className="text-white" />
          </button>
        </div>

        {/* Metadata */}
        <div className="flex flex-col gap-2 rounded-[18px] bg-[#FFFFFF0B] p-2.5">
          {metaRows.map((m) => (
            <div
              key={m.label}
              className="flex items-center justify-between w-full"
            >
              <span className="font-sans text-xs text-launcher-textDark">
                {m.label}
              </span>
              <span className={`font-mono text-xs font-bold ${m.valueColor}`}>
                {m.value}
              </span>
            </div>
          ))}
        </div>

        {/* Health */}
        <div className="flex flex-col gap-1 rounded-[18px] bg-[#103D3026] border border-[#4DE0A766] p-2">
          <div className="flex items-center gap-2">
            <div className="w-2 h-2 rounded-full bg-launcher-success" />
            <span className="font-sans text-[13px] font-bold text-[#B6FFCE]">
              工程状态正常
            </span>
          </div>
          <p className="font-sans text-xs text-launcher-textSecondary leading-[1.4]">
            配置文件、插件清单与引擎版本匹配，当前可以直接打开。
          </p>
        </div>

        {/* Operations */}
        <span className="font-sans text-sm font-bold text-white">
          工程操作
        </span>
        <div className="flex flex-col gap-0.5">
          {ops.map((op) => {
            const Icon = op.icon;
            return (
              <button
                key={op.label}
                className="flex items-center gap-2.5 h-7 rounded-2xl bg-[#FFFFFF0B] px-2.5 hover:bg-[#FFFFFF18] transition-colors w-full"
              >
                <Icon size={15} className="text-launcher-textSecondary" />
                <span className="font-sans text-[13px] text-[#D9DEE7]">
                  {op.label}
                </span>
              </button>
            );
          })}
        </div>
      </div>
    </div>
  );
}
