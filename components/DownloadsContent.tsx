"use client";

import {
  Download,
  CheckCheck,
  Settings,
  Trash2,
  History,
  Plus,
  Pause,
  RefreshCw,
  X,
} from "lucide-react";

type DownloadCategory = "active" | "completed" | "settings";

interface DownloadsContentProps {
  activeCategory?: DownloadCategory;
  onCategoryChange?: (cat: DownloadCategory) => void;
}

const activeTasks = [
  {
    name: "Fortnite 更新包",
    gradientFrom: "#2563EB",
    gradientTo: "#22D3EE",
    size: "38px",
    state: "下载中",
    stateColor: "bg-[#1D4ED840] text-[#93C5FD]",
    progress: 62,
    speed: "31.4 MB/s",
    eta: "28 分钟",
    selected: true,
  },
  {
    name: "UE 5.5 Preview",
    gradientFrom: "#111827",
    gradientTo: "#64748B",
    size: "34px",
    state: "已暂停",
    stateColor: "bg-[#FFFFFF10] text-[#94A3B8]",
    progress: 34,
    speed: "0 MB/s",
    eta: "47 分钟",
    selected: false,
  },
  {
    name: "ARK: Survival Ascended",
    gradientFrom: null,
    gradientTo: null,
    size: "34px",
    state: "失败",
    stateColor: "bg-[#FF5A5F2E] text-[#FFB8BE]",
    progress: 0,
    speed: null,
    eta: null,
    error: true,
    errorMsg: "网络中断 · 已保留 8.3 GB 缓存",
    selected: false,
  },
];

const completedTasks = [
  { name: "Alan Wake 2 - Night Springs", time: "今天 14:22" },
  { name: "Twinmotion 资源包", time: "昨天 22:09" },
];

