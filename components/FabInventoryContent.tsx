"use client";

import {
  RefreshCw,
  Search,
  SlidersHorizontal,
  Trees,
  Plug,
  Box,
  Palette,
  ArrowDownToLine,
  Ellipsis,
  HardDriveDownload,
} from "lucide-react";

const assets = [
  {
    title: "Nordic Megascans Pack",
    gradient: "from-[#31566B] to-[#111820]",
    icon: Trees,
    iconColor: "text-[#FFFFFFCC]",
    meta: "环境 · 3.2 GB",
    state: "已下载",
    stateColor: "text-[#B6FFCE]",
    selected: true,
  },
  {
    title: "MetaHuman Animator",
    gradient: "from-[#5B3D20] to-[#121212]",
    icon: Plug,
    iconColor: "text-[#FFB15CCC]",
    meta: "插件 · 1.1 GB",
    state: "可更新",
    stateColor: "text-launcher-warning",
    selected: false,
  },
  {
    title: "City Props Kit",
    gradient: null,
    icon: Box,
    iconColor: "text-launcher-textSecondary",
    meta: "道具 · 未下载",
    state: null,
    stateColor: "",
    selected: false,
  },
  {
    title: "Advanced Glass Materials",
    gradient: null,
    icon: Palette,
    iconColor: "text-launcher-textSecondary",
    meta: "材质 · 已安装",
    state: null,
    stateColor: "",
    selected: false,
  },
];

const metaRows = [
  { label: "本地状态", value: "已下载", valueColor: "text-[#B6FFCE]" },
  { label: "兼容引擎", value: "UE 5.2+", valueColor: "text-[#D9DEE7]" },
  { label: "缓存大小", value: "3.2 GB", valueColor: "text-[#D9DEE7]" },
];

const deps = [
  { name: "Nanite meshes", state: "Ready", stateColor: "text-[#B6FFCE]" },
  { name: "8K material set", state: "Cached", stateColor: "text-[#9ED8FF]" },
  {
    name: "Collision presets",
    state: "Missing",
    stateColor: "text-launcher-warning",
  },
];

