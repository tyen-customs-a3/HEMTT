/**
	Adds a curated personal arsenal that can be accessed via ACE Self Interact by the players.
	
	Parameters:
	0: unit <OBJECT>
	
	UNIT MUST BE LOCAL
	
	Example:
	[player] call pca_fnc_curatedArsenal;
	
	Returns: 
	NONE
*/

params ["_unit"];

if !(hasInterface) exitWith {};

private _unitRole = _unit getVariable ["tmf_assignGear_role", nil];

arsenal = "building" createVehicleLocal [0,0,0];
_unit setVariable ["startPos", getPosASL _unit];

//********************************************************************************//
// Unit Equipment
//********************************************************************************//
//Common
private _itemGear = 
[
	"pca_vest_invisible_plate",
	"pca_backpack_invisible_large"
];

//Cosmetic 
private _itemCosmetic = 
[
	"pca_headband_blk",
	"pca_headband_tan",
	"pca_m88_fieldcap_gray",
	"pca_ushanka",
	"rhs_6m2",
	"rhs_6m2_1",
	
	"CUP_H_RUS_Balaclava_Ratnik_Headphones",
	"CUP_H_RUS_Balaclava_Ratnik_Headphones_v2",
	"CUP_H_RUS_Balaclava_Ratnik_Headphones_winter",
	"CUP_H_RUS_Balaclava_Ratnik_Headphones_winter_v2",
	"H_Bandanna_gry",
	"H_Bandanna_cbr",
	
	"CUP_H_FR_BandanaGreen",
	"CUP_H_RUS_Bandana_GSSh_Headphones",
	"H_Bandanna_khk_hs",
	"H_Bandanna_sand",
	"usm_bdu_boonie_blk",
	"usm_bdu_boonie_gry",
	"usm_bdu_cap_blk",
	"usm_bdu_boonie_odg",
	"usm_bdu_cap_gry",
	"usm_bdu_cap_odg",
	"usm_bdu_8point_blk",
	"usm_bdu_8point_gry",
	"usm_bdu_8point_odg",
	
	"H_Watchcap_blk",
	"rhs_beanie_green",
	"aegis_beanie_blk",
	"CUP_H_C_Beanie_01",
	"bear_beanie_brown",
	"CUP_H_C_Beanie_02",
	"CUP_H_C_Beanie_03",
	"H_Watchcap_cbr",
	"CUP_H_C_Beanie_04",
	"rhs_beanie",
	"CUP_H_PMC_Beanie_Black",
	"CUP_H_PMC_Beanie_Headphones_Black",
	"CUP_H_PMC_Beanie_Khaki",
	"CUP_H_PMC_Beanie_Headphones_Khaki",
	
	"CUP_H_PMC_Beanie_Winter",
	"CUP_H_PMC_Beanie_Headphones_Winter",
	"aegis_beanie_blk",
	"H_Beret_blk",
	"CUP_H_C_Beret_04",
	"CUP_H_C_Beret_01",
	"CUP_H_C_Beret_02",
	"CUP_H_C_Beret_03",
	"H_Beret_EAF_01_F",
	"rhsusf_Bowman",
	"simc_bush_hat_blk",
	"simc_bush_hat_flat_blk",
	"simc_bush_hat_foldf_blk",
	"simc_bush_hat_foldr_blk",
	"simc_bush_hat_tie_blk",
	
	"simc_bush_hat_og107_2",
	"simc_bush_hat_flat_og107_2",
	"simc_bush_hat_foldf_og107_2",
	"simc_bush_hat_foldr_og107_2",
	"simc_bush_hat_tie_og107_2",
	"H_Cap_blk",
	"H_Cap_grn",
	"H_Cap_oli",
	"H_Cap_oli_hs",
	"CUP_H_C_Policecap_01",
	"milgp_h_cap_01_gry",
	"milgp_h_cap_01_goggles_gry",
	"milgp_h_cap_02_gry",
	"milgp_h_cap_02_goggles_gry",
	"milgp_h_cap_03_gry",
	
	
	"milgp_h_cap_backwards_01_gry",
	"milgp_h_cap_backwards_01_goggles_gry",
	"milgp_h_cap_backwards_02_gry",
	"milgp_h_cap_backwards_02_goggles_gry",
	"milgp_h_cap_backwards_03_gry",
	"milgp_h_cap_backwards_03_goggles_gry",
	"rhs_cossack_visor_cap_tan",
	"rhs_cossack_papakha",
	"H_Tank_black_F",
	"H_Tank_eaf_F",
	"CUP_H_NAPA_Fedora",
	"rhs_fieldcap_m88",
	"rhs_fieldcap_m88_back",
	"CUP_H_C_Fireman_Helmet_01",
	"H_Construction_basic_black_F",
	
	"H_Construction_earprot_black_F",
	"H_Construction_headset_black_F",
	"H_Construction_basic_vrana_F",
	"H_Construction_earprot_vrana_F",
	"H_Construction_headset_vrana_F",
	"H_Construction_basic_white_F",
	"H_Construction_earprot_white_F",
	"H_Construction_headset_white_F",
	"H_Construction_basic_yellow_F",
	"H_Construction_earprot_yellow_F",
	"H_Construction_headset_yellow_F",
	"H_Hat_brown",
	"H_Hat_camo",
	"H_Hat_grey",
	"H_Hat_tan",
	
	"H_HeadBandage_clean_F",
	"H_HeadBandage_stained_F",
	"H_HeadBandage_bloody_F",
	"rhs_headband",
	"CUP_H_FR_Headband_Headset",
	"CUP_H_CZ_Pakol_headset_f_brown",
	"CUP_H_CZ_Pakol_headset_b_grey",
	"H_ParadeDressCap_01_US_F",
	"H_ParadeDressCap_01_LDF_F",
	"H_ParadeDressCap_01_CSAT_F",
	"H_ParadeDressCap_01_AAF_F",
	"rhs_pilotka",
	"H_ShemagOpen_khk",
	"H_Shemag_olive",
	"H_ShemagOpen_tan",
	
	"CUP_H_TK_Lungee",
	"CUP_H_TKI_Lungee_Open_01",
	"CUP_H_TKI_Lungee_Open_04",
	"CUP_H_TKI_Lungee_Open_05",
	"CUP_H_TKI_Lungee_Open_06",
	"CUP_H_TKI_Lungee_01",
	"CUP_H_TKI_Lungee_04",
	"CUP_H_TKI_Lungee_05",
	"CUP_H_TKI_Lungee_06",
	"CUP_H_TKI_Pakol_1_01",
	"CUP_H_TKI_Pakol_2_04",
	"CUP_H_TKI_Pakol_2_05",
	"CUP_H_TKI_Pakol_1_02",
	"CUP_H_TKI_SkullCap_01",
	"CUP_H_TKI_SkullCap_04",
	
	"CUP_H_TKI_SkullCap_06",
	"amf_tarte_artillerie",
	"H_Hat_Tinfoil_F",
	"rhs_tsh4",
	"rhs_tsh4_bala",
	"rhs_tsh4_ess",
	"rhs_tsh4_ess_bala",
	"rhs_ushanka",
	"CUP_H_C_Ushanka_03",
	"CUP_H_C_Ushanka_02",
	"CUP_H_C_Ushanka_01",
	"CUP_H_C_Ushanka_04",
	"TC_Helmet_CowboyHat1",
	"TC_Helmet_CowboyHat3",
	"TC_Helmet_CowboyHat2",
	
	//Facewear
	
	"pca_balaclava_blk_1",
	"pca_balaclava_blk_2",
	"rhs_facewear_6m2",
	"rhs_facewear_6m2_1",
	"G_AirPurifyingRespirator_02_black_F",
	"G_AirPurifyingRespirator_01_F",
	"G_Aviator",
	"G_Balaclava_blk",
	"G_Balaclava_combat",
	"G_Balaclava_Halloween_01",
	"G_Balaclava_lowprofile",
	"G_Balaclava_Flames1",
	"aegis_bala_skull",
	"rhs_ess_black",
	"rhs_googles_black",
	
	"rhs_googles_clear",
	"rhs_googles_orange",
	"rhs_googles_yellow",
	"G_Bandanna_aviator",
	"G_Bandanna_beast",
	"G_Bandanna_blk",
	"aegis_bandanna_kawaii",
	"G_Bandanna_shades",
	"G_Bandanna_sport",
	"G_Bandanna_Vampire_01",
	"CUP_Beard_Black",
	"CUP_Beard_Blonde",
	"CUP_Beard_Brown",
	"G_Blindfold_01_black_F",
	"G_Blindfold_01_white_F",
	"immersion_cigs_cigar0",
	"aegis_cigarette",
	"murshun_cigs_cig0",
	"G_Combat",
	"aegis_combat_gogg_blk",
	"CUP_G_ESS_BLK_Dark",
	"CUP_G_ESS_BLK_Ember",
	"CUP_G_ESS_BLK",
	"CUP_G_ESS_BLK_Facewrap_Black",
	"CUP_G_ESS_BLK_Scarf_Blk",
	"CUP_G_ESS_BLK_Scarf_Grn",
	"CUP_G_ESS_BLK_Scarf_Red",
	"CUP_G_ESS_BLK_Scarf_Face_Blk",
	"CUP_G_ESS_BLK_Scarf_Face_Grn",
	"CUP_G_ESS_BLK_Scarf_Face_Red",
	"CUP_G_ESS_KHK_Scarf_Face_Tan",
	
	"CUP_G_ESS_BLK_Scarf_White",
	"CUP_G_ESS_BLK_Scarf_Face_White",
	"CUP_G_ESS_BLK_Scarf_Blk_Beard",
	"CUP_G_ESS_BLK_Scarf_Blk_Beard_Blonde",
	"CUP_G_ESS_BLK_Scarf_Grn_Beard",
	"CUP_G_ESS_BLK_Scarf_Grn_Beard_Blonde",
	"CUP_G_ESS_BLK_Scarf_Red_Beard",
	"CUP_G_ESS_BLK_Scarf_Red_Beard_Blonde",
	"CUP_G_ESS_KHK_Scarf_Tan_Beard",
	"CUP_G_ESS_KHK_Scarf_Tan_Beard_Blonde",
	"CUP_G_ESS_BLK_Scarf_White_Beard",
	"CUP_G_ESS_BLK_Scarf_White_Beard_Blonde",
	"milgp_f_face_shield_blk",
	"milgp_f_face_shield_cb",
	"milgp_f_face_shield_goggles_blk",
	"milgp_f_face_shield_goggles_cb",
	"milgp_f_face_shield_goggles_shemagh_blk",
	"milgp_f_face_shield_goggles_shemagh_cb",
	"milgp_f_face_shield_shades_blk",
	"milgp_f_face_shield_shades_cb",
	"milgp_f_face_shield_shades_shemagh_blk",
	"milgp_f_face_shield_shades_shemagh_cb",
	"milgp_f_face_shield_shemagh_blk",
	"milgp_f_face_shield_shemagh_cb",
	"milgp_f_face_shield_tactical_blk",
	"milgp_f_face_shield_tactical_cb",
	"milgp_f_face_shield_tactical_shemagh_blk",
	"milgp_f_face_shield_tactical_shemagh_cb",
	"CUP_G_PMC_RadioHeadset",
	"CUP_G_PMC_RadioHeadset_Glasses",
	"CUP_G_PMC_RadioHeadset_Glasses_Dark",
	
	"CUP_G_Scarf_Face_Blk",
	"CUP_G_Scarf_Face_Grn",
	"CUP_G_Scarf_Face_Red",
	"CUP_G_Scarf_Face_Tan",
	"CUP_G_Scarf_Face_White",
	"pmk1_gas_mask",
	"G_Respirator_white_F",
	"G_Respirator_yellow_F",
	"CUP_G_TK_RoundGlasses",
	"CUP_G_TK_RoundGlasses_blk",
	"CUP_G_TK_RoundGlasses_gold",
	"G_Shades_Black",
	"G_Shades_Blue",
	"G_Shades_Red",
	"aegis_shades_yellow",
	"aegis_shemag_cbr",
	"aegis_shemag_shades_cbr",
	"aegis_shemag_tactical_cbr",
	"aegis_shemag_khk",
	"aegis_shemag_shades_khk",
	"aegis_shemag_tactical_khk",
	"aegis_shemag_oli",
	"aegis_shemag_shades_oli",
	"aegis_shemag_tactical_oli",
	"aegis_shemag_tan",
	"aegis_shemag_shades_tan",
	"aegis_shemag_tactical_tan",
	"aegis_shemag_white",
	"aegis_shemag_shades_white",
	"aegis_shemag_tactical_white",
	"rhsusf_shemagh_gogg_od",
	"rhsusf_shemagh2_gogg_od",
	
	"rhsusf_shemagh_gogg_white",
	"rhsusf_shemagh2_gogg_white",
	"rhsusf_shemagh_od",
	"rhsusf_shemagh2_od",
	"rhsusf_shemagh_white",
	"rhsusf_shemagh2_white",
	"rhsusf_oakley_goggles_blk",
	"rhsusf_oakley_goggles_clr",
	"rhsusf_oakley_goggles_ylw",
	"G_Spectacles",
	"G_Squares_Tinted",
	"G_Squares",
	"G_Sport_Red",
	"G_Sport_Blackyellow",
	"G_Sport_BlackWhite",
	"G_Sport_Checkered",
	"G_Sport_Blackred",
	"G_Sport_Greenblack",
	"usm_swdgoggles",
	"simc_goggles_swdg",
	"simc_goggles_swdg_low",
	"G_Tactical_Clear",
	"milgp_f_tactical_khk",
	"G_Spectacles_Tinted",
	"CUP_PMC_G_thug",
	"G_Goggles_VR",
	"G_WirelessEarpiece_F",
	
	
	//nvgslot
	
	"rhs_6m2_nvg",
	"rhs_6m2_1_nvg",
	"avon_fm12",
	"avon_sf12",
	"avon_fm12_nvg",
	"avon_sf12_nvg",
	"pca_nvg_balaclava",
	"pca_nvg_balaclava2",
	"pca_nvg_glasses_blk",
	"pca_nvg_glasses_clr",
	"pca_nvg_glasses_org",
	"pca_nvg_glasses_ylw",
	"pca_nvg_ess_blk",
	"immersion_cigs_cigar0_nv",
	"pca_nvg_cigarette",
	"murshun_cigs_cig0_nv",
	"pca_nvg_face_shield_blk",
	"pca_nvg_face_shield_cb",
	"pca_nvg_face_shield_shemagh_blk",
	"pca_nvg_face_shield_shemagh_cb",
	"immersion_pops_pop0_nv",
	"pca_nvg_shemagh_white",
	"pca_nvg_shemagh2_white",
	"pca_nvg_shemagh_od",
	"pca_nvg_shemagh2_od",
	"pca_nvg_shemagh_lowered_cbr",
	"pca_nvg_shemagh_lowered_khk",
	"pca_nvg_shemagh_lowered_oli",
	"pca_nvg_shemagh_lowered_tan",
	"pca_nvg_shemagh_lowered_white",
	"pca_nvg_oakley_goggles_blk",
	"pca_nvg_oakley_goggles_clr",
	"pca_nvg_oakley_goggles_ylw",
	"pca_nvg_tactical_glasses",
	
	"pca_nvg_tactical_goggles",
	"murshun_cigs_cigpack",
	"murshun_cigs_lighter"
	
];