export default function DownloadsContent({
  activeCategory = "active",
  onCategoryChange,
}: DownloadsContentProps) {
  return (
    <div className="flex gap-[18px] flex-1 min-h-0">
      {/* Left Sidebar */}
      <div className="flex flex-col gap-3 w-[200px] shrink-0 rounded-3xl bg-launcher-panelAlt backdrop-blur-[18px] border border-launcher-border p-3.5">
        {/* Sidebar Header */}
        <div className="flex flex-col gap-1 px-1 py-0.5">
          <span className="font-sans text-[13px] font-bold text-[#F8FAFC]">
            下载中心
          </span>
          <span className="font-sans text-[10px] font-medium text-[#94A3B8]">
            队列与记录
          </span>
        </div>

        {/* Categories */}
        <div className="flex flex-col gap-2">
          {/* Active Downloads */}
          <button
            onClick={() => onCategoryChange?.("active")}
            className={`flex items-center gap-2.5 rounded-2xl p-2.5 transition-colors ${
              activeCategory === "active"
                ? "bg-gradient-to-b from-[#2563EB59] to-[#0F172ACC] border border-[#60A5FA66]"
                : "bg-[#FFFFFF08] border border-transparent hover:bg-[#FFFFFF0D]"
            }`}
          >
            <div
              className={`flex items-center justify-center w-[31px] h-[31px] rounded-[10px] ${
                activeCategory === "active" ? "bg-[#60A5FA2E]" : "bg-[#FFFFFF10]"
              }`}
            >
              <Download
                size={16}
                className={
                  activeCategory === "active"
                    ? "text-[#BFDBFE]"
                    : "text-launcher-textSecondary"
                }
              />
            </div>
            <div className="flex flex-col gap-0.5 flex-1 min-w-0">
              <span
                className={`font-sans text-xs font-bold ${
                  activeCategory === "active"
                    ? "text-[#F8FAFC]"
                    : "text-[#E5E7EB]"
                }`}
              >
                正在下载
              </span>
              <span
                className={`font-sans text-[10px] font-medium ${
                  activeCategory === "active"
                    ? "text-[#BAE6FD]"
                    : "text-[#94A3B8]"
                }`}
              >
                3 项 · 12.4 MB/s
              </span>
            </div>
            {activeCategory === "active" && (
              <div className="w-[7px] h-[7px] rounded-full bg-[#38BDF8] shrink-0" />
            )}
          </button>

          {/* Completed */}
          <button
            onClick={() => onCategoryChange?.("completed")}
            className={`flex items-center gap-2.5 rounded-2xl p-2.5 transition-colors ${
              activeCategory === "completed"
                ? "bg-gradient-to-b from-[#2563EB59] to-[#0F172ACC] border border-[#60A5FA66]"
                : "bg-[#FFFFFF08] border border-transparent hover:bg-[#FFFFFF0D]"
            }`}
          >
            <div
              className={`flex items-center justify-center w-[30px] h-[30px] rounded-[10px] ${
                activeCategory === "completed"
                  ? "bg-[#22C55E1F]"
                  : "bg-[#FFFFFF10]"
              }`}
            >
              <CheckCheck
                size={16}
                className={
                  activeCategory === "completed"
                    ? "text-[#86EFAC]"
                    : "text-launcher-textSecondary"
                }
              />
            </div>
            <div className="flex flex-col gap-0.5 flex-1 min-w-0">
              <span
                className={`font-sans text-xs ${
                  activeCategory === "completed"
                    ? "font-bold text-[#F8FAFC]"
                    : "font-semibold text-[#E5E7EB]"
                }`}
              >
                下载完成
              </span>
              <span className="font-sans text-[10px] font-medium text-[#94A3B8]">
                24 项完成
              </span>
            </div>
            {activeCategory === "completed" && (
              <div className="w-[7px] h-[7px] rounded-full bg-[#38BDF8] shrink-0" />
            )}
          </button>

          {/* Settings */}
          <button
            onClick={() => onCategoryChange?.("settings")}
            className={`flex items-center gap-2.5 rounded-2xl p-2.5 transition-colors ${
              activeCategory === "settings"
                ? "bg-gradient-to-b from-[#2563EB59] to-[#0F172ACC] border border-[#60A5FA66]"
                : "bg-[#FFFFFF08] border border-transparent hover:bg-[#FFFFFF0D]"
            }`}
          >
            <div
              className={`flex items-center justify-center w-[30px] h-[30px] rounded-[10px] ${
                activeCategory === "settings"
                  ? "bg-[#A78BFA24]"
                  : "bg-[#FFFFFF10]"
              }`}
            >
              <Settings
                size={16}
                className={
                  activeCategory === "settings"
                    ? "text-[#C4B5FD]"
                    : "text-launcher-textSecondary"
                }
              />
            </div>
            <div className="flex flex-col gap-0.5 flex-1 min-w-0">
              <span
                className={`font-sans text-xs ${
                  activeCategory === "settings"
                    ? "font-bold text-[#F8FAFC]"
                    : "font-semibold text-[#E5E7EB]"
                }`}
              >
                下载设置
              </span>
              <span className="font-sans text-[10px] font-medium text-[#94A3B8]">
                限速 · 路径 · 通知
              </span>
            </div>
            {activeCategory === "settings" && (
              <div className="w-[7px] h-[7px] rounded-full bg-[#38BDF8] shrink-0" />
            )}
          </button>
        </div>

        {/* Spacer */}
        <div className="flex-1" />

        {/* Utilities */}
        <div className="flex flex-col gap-2 rounded-2xl bg-[#02061766] border border-[#FFFFFF14] p-2.5">
          <button className="flex items-center gap-2 h-[30px] hover:opacity-80 transition-opacity">
            <Trash2 size={14} className="text-[#CBD5E1]" />
            <span className="font-sans text-[11px] font-semibold text-[#CBD5E1]">
              清理缓存
            </span>
          </button>
          <button className="flex items-center gap-2 h-[30px] hover:opacity-80 transition-opacity">
            <History size={14} className="text-[#94A3B8]" />
            <span className="font-sans text-[11px] font-semibold text-[#94A3B8]">
              下载历史
            </span>
          </button>
        </div>
      </div>

      {/* Center: Active Downloads Workspace */}
      <div className="flex flex-col gap-3 flex-1 min-w-0 rounded-3xl bg-launcher-panelAlt backdrop-blur-[18px] border border-launcher-border p-4">
        {/* Queue Header */}
        <div className="flex items-center justify-between w-full h-[52px] shrink-0">
          <div className="flex flex-col gap-0.5">
            <span className="font-sans text-xl font-bold text-[#F6F8FF]">
              正在下载
            </span>
            <span className="font-sans text-[11px] text-[#9AA7BD]">
              3 个任务进行中 · 当前 38.6 MB/s
            </span>
          </div>
          <div className="flex items-center gap-2.5">
            <button className="flex items-center justify-center w-[38px] h-[38px] rounded-[10px] bg-[#FFFFFF10] border border-[#FFFFFF18] hover:bg-launcher-hover transition-colors">
              <Plus size={18} className="text-[#DDE7FF]" />
            </button>
            <button className="flex items-center gap-2 h-[38px] rounded-[18px] bg-[#FF840018] border border-[#FF840044] px-4 hover:bg-[#FF840028] transition-colors">
              <Pause size={14} className="text-[#FF8400]" />
              <span className="font-sans text-xs font-semibold text-[#FF8400]">
                全部暂停
              </span>
            </button>
          </div>
        </div>

        {/* Queue Summary */}
        <div className="flex gap-2.5 w-full h-[42px] shrink-0">
          {[
            { label: "总速度", value: "38.6 MB/s" },
            { label: "剩余", value: "84.2 GB" },
            { label: "预计完成", value: "02:18" },
          ].map((s) => (
            <div
              key={s.label}
              className="flex flex-col justify-center flex-1 rounded-[10px] bg-[#0A0F16B8] border border-[#FFFFFF12] px-2.5 py-1.5"
            >
              <span className="font-sans text-[11px] text-[#7D8AA1]">
                {s.label}
              </span>
              <span className="font-mono text-sm font-bold text-[#EAF1FF]">
                {s.value}
              </span>
            </div>
          ))}
        </div>

        {/* Task List */}
        <div className="flex flex-col gap-2.5 flex-1 min-h-0 overflow-y-auto">
          {/* Active Task - Fortnite */}
          <div className="flex flex-col gap-2.5 rounded-[14px] bg-[#151C27E6] border border-[#4B9CFF55] p-3.5">
            <div className="flex items-center gap-2.5 w-full">
              <div className="w-[38px] h-[38px] rounded-[9px] bg-gradient-to-br from-[#2563EB] to-[#22D3EE] shrink-0" />
              <div className="flex flex-col gap-0.5 flex-1 min-w-0">
                <span className="font-sans text-sm font-bold text-white">
                  Fortnite 更新包
                </span>
                <span className="font-sans text-[11px] text-[#9AA7BD]">
                  42.8 GB / 68.0 GB
                </span>
              </div>
              <span className="font-sans text-xs font-bold text-[#93C5FD] bg-[#1D4ED840] rounded-lg px-2 py-1">
                下载中
              </span>
            </div>
            {/* Progress */}
            <div className="w-full h-2 rounded-full bg-[#FFFFFF14] overflow-hidden">
              <div className="h-full w-[62%] rounded-full bg-gradient-to-r from-[#59A7FF] to-[#7CFFCB]" />
            </div>
            <div className="flex items-center justify-between w-full">
              <span className="font-sans text-xs text-[#AEBAD0]">
                31.4 MB/s · 剩余 28 分钟
              </span>
              <div className="flex gap-2">
                <button className="flex items-center justify-center w-7 h-7 rounded-lg bg-[#FFFFFF10] hover:bg-launcher-hover transition-colors">
                  <Pause size={14} className="text-white" />
                </button>
                <button className="flex items-center justify-center w-7 h-7 rounded-lg bg-[#FF5A5F22] hover:bg-[#FF5A5F33] transition-colors">
                  <X size={14} className="text-[#FFB8BE]" />
                </button>
              </div>
            </div>
          </div>

          {/* Paused Task - UE 5.5 */}
          <div className="flex flex-col gap-2 rounded-[14px] bg-[#111821CC] border border-[#FFFFFF14] p-3">
            <div className="flex items-center gap-2.5 w-full">
              <div className="w-[34px] h-[34px] rounded-[8px] bg-gradient-to-br from-[#111827] to-[#64748B] shrink-0" />
              <div className="flex flex-col gap-0.5 flex-1 min-w-0">
                <span className="font-sans text-sm font-bold text-white">
                  UE 5.5 Preview
                </span>
                <span className="font-sans text-[11px] text-[#9AA7BD]">
                  12.3 GB / 36.1 GB
                </span>
              </div>
              <div className="flex gap-2">
                <button className="flex items-center justify-center w-7 h-7 rounded-lg bg-[#FFFFFF10] hover:bg-launcher-hover transition-colors">
                  <RefreshCw size={14} className="text-white" />
                </button>
                <button className="flex items-center justify-center w-7 h-7 rounded-lg bg-[#FF5A5F22] hover:bg-[#FF5A5F33] transition-colors">
                  <X size={14} className="text-[#FFB8BE]" />
                </button>
              </div>
            </div>
            <div className="w-full h-[7px] rounded-full bg-[#FFFFFF12] overflow-hidden">
              <div className="h-full w-[34%] rounded-full bg-[#7D8AA1]" />
            </div>
            <div className="flex items-center justify-between w-full">
              <span className="font-sans text-[11px] text-[#A2AEC2]">
                已暂停 · 0 MB/s
              </span>
              <span className="font-sans text-[11px] text-[#A2AEC2]">
                预计 47 分钟
              </span>
            </div>
          </div>

          {/* Failed Task - ARK */}
          <div className="flex items-center gap-3 rounded-[14px] bg-[#211318CC] border border-[#FF6B6B44] p-3">
            <div className="flex items-center justify-center w-[34px] h-[34px] rounded-[8px] bg-[#FF5A5F22] shrink-0">
              <span className="font-mono text-lg font-bold text-[#FFB8BE]">
                !
              </span>
            </div>
            <div className="flex flex-col gap-1 flex-1 min-w-0">
              <span className="font-sans text-sm font-bold text-[#FFF2F3]">
                ARK: Survival Ascended
              </span>
              <span className="font-sans text-[11px] text-[#C89AA1]">
                网络中断 · 已保留 8.3 GB 缓存
              </span>
            </div>
            <div className="flex gap-2 shrink-0">
              <button className="flex items-center justify-center h-7 rounded-lg bg-[#FF5A5F2E] px-2.5 hover:bg-[#FF5A5F3E] transition-colors">
                <RefreshCw size={13} className="text-[#FFB8BE]" />
              </button>
              <button className="flex items-center justify-center h-7 rounded-lg bg-[#FFFFFF10] px-2.5 hover:bg-launcher-hover transition-colors">
                <Trash2 size={13} className="text-[#CBD5E1]" />
              </button>
            </div>
          </div>

          {/* Completed Section */}
          <div className="flex flex-col gap-2 rounded-[14px] bg-[#0D151FCC] border border-[#7CFFCB26] p-3">
            <div className="flex items-center justify-between w-full">
              <span className="font-sans text-[13px] font-bold text-[#DCE6F7]">
                下载完成
              </span>
              <button className="font-sans text-[11px] text-[#7FB5FF] hover:underline">
                查看全部
              </button>
            </div>
            {completedTasks.map((t) => (
              <div
                key={t.name}
                className="flex items-center justify-between w-full"
              >
                <span className="font-sans text-xs text-[#AEBAD0]">
                  {t.name}
                </span>
                <span className="font-mono text-[11px] text-[#7D8AA1]">
                  {t.time}
                </span>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Right: Task Detail Pane */}
      <div className="flex flex-col gap-3 w-[330px] shrink-0 rounded-3xl bg-launcher-panelAlt backdrop-blur-[18px] border border-launcher-border p-4">
        {/* Title */}
        <span className="font-sans text-xl font-bold text-white">
          当前任务详情
        </span>
        <span className="font-sans text-xs text-[#8D96A4] leading-[1.35]">
          跟随正在下载列表中选中的任务变化。
        </span>

        {/* Selected Download Detail */}
        <div className="flex flex-col gap-2.5 rounded-[18px] bg-[#151C27E6] border border-[#4B9CFF55] p-3">
          {/* Top */}
          <div className="flex items-center gap-3 w-full">
            <div className="w-[46px] h-[46px] rounded-xl bg-gradient-to-br from-[#2563EB] to-[#22D3EE] shrink-0" />
            <div className="flex flex-col gap-1 flex-1 min-w-0">
              <span className="font-sans text-[15px] font-bold text-[#F6F8FF]">
                Fortnite 更新包
              </span>
              <span className="font-sans text-[11px] text-[#9AA7BD]">
                正在下载 · 42.8 GB / 68.0 GB
              </span>
            </div>
          </div>

          {/* Progress */}
          <div className="flex items-center justify-between w-full">
            <span className="font-sans text-xs text-[#AEBAD0]">进度</span>
            <span className="font-mono text-xs font-bold text-[#BBD8FF]">
              62%
            </span>
          </div>
          <div className="w-full h-[9px] rounded-full bg-[#FFFFFF14] overflow-hidden">
            <div className="h-full w-[62%] rounded-full bg-gradient-to-r from-[#59A7FF] to-[#7CFFCB]" />
          </div>

          {/* Metrics */}
          <div className="flex flex-col gap-2">
            <div className="flex items-center justify-between w-full">
              <span className="font-sans text-[11px] text-[#8D96A4]">
                当前速度
              </span>
              <span className="font-mono text-xs font-bold text-[#EAF1FF]">
                31.4 MB/s
              </span>
            </div>
            <div className="flex items-center justify-between w-full">
              <span className="font-sans text-[11px] text-[#8D96A4]">
                预计剩余
              </span>
              <span className="font-mono text-xs font-bold text-[#EAF1FF]">
                28 分钟
              </span>
            </div>
          </div>

          {/* Actions */}
          <div className="flex gap-2 w-full h-8">
            <button className="flex items-center justify-center flex-1 rounded-[10px] bg-[#FFFFFF12] hover:bg-launcher-hover transition-colors">
              <span className="font-sans text-xs font-bold text-[#DCE6F7]">
                暂停
              </span>
            </button>
            <button className="flex items-center justify-center flex-1 rounded-[10px] bg-[#FF5A5F22] border border-[#FF5A5F44] hover:bg-[#FF5A5F33] transition-colors">
              <span className="font-sans text-xs font-bold text-[#FFD5D9]">
                取消
              </span>
            </button>
          </div>
        </div>

        {/* Queue Status Card */}
        <div className="flex flex-col gap-2 rounded-[18px] bg-[#FFFFFF0B] border border-[#FFFFFF18] p-3">
          <span className="font-sans text-sm font-bold text-white">
            队列概览
          </span>
          {[
            { label: "正在下载", value: "3", color: "text-[#BBD8FF]" },
            { label: "下载完成", value: "2", color: "text-[#7CFFCB]" },
            { label: "需要处理", value: "1", color: "text-[#FFB8BE]" },
          ].map((row) => (
            <div
              key={row.label}
              className="flex items-center justify-between w-full"
            >
              <span className="font-sans text-xs text-[#9AA7BD]">
                {row.label}
              </span>
              <span className={`font-mono text-[13px] font-bold ${row.color}`}>
                {row.value}
              </span>
            </div>
          ))}
        </div>

        {/* Speed Visualization */}
        <div className="flex flex-col gap-2.5 rounded-[18px] bg-[#071522] border border-[#1E3A5F] p-3">
          <div className="flex items-center justify-between w-full">
            <span className="font-sans text-[13px] font-bold text-white">
              网速可视化
            </span>
            <span className="font-mono text-xs font-bold text-[#7CFFCB]">
              31.4 MB/s
            </span>
          </div>
          {/* Bar Chart */}
          <div className="relative w-full h-[84px] rounded-xl bg-[#081A2A] overflow-hidden">
            <div className="absolute inset-0 flex items-end justify-around px-2 pb-1">
              {[42, 46, 44, 38, 28, 34, 36, 33, 40, 39, 32, 22, 18, 24].map(
                (h, i) => (
                  <div
                    key={i}
                    className="w-2 rounded-sm bg-[#2563EB] opacity-40"
                    style={{ height: `${h}px` }}
                  />
                )
              )}
            </div>
            {/* Speed line overlay */}
            <svg
              className="absolute inset-0 w-full h-full"
              viewBox="0 0 270 78"
              preserveAspectRatio="none"
            >
              <polyline
                points="8,42 22,36 36,38 50,42 78,48 92,44 106,40 120,42 148,36 162,37 176,44 204,54 218,58 232,52"
                fill="none"
                stroke="#8BB64A"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              />
            </svg>
          </div>
        </div>
      </div>
    </div>
  );
}
