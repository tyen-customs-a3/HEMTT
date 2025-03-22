/**
	Modified from nkenny's Garrison Script
	Order units to occupy buildings near the target
	
	Parameters:
	0: the group leader <OBJECT> 
	1: the target <OBJECT> or position <ARRAY>
	2: units in the group, default all <ARRAY>
	
	Example:
	[bob, joe] call pca_fnc_garrisonPoint;
	
	Returns: 
	BOOL
*/

#define COVER_DISTANCE 25
#define BUILDING_DISTANCE 25

params ["_unit", "_target", ["_units", []]];

_target = _target call CBA_fnc_getPos;
private _grp = group _unit;

if !(_unit call pca_fnc_isAlive) exitWith {false};

_grp setBehaviour "COMBAT";
_grp setFormation "WEDGE";
_grp enableAttack false;

if (_units isEqualTo []) then 
{
	_units = [_unit, 30] call pca_fnc_findReadyUnits;
};

if (_units isEqualTo []) exitWith {false};

private _potentialPos = [_target, BUILDING_DISTANCE, true, false] call pca_fnc_findBuildings;
_potentialPos = _potentialPos apply {[_x select 2, _x]};
_potentialPos sort false;
_potentialPos = _potentialPos apply {_x select 1};

if (count _potentialPos < count _units) then 
{
	private _cover = (nearestTerrainObjects [_target, ["BUSH", "TREE", "HIDE", "WALL", "FENCE"], COVER_DISTANCE, false, true]) apply {_x getPos [1.5, random 360]};
	_potentialPos append _cover;
};

if (_potentialPos isEqualTo []) exitWith 
{
	{
		_x doFollow leader _x;
	} forEach _units;
};

_target = _potentialPos select 0;
doStop _units;
_units doWatch _target;

{
	private _pos = if (_potentialPos isEqualTo []) then 
	{
		_target;
	}
	else
	{
		_potentialPos deleteAt 0;
	};
	
	[
		{
			params ["_unit", "_pos"];
			_unit moveTo _pos;
			_unit setDestination [_pos, "FORMATION PLANNED", true];
		}, [_x, _pos], random 2
	] call CBA_fnc_waitAndExecute;
} forEach _units;

//end
true;