/**
	Modified from nkenny's Assault Script
	actualizes the assault cycle
	
	Parameters:
	0: the number of cycles to repeat <NUMBER> 
	1: unit list <ARRAY>
	2: list of positions to utilize <ARRAY>
	3: radius added to target's final position <NUMBER>
	
	Example:
	[15, units bob] call pca_fnc_doAttackPoint;
	
	Returns: 
	BOOL
*/

params ["_cycle", "_units", "_pos", "_radius"]; 

_units = _units select {(_x call pca_fnc_isAlive) && {!isPlayer _x}};
if (_units isEqualTo []) exitWith {};

{
	private _targetPos = selectRandom _pos;
	
	if (_radius > 0) then 
	{
		_targetPos = _targetPos vectorAdd [((random (_radius * 2)) - _radius), ((random (_radius * 2)) - _radius), 0];
	};
	
	if ((currentCommand _x) isNotEqualTo "MOVE") then 
	{
		_x doMove _targetPos;
		_x setDestination [_targetPos, "FORMATION PLANNED", false]; 
		_x lookAt _targetPos;
	};
	
	_x forceSpeed 4;
	_x setUnitPosWeak (["UP", "MIDDLE"] select (getSuppression _x isNotEqualTo 0));
} forEach (units _grp);

if !(_cycle <= 1 || {_units isEqualTo []}) then 
{
	[
		{_this call pca_fnc_doAttackPoint},
		[_cycle - 1, _units, _pos, _radius],
		5
	] call CBA_fnc_waitAndExecute;
};

//end
true;
