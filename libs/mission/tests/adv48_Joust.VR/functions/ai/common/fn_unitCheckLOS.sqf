/**
	Checks to see if a unit can see a given target within the specified range.
	
	Parameters:
	0: a unit <OBJECT>
	1: the target <LOCATION, OBJECT or GROUP>
	2: the range to check <NUMBER> (DEFAULT: 200)
	3: Field of Views to check <NUMBER> (DEFAULT: 120)
	
	Example:
	[bob, joe, 200, 120] call pca_fnc_unitCheckLOS;
	
	Returns: 
	True if unit have direct LOS on target within range, else false <BOOL>
*/

params ["_unit", "_target", ["_range", 200], ["_fov", 120]];

private _hasLOS = false;
private _inRange = (_unit distance _target) < _range;

if (_inRange) then 
{
	_checkView = [position _unit, getDir _unit, _fov, position _target] call BIS_fnc_inAngleSector;
	_checkSight = count (lineIntersectsWith [eyePos _unit, eyePos _target, _unit, _target]) == 0;
	_hasLOS = _checkView && _checkSight;
};

//Return
_hasLOS;