export default function FabInventoryContent() {
  return (
    <div className="flex gap-[18px] flex-1 min-h-0">
      {/* Fab Asset Inventory */}
      <div className="flex flex-col gap-4 flex-1 min-w-0 rounded-3xl bg-launcher-panelAlt backdrop-blur-[18px] border border-launcher-border p-5">
        {/* Header */}
        <div className="flex items-start justify-between w-full">
          <div className="flex flex-col gap-1">
            <h2 className="font-sans text-2xl font-bold text-white">
              资产库存
            </h2>
            <p className="font-sans text-[13px] text-launcher-textDark">
              管理已拥有的 Fab 资产、插件、材质包和可导入工程内容。
            </p>
          </div>
          <button className="flex items-center justify-center gap-2 h-11 rounded-[22px] bg-white px-[18px] hover:bg-gray-100 transition-colors shrink-0">
            <RefreshCw size={18} className="text-launcher-bg" />
            <span className="font-sans text-sm font-bold text-launcher-bg">
              同步库存
            </span>
          </button>
        </div>

        {/* Toolbar */}
        <div className="flex items-center gap-3 h-12 w-full">
          <div className="flex items-center gap-2.5 flex-1 h-full rounded-3xl bg-[#FFFFFF10] border border-launcher-borderLight px-3.5">
            <Search size={18} className="text-launcher-textDark" />
            <span className="font-sans text-[13px] text-launcher-textDark">
              搜索资产、作者、标签
            </span>
          </div>
          <button className="flex items-center justify-center gap-2 h-full rounded-3xl bg-white px-3.5 border border-launcher-border hover:bg-gray-100 transition-colors">
            <SlidersHorizontal size={16} className="text-launcher-bg" />
            <span className="font-sans text-[13px] font-bold text-launcher-bg">
              筛选
            </span>
          </button>
        </div>

        {/* Categories */}
        <div className="flex gap-2.5 h-11 w-full">
          <button className="flex items-center justify-center h-full rounded-[22px] bg-white px-3.5">
            <span className="font-sans text-[13px] font-bold text-launcher-bg">
              全部 284
            </span>
          </button>
          <button className="flex items-center justify-center h-full rounded-[22px] bg-[#FFFFFF10] border border-launcher-borderLight px-3.5">
            <span className="font-sans text-[13px] font-semibold text-[#D9DEE7]">
              环境
            </span>
          </button>
          <button className="flex items-center justify-center h-full rounded-[22px] bg-[#FFFFFF10] border border-launcher-borderLight px-3.5">
            <span className="font-sans text-[13px] font-semibold text-[#D9DEE7]">
              插件
            </span>
          </button>
        </div>

        {/* Asset Grid */}
        <div className="flex flex-col gap-3 flex-1 min-h-0 overflow-y-auto">
          {/* Row 1 */}
          <div className="flex gap-3 flex-1 min-h-0">
            {assets.slice(0, 2).map((a) => {
              const Icon = a.icon;
              return (
                <div
                  key={a.title}
                  className={`flex flex-col gap-3 flex-1 rounded-[18px] p-3.5 ${
                    a.selected
                      ? "bg-[#FFFFFF16] border border-[#4A9FD866]"
                      : "bg-[#FFFFFF0B]"
                  }`}
                >
                  {/* Thumbnail */}
                  <div
                    className={`relative w-full h-[112px] rounded-[14px] overflow-hidden ${
                      a.gradient
                        ? `bg-gradient-to-br ${a.gradient}`
                        : "bg-[#FFFFFF10]"
                    }`}
                  >
                    {a.gradient ? (
                      <>
                        <div className="absolute top-3 left-[110px] w-[120px] h-[120px] rounded-full bg-[#4A9FD855] blur-[26px]" />
                        <div className="absolute inset-0 flex items-center justify-center">
                          <Icon size={48} className={a.iconColor} />
                        </div>
                      </>
                    ) : (
                      <div className="absolute inset-0 flex items-center justify-center">
                        <Icon size={46} className={a.iconColor} />
                      </div>
                    )}
                  </div>

                  {/* Title */}
                  <span className="font-sans text-[15px] font-bold text-white">
                    {a.title}
                  </span>

                  {/* Meta */}
                  <div className="flex items-center justify-between w-full">
                    <span className="font-mono text-[11px] text-launcher-textDark">
                      {a.meta}
                    </span>
                    {a.state && (
                      <span
                        className={`font-sans text-xs font-bold ${a.stateColor}`}
                      >
                        {a.state}
                      </span>
                    )}
                  </div>
                </div>
              );
            })}
          </div>

          {/* Row 2 */}
          <div className="flex gap-3 flex-1 min-h-0">
            {assets.slice(2, 4).map((a) => {
              const Icon = a.icon;
              return (
                <div
                  key={a.title}
                  className="flex flex-col gap-3 flex-1 rounded-[18px] bg-[#FFFFFF0B] p-3.5"
                >
                  {/* Thumbnail */}
                  <div className="flex items-center justify-center w-full h-[112px] rounded-[14px] bg-[#FFFFFF10]">
                    <Icon size={46} className={a.iconColor} />
                  </div>

                  {/* Title */}
                  <span className="font-sans text-[15px] font-bold text-white">
                    {a.title}
                  </span>

                  {/* Meta */}
                  <span className="font-mono text-[11px] text-launcher-textDark">
                    {a.meta}
                  </span>
                </div>
              );
            })}
          </div>
        </div>

        {/* Footer */}
        <div className="flex items-center justify-between w-full h-[42px] shrink-0">
          <span className="font-sans text-xs text-launcher-textDark">
            显示 4 / 284 个资产 · 本地缓存 42.6 GB
          </span>
          <button className="flex items-center gap-2 h-[34px] rounded-[17px] bg-[#FFFFFF0D] border border-[#FFFFFF20] px-3 hover:bg-launcher-hover transition-colors">
            <HardDriveDownload size={16} className="text-[#D9DEE7]" />
            <span className="font-sans text-xs font-bold text-[#D9DEE7]">
              管理缓存
            </span>
          </button>
        </div>
      </div>

      {/* Asset Detail Panel */}
      <div className="flex flex-col gap-4 w-[340px] shrink-0 rounded-3xl bg-launcher-panelAlt backdrop-blur-[18px] border border-launcher-border p-5">
        {/* Preview */}
        <div className="relative w-full h-[170px] rounded-[20px] overflow-hidden bg-gradient-to-br from-[#31566B] to-[#101820] border border-[#FFFFFF22]">
          <div className="absolute top-5 left-[96px] w-[144px] h-[144px] rounded-full bg-[#4A9FD855] blur-[26px]" />
          <div className="absolute inset-0 flex items-center justify-center">
            <Trees size={64} className="text-[#FFFFFFCC]" />
          </div>
        </div>

        {/* Title */}
        <div className="flex flex-col gap-1">
          <span className="font-sans text-[21px] font-bold text-white">
            Nordic Megascans Pack
          </span>
          <span className="font-mono text-[11px] text-launcher-textDark">
            Quixel · Environment Collection
          </span>
        </div>

        {/* Actions */}
        <div className="flex gap-2.5 w-full">
          <button className="flex items-center justify-center gap-2 flex-1 h-[42px] rounded-[21px] bg-white hover:bg-gray-100 transition-colors">
            <ArrowDownToLine size={17} className="text-launcher-bg" />
            <span className="font-sans text-[13px] font-bold text-launcher-bg">
              导入工程
            </span>
          </button>
          <button className="flex items-center justify-center w-[42px] h-[42px] rounded-[21px] bg-launcher-hover border border-launcher-border hover:bg-launcher-hoverLight transition-colors">
            <Ellipsis size={18} className="text-white" />
          </button>
        </div>

        {/* Metadata */}
        <div className="flex flex-col gap-3 rounded-[18px] bg-[#FFFFFF0B] p-3.5">
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

        {/* Import Target */}
        <div className="flex flex-col gap-2.5 rounded-[18px] bg-[#103D3026] border border-[#4DE0A766] p-3.5">
          <div className="flex items-center gap-2">
            <div className="w-2 h-2 rounded-full bg-launcher-success" />
            <span className="font-sans text-[13px] font-bold text-[#B6FFCE]">
              可导入当前工程
            </span>
          </div>
          <p className="font-sans text-xs text-launcher-textSecondary leading-[1.4]">
            Aurora Facility 使用 UE 5.4.4，资产材质与 Nanite 设置可直接兼容。
          </p>
        </div>

        {/* Dependencies */}
        <span className="font-sans text-sm font-bold text-white">
          依赖与版本
        </span>
        <div className="flex flex-col gap-1">
          {deps.map((d) => (
            <div
              key={d.name}
              className="flex items-center justify-between h-7 rounded-[14px] bg-[#FFFFFF0B] px-2.5"
            >
              <span className="font-sans text-xs text-[#D9DEE7]">
                {d.name}
              </span>
              <span className={`font-mono text-[11px] ${d.stateColor}`}>
                {d.state}
              </span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
