/**
	Check if unit is inside a building
	
	Parameters:
	0: unit to be checked <OBJECT>
	
	Example:
	bob call pca_fnc_isIndoor;
	
	Returns: 
	true if unit is indoor
*/

params ["_unit"];

private _trace = lineIntersectsSurfaces [eyePos _unit, eyePos _unit vectorAdd [0, 0, 10], _unit, objNull, true, -1, "GEOM", "NONE", true];

if (_trace isEqualTo []) exitWith {false};

_trace findIf {_x select 3 isKindOf "Building"} != -1;