//********************************************************************************//
// Miscellaneous Items
//********************************************************************************//

private _itemMisc = 
[
	"ACRE_PRC343",
	"ACE_MapTools",
	"ACE_RangeCard",
	"ItemCompass",
	"ItemMap",
	"ItemWatch",
	"ToolKit"
];

{
	_itemGear pushBackUnique _x;
} forEach (primaryWeaponItems _unit);

{
	_itemGear pushBackUnique _x;
} forEach (handgunItems _unit);

{
	_itemGear pushBackUnique _x;
} forEach (assignedItems _unit);

_itemGear pushBack uniform _unit;
_itemGear pushBack vest _unit;
_itemGear pushBack headgear _unit;
_itemGear pushBack backpack _unit;

switch (true) do 
{
	//Rifleman
	case (_unitRole == "rm") : 
	{
		[arsenal, (_itemGear + _itemCosmetic + _itemMisc )] call ace_arsenal_fnc_initBox;
	};
};

_action = 
[
	"personal_arsenal", "Personal Arsenal", "\A3\ui_f\data\igui\cfg\weaponicons\MG_ca.paa",
	{
		[arsenal, _player] call ace_arsenal_fnc_openBox
	},
	{
		(player distance2D (player getVariable ["startPos", [0,0,0]])) < 100
	},
	{},
	[],
	[0,0,0],
	3
] call ace_interact_menu_fnc_createAction;

[_unit, 1, ["ACE_SelfActions"], _action] call ace_interact_menu_fnc_addActionToObject;