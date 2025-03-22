/**
	Creates a squad on a given position
	
	Parameters:
	0: the position <POSITION>
	1: vehicle class name <STRING>
	2: number of unit to spawn <NUMBER>
	3: randomize the unit's role if true <BOOL> <DEFAULT: TRUE>
	
	Example:
	[getPos bob, "O_MRAP_02_F", 4] call pca_fnc_createVehicle;
	
	Returns: 
	The spawned group 
*/

params ["_spawnPos", ["_vehicleType", ""], "_numOfUnits", ["_random", true]];

private _veh = _vehicleType createVehicle _spawnPos;

private _grp = createGroup [east, true];

_grp allowFleeing 0;
_grp deleteGroupWhenEmpty true;
_grp addVehicle _veh;

private _roles = ["rm","rm_lat","rm_mat","ar","mg"];
private _usedRoles = +_roles;

while {_numOfUnits > 0} do 
{
	private _role = "rm";
	
	if (_random) then 
	{
		_role = _roles selectRandomWeighted [0.55,0.1,0.05,0.2,0.1];
	}
	else
	{
		if (count _usedRoles <= 0) then 
		{
			_usedRoles = +_roles;
		};
		
		_role = _usedRoles deleteAt 0;
	};
	
	private _unit = _grp createUnit ["O_Soldier_F", [_spawnPos # 0, _spawnPos # 1, 0], [], 5, "NONE"];
	[_unit, pca_opfor_faction, _role] call pca_fnc_assignFactionRole;
	
	_unit moveInAny _veh;
	
	_numOfUnits = _numOfUnits - 1;
};

_grp;