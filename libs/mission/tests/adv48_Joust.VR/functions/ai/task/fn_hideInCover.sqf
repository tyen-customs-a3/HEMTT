/**
	Modified from nkenny's Cover Script
	Order units to disperse into covers
	
	Parameters:
	0: the group leader <OBJECT> 
	1: the target <OBJECT> or position <ARRAY>
	2: units in the group, default all <ARRAY>
	3: predefined covers <ARRAY>
	4: ready anti-tank weapons? <BOOL>
	
	Example:
	[bob, joe] call pca_fnc_hideInCover;
	
	Returns: 
	BOOL
*/

#define RETREAT_DISTANCE 8
#define COVER_DISTANCE 18
#define BUILDING_DISTANCE 30

params ["_unit", "_target", ["_units", []], ["_cover", []], ["_antiTank", false]];

_target = _target call CBA_fnc_getPos;
private _grp = group _unit;

if !(_unit call pca_fnc_isAlive) exitWith {false};

_unit setBehaviour "COMBAT";
_grp setFormation "WEDGE";
_grp setCombatMode "GREEN";
_grp enableAttack false;

private _units = units _unit;

if (_units isEqualTo []) then 
{
	_units = [_unit, 30] call pca_fnc_findReadyUnits;
};

if (_units isEqualTo []) exitWith {false};

if (_cover isEqualTo []) then 
{
	private _coverPos = _unit getPos [RETREAT_DISTANCE, _target getDir _unit];
	_cover = (nearestTerrainObjects [_unit, ["BUSH", "TREE", "SMALL TREE", "HIDE"], COVER_DISTANCE, true, true]) apply {_x getPos [1.5, _target getDir _x]};
	
	_cover append ([_coverPos, BUILDING_DISTANCE, true, true] call pca_fnc_findBuildings);
	
	private _distance2D = (_unit distance2D _target) + 2;
	_cover = _cover select {_x distance2D _target > _distance2D};
};

_unit doWatch objNull;

[_units, _target, _cover] call pca_fnc_doHideInCover;

private _launchers = _units select {(secondaryWeapon _x) isNotEqualTo ""};

private _enemies = _unit targets [true, 600, [], 0, _target];
private _vehicles = _enemies findIf {(vehicle _x) isKindOf "Tank" || {(vehicle _x) isKindOf "Air"}};

if (_antiTank && {_vehicles != -1} && {_launchers isNotEqualTo []}) then 
{
	{
		_x setCombatMode "RED";
		_x commandTarget (_enemies select _vehicles);
		
		_x selectWeapon (secondaryWeapon _x);
		_x setUnitPosWeak "MIDDLE";
	} forEach _launchers;
	
	_unit doFire (_enemies select _vehicles);
};

//end
true;
