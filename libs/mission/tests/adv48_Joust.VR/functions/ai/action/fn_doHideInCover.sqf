/**
	Modified from nkenny's Cover Script
	actualizes the cover action
	
	Parameters:
	0: list of units <ARRAY> 
	1: the target <OBJECT> or position <ARRAY>
	2: list of buildings nearby <ARRAY>
	
	Example:
	[units bob, joe] call pca_fnc_doHideInCover;
	
	Returns: 
	BOOL
*/

params ["_units", "_pos", ["_buildings", []]]; 

_units = _units select {(_x call pca_fnc_isAlive) && {isNull objectParent _x} && {!isPlayer _x}};
if (_units isEqualTo []) exitWith {false};

{
	[
		{
			params ["_arguments"];
			_arguments call pca_fnc_doSeekCover;
		}, [_x, _pos, nil, _buildings], 1 + random 2
	] call CBA_fnc_waitAndExecute;
} forEach _units;

//end
true;
