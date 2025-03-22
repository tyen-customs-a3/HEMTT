/**
	Modified from nkenny's Assault Script
	Order units to assault a given location
	
	Parameters:
	0: the group leader <OBJECT> 
	1: the target <OBJECT> or position <ARRAY>
	2: units in the group, default all <ARRAY>
	3: amount of attack cycles <NUMBER>
	4: adds random radius to final position, default 0 <NUMBER>
	
	Example:
	[bob, joe] call pca_fnc_attackPoint;
	
	Returns: 
	BOOL
*/

params ["_unit", "_target", ["_units", []], ["_cycle", 15], ["_radius", 0]];

_target = _target call CBA_fnc_getPos;
private _grp = group _unit;

if !(_unit call pca_fnc_isAlive) exitWith {false};

if (_units isEqualTo []) then 
{
	_units = [_unit, 30] call pca_fnc_findReadyUnits;
};

if (_units isEqualTo []) exitWith {false};

private _potentialPos = [_target, 8, true, false] call pca_fnc_findBuildings;
_potentialPos append ((_unit targets [true, 10, [], 0, _target]) apply {_unit getHideFrom _x});
_potentialPos pushBack _target;

_grp setFormDir (_unit getDir _target);
_units doWatch _target;

[_cycle, _units, _potentialPos, _radius] call pca_fnc_doAttackPoint; 

//end
true;