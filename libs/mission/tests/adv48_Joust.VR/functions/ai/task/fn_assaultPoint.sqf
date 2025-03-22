/**
	Modified from nkenny's Assault Script
	Order units to assault building positions or open terrain based on enemy positions
	
	Parameters:
	0: the group leader <OBJECT> 
	1: the target <OBJECT>
	2: units in the group, default all <ARRAY>
	3: range to find buildings, default 50 <ARRAY>
	4: adds random radius to final position, default 0 <NUMBER>
	
	Example:
	[bob, joe, [], 50] call pca_fnc_assaultPoint;
	
	Returns: 
	BOOL
*/

params ["_unit", ["_target", objNull], ["_units", []], ["_range", 50], ["_radius", 0]];

if !(_unit call pca_fnc_isAlive) exitWith {false};

if (_units isEqualTo []) then 
{
	_units = [_unit, 30] call pca_fnc_findReadyUnits;
};

if (_units isEqualTo []) exitWith {false};

private _potentialPos = [_target, _range, true, false] call pca_fnc_findBuildings;
_potentialPos = _potentialPos select {_x distance _target < 5};

private _pos = if (_potentialPos isEqualTo []) then 
{
	if (_unit call pca_fnc_isIndoor) exitWith 
	{
		getPosATL _unit;
	};
	
	getPosATL _target;
}
else
{
	_potentialPos pushBack (getPosATL _target);
	selectRandom _potentialPos;
};

if (_radius > 0) then 
{
	_pos = _pos vectorAdd [((random (_radius * 2)) - _radius), ((random (_radius * 2)) - _radius), 0]; 
};

_units doWatch _target;
_units doMove _pos;
_unit setDestination [_pos, "LEADER PLANNED", true];

//end
true;