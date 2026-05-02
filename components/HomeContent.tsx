import HeroProject from "@/components/HeroProject";
import FabInventory from "@/components/FabInventory";
import EngineManagement from "@/components/EngineManagement";
import AccountLogin from "@/components/AccountLogin";
import Downloads from "@/components/Downloads";
import SettingsPanel from "@/components/Settings";

export default function HomeContent() {
  return (
    <div className="flex gap-[18px] h-full">
      {/* Primary Panels */}
      <div className="flex flex-col gap-[18px] flex-1 min-w-0">
        <HeroProject />

        {/* Fab and Engine Row */}
        <div className="flex gap-[18px] flex-1 min-h-0">
          <FabInventory />
          <EngineManagement />
        </div>
      </div>

      {/* Right Rail */}
      <div className="flex flex-col gap-[18px] w-[340px] shrink-0">
        <AccountLogin />
        <Downloads />
        <SettingsPanel />
      </div>
    </div>
  );
}
