/**
	Modified from nkenny's Cover Script
	order units to hide in nearby cover positions or buildings.
	
	Parameters:
	0: unit hiding <OBJECT> 
	1: the target <OBJECT> or position <ARRAY>
	2: range to search for cover and concealment, default 35 <NUMBER>
	3: array of predetermined building positions <ARRAY>
	
	Example:
	[bob, joe, 30] call pca_fnc_doSeekCover;
	
	Returns: 
	BOOL
*/

params ["_unit", "_pos", ["_range", 35], ["_buildings", []]]; 

if 
(
	(currentCommand _unit) in ["GET IN", "ACTION", "HEAL"] 
	|| {!(_unit checkAIFeature "PATH")}
	|| {!(_unit checkAIFeature "MOVE")}
) exitWith {false};

if (_unit call pca_fnc_isIndoor) exitWith 
{
	doStop _unit;
	false;
};

if (_buildings isEqualTo []) then 
{
	_buildings = [_unit, _range, true, true] call pca_fnc_findBuildings;
};

doStop _unit;
_unit forceSpeed 4;

if (_buildings isNotEqualTo []) exitWith 
{
	_unit setUnitPosWeak "MIDDLE";
	_unit doMove (selectRandom _buildings);
};

private _cover = nearestTerrainObjects [_unit getPos [1, getDir _unit], ["BUSH", "TREE", "SMALL TREE", "HIDE", "ROCK", "WALL", "FENCE"], 9, true, true];

private _cover = [];

private _targetPos = if (_cover isEqualTo []) then 
{
	_unit getPos [10 + random _range, (_pos getDir _unit) + 45 - random 90];
}
else
{
	(_cover select 0) getPos [-1.2, _unit getDir (_cover select 0)];
};

if (surfaceIsWater _targetPos) then 
{
	_targetPos = getPosATL _unit;
};

_unit doMove _targetPos;

//end
true;
