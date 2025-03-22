/**
	Garrison buildings in a given point 
	
	Parameters:
	0: the position <POSITION>
	1: the check radius <NUMBER>
	2: maximum unit allowed to spawn <NUMBER>
	
	Example:
	[getPos bob, 100, 10] call pca_fnc_createGarrison;
	
	Returns: 
	All spawned units and groups
*/

params ["_centerPos", "_radius", "_maxUnitAllowed"];

([_centerPos, _radius] call pca_fnc_nearbyBuildings) params ["_buildings", "_numOfBuildingPos"];
private _numOfUnits = _maxUnitAllowed min _numOfBuildingPos;

[_buildings, true] call CBA_fnc_shuffle;

private _groups = [];
private _spawnedUnits = [];

while {_numOfUnits > 0 && _numOfBuildingPos > 0} do 
{
	private _index = _buildings call BIS_fnc_randomIndex;
	(_buildings # _index) params ["_positions", "_building"]; 
	
	private _grp = _building getVariable ["pca_garrison_group", grpNull];
	
	if (isNull _grp) then 
	{
		_grp = createGroup [east, true];
		_building setVariable ["pca_garrison_group", _grp];
		_grp setBehaviour "SAFE";
		_groups pushBack _grp;
	};
	
	private _numOfPositions = (count _positions);
	private _unitsToSpawn = (random [1, _numOfPositions / 4, _numOfPositions]) min _numOfUnits;
	
	for "_i" from 1 to _unitsToSpawn do 
	{
		private _posIndex = _positions call BIS_fnc_randomIndex;
		private _buildingPos = _positions # _posIndex;
		private _unitRole = selectRandomWeighted 
		[
			"rm", 0.55,
			"rm_lat", 0.1,
			"rm_mat", 0.05,
			"ar", 0.2,
			"mg", 0.1
		];
		
		private _unit = _grp createUnit ["O_Soldier_F", [0,0,0], [], 0, "NONE"];
		_unit setPos _buildingPos;
		[_unit, pca_opfor_faction, _unitRole] call pca_fnc_assignFactionRole;
		_unit disableAI "PATH";
		_unit setUnitPos "UP";
		_unit setDir (0 + (random 360));
		
		_numOfUnits = _numOfUnits - 1;
		_spawnedUnits pushBack _unit;
		_numOfBuildingPos = _numOfBuildingPos - 1;
		_positions deleteAt _posIndex;
	};
	
	if (count _positions <= 0) then 
	{
		_buildings deleteAt _index;
	};
};

[_spawnedUnits, _groups]
