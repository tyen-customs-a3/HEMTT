class CfgWeapons {
    class ItemCore;
    class ACE_ItemCore;
    class CBA_MiscItem_ItemInfo;
    class InventoryFirstAidKitItem_Base_F;
    class MedikitItem;

    class FirstAidKit: ItemCore {
        type = 0;
        ACE_isMedicalItem = 1;
        class ItemInfo: InventoryFirstAidKitItem_Base_F {
            mass = 4;
        };
    };
    class Medikit: ItemCore {
        type = 0;
        ACE_isMedicalItem = 1;
        class ItemInfo: MedikitItem {
            mass = 60;
        };
    };

    class ACE_fieldDressing: ACE_ItemCore {
        scope = 2;
        author = ECSTRING(common,ACETeam);
        model = QPATHTOF(data\bandage.p3d);
        picture = QPATHTOF(ui\fieldDressing_ca.paa);
        displayName = CSTRING(Bandage_Basic_Display);
        descriptionShort = CSTRING(Bandage_Basic_Desc_Short);
        descriptionUse = CSTRING(Bandage_Basic_Desc_Use);
        ACE_isMedicalItem = 1;
        class ItemInfo: CBA_MiscItem_ItemInfo {
            mass = 0.6;
        };
    };
    class ACE_packingBandage: ACE_ItemCore {
        scope = 2;
        author = ECSTRING(common,ACETeam);
        displayName = CSTRING(Packing_Bandage_Display);
        picture = QPATHTOF(ui\packingBandage_ca.paa);
        model = QPATHTOF(data\packingbandage.p3d);
        descriptionShort = CSTRING(Packing_Bandage_Desc_Short);
        descriptionUse = CSTRING(Packing_Bandage_Desc_Use);
        ACE_isMedicalItem = 1;
        class ItemInfo: CBA_MiscItem_ItemInfo {
            mass = 0.6;
        };
    };
};
