/**
	Modified from nkenny's Flanking Script
	Order units to conduct flanking maneuvers
	
	Parameters:
	0: the group leader <OBJECT> 
	1: the target <OBJECT> or position <ARRAY>
	2: units in the group, default all <ARRAY>
	3: amount of attack cycles <NUMBER>
	4: default overwatch position <ARRAY>
	5: adds a random radius to target's final position? default 0 <NUMBER>
	
	Example:
	[bob, joe] call pca_fnc_flankPoint;
	
	Returns: 
	BOOL
*/

params ["_unit", "_target", ["_units", []], ["_cycle", 15], ["_overwatch", []], ["_radius", 0]];

_target = _target call CBA_fnc_getPos;
private _grp = group _unit;

if !(_unit call pca_fnc_isAlive) exitWith {false};

if (_units isEqualTo []) then 
{
	_units = [_unit, 30] call pca_fnc_findReadyUnits;
};

if (_units isEqualTo []) exitWith {false};

private _potentialPos = [_target, 12, true, false] call pca_fnc_findBuildings;
_potentialPos pushBack _target;

if (_overwatch isEqualTo []) then 
{
	private _distance2D = ((_unit distance2D _target) * 0.66) min 250;
	_overwatch = selectBestPlaces [_target, _distance2D, "(2*hills) + (2*forest+trees+houses) - (2*meadow) - (2*windy) - (2*sea) - (10*deadBody)", 100, 3] apply {[(_x select 0) distance2D _unit, _x select 0]};
	_overwatch = _overwatch select {!(surfaceIsWater (_x select 1))};
	_overwatch sort true;
	_overwatch = _overwatch apply {_x select 1};
	
	if (_overwatch isEqualTo []) then 
	{
		_overwatch pushBack ([getPos _unit, _distance2D, 100, 8, _target] call pca_fnc_findOverwatch);
	};
	
	_overwatch = _overwatch select 0;
};

_unit setSpeedMode "FULL";
(units _unit) allowGetIn false;
_grp setFormDir (_unit getDir _target);
_units commandMove _overwatch;

[{_this call pca_fnc_doFlankPoint}, [_cycle, _units, _pos, _overwatch, _radius], 2 + random 8] call CBA_fnc_waitAndExecute;

//end
true;