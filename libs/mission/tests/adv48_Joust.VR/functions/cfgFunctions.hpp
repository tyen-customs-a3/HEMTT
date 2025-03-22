class ai 
{
	tag = "pca";
	class ai_action 
	{
		file = "functions\ai\action";
		class doAttackPoint {};
		class doFlankPoint {};
		class doHideInCover {};
		class doSeekCover {};
	};
	class ai_attribute
	{
		file = "functions\ai\attribute";
		class assignFactionRole {};
		class assignGearCurated {};
		class setUnitSkills {};
		class unlimitedAmmo {};
	};
	class ai_cache 
	{
		file = "functions\ai\cache";
		class groupCache {};
		class unitCache {};
		class unitCacheDeactivate {};
	};
	class ai_common 
	{
		file = "functions\ai\common";
		class deleteGroup {};
		class enemyInRadius {};
		class findClosestTarget {};
		class findReadyUnits {};
		class generateBuildingWaypoints {};
		class isAlive {};
		class isIndoor {};
		class playerInRadius {};
		class unitCheckLOS {};
		class unitShareInformation {};
	};
	class ai_data
	{
		file = "functions\ai\data";
		class applyWaypoints {};
		class saveCrew {};
		class saveGroup {};
		class saveUnit {};
		class saveVehicle {};
		class saveWaypoints {};
		class spawnGroup {};
		class spawnUnit {};
		class spawnVehicle {};
	};
	class ai_spawn
	{
		file = "functions\ai\spawn";
		class createGarrison {};
		class createGroupSync {};
		class createSquad {};
		class createVehicle {};
	};
	class ai_task 
	{
		file = "functions\ai\task";
		class assaultPoint {};
		class attackPoint {};
		class clearBuildings {};
		class flankAttack {};
		class flankPoint {};
		class garrisonArea {};
		class garrisonPoint {};
		class hideInCover {};
		class huntTargets {};
		class moveToBuildings {};
		class patrolArea {};
		class patrolAreaPolygon {};
		class shockAttack {};
	};
};

class common 
{
	tag = "pca";
	class common
	{
		file = "functions\common";
		class drawObjectMapMarker {};
		class findBuildings {};
		class findOverwatch {};
		class nearbyBuildings {};
		class removeEventHandlers {};
	};
};

class player 
{
	tag = "pca";
	class player_eh 
	{
		file = "functions\player\eh";
		class explosionEH {};
		class hitEH {};
	};
	class player_jip 
	{
		file = "functions\player\jip";
		class jipChooseTarget {};
		class jipEmptySeat {};
	};
	class player_loadout
	{
		file = "functions\player\loadout";
		class curatedArsenal {};
	};
};