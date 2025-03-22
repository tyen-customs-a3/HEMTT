/**
	Modified from dedmen's script
	Finds closest target from a unit
	
	Parameters:
	0: group to check <GROUP>
	1: check radius <NUMBER> 
	2: position to compare radius to <ARRAY>
	3: limits the target to player only <BOOL>
	
	Example:
	[group bob, 500] call pca_fnc_findClosestTarget;
	
	Returns: 
	closest unit
*/

params 
[
	["_grp", grpNull, [grpNull, objNull]],
	["_range", 500, [0]],
	["_pos", [], [[]]],
	["_onlyPlayers", true, [false]]
];

private _grpLeader = leader _grp;
private _sideExclusion = [side _grp, civilian, sideUnknown, sideEmpty, sideLogic];

if (_pos isEqualTo []) then 
{
	_pos = _grpLeader;
};

_pos = _pos call CBA_fnc_getPos;

private _units = [allUnits, switchableUnits + playableUnits] select _onlyPlayers;

_units = _units select 
{
	!(side _x in _sideExclusion) 
	&& {_x distance2D _pos < _range} 
	&& {(getPosATL _x) select 2 < 200}
	&& {[side _x, side _grp] call BIS_fnc_sideIsEnemy}
};

if (_units isEqualTo []) exitWith {objNull};

private _unitDist = _units apply {[_grpLeader distance2D _x, _x]};
_unitDist sort true;

(_unitDist param [0, [0, objNull]]) param [1, objNull];
