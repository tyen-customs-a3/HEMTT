/**
	Modified from nkenny's Assault Script
	Finds units ready for group actions
	
	Parameters:
	0: originating unit <OBJECT>
	1: find units within range, default 50 <NUMBER> 
	2: find units within given pool, default [] <ARRAY>
	
	Example:
	[bob, 50] call pca_fnc_findReadyUnits;
	
	Returns: 
	available units
*/

params 
[
	["_unit", objNull, [objNull]],
	["_radius", 200, [0]],
	["_units", [], [[]]]
];

if (_units isEqualTo []) then 
{
	_units = units _unit;
};

_units = _units select 
{
	_x distance2D _unit < _radius 
	&& {!isPlayer _x} 
	&& {!fleeing _x} 
	&& {isNull objectParent _x} 
	&& {_x checkAIFeature "PATH"} 
	&& {_x checkAIFeature "MOVE"} 
	&& {_x call pca_fnc_isAlive}
	&& {!(currentCommand _x in ["GET IN", "ACTION", "HEAL"])} 
};

//return
_units;