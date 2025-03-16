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
        author = "ACE-Team";
        model = "data\bandage.p3d";
        picture = "ui\fieldDressing_ca.paa";
        displayName = "Field Dressing";
        descriptionShort = "Basic bandage for wounds";
        descriptionUse = "Used for basic treatment";
        ACE_isMedicalItem = 1;
        class ItemInfo: CBA_MiscItem_ItemInfo {
            mass = 0.6;
        };
    };
    class ACE_packingBandage: ACE_ItemCore {
        scope = 2;
        author = "ACE-Team";
        displayName = "Packing Bandage";
        picture = "ui\packingBandage_ca.paa";
        model = "data\packingbandage.p3d";
        descriptionShort = "Bandage for deep wounds";
        descriptionUse = "Pack deep wounds";
        ACE_isMedicalItem = 1;
        class ItemInfo: CBA_MiscItem_ItemInfo {
            mass = 0.6;
        };
    };
};
