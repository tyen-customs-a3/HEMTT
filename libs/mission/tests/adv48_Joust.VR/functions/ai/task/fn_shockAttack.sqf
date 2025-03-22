/**
	Modified from nkenny's Creeping Script
	Order units to get as close as possible to the target before opening fire
	
	Parameters:
	0: the group <GROUP> or unit <OBJECT> 
	1: range of tracking, default 1000 <NUMBER>
	2: update cycles in seconds, default 30 <NUMBER>
	3: center position, if no position is given, the group's position will be used as the center <ARRAY>
	4: add a random radius to target's position, default 50 <NUMBER>
	
	MUST be done in a scheduled environment (SPAWN)
	
	Example:
	[bob, 1000] spawn pca_fnc_shockAttack;
	
	Returns: 
	BOOL
*/

if !(canSuspend) exitWith 
{
	_this spawn pca_fnc_shockAttack;
};

params 
[
	["_grp", grpNull, [grpNull, objNull]],
	["_range", 1000, [0]],
	["_cycle", 30, [0]],
	["_pos", [], [[]]],
	["_radius", 50, [0]]
];

private _fnc_shockOrderrs = 
{
	params ["_grp", "_target"];
	
	private _newDist = (leader _grp) distance2D _target;
	private _inForest = ((selectBestPlaces [getPos (leader _grp), 2, "(forest + trees) * 0.5", 1, 1]) select 0) select 1;
	
	/*
	if (behaviour (leader _grp) isEqualTo "COMBAT") exitWith 
	{
		_grp setCombatMode "RED";
		
		{
			_x setUnitPosWeak "MIDDLE";
			_x doMove _targetPos;
			true;
		} count (units _grp);
	};
	*/
	
	if (_inForest > 0.9 || _newDist > 200) then 
	{
		{
			_x setUnitPosWeak "UP"; 
			true;
		} count (units _grp);
	};
	if (_inForest < 0.6 || _newDist < 100) then 
	{
		{
			_x setUnitPosWeak "MIDDLE"; 
			true;
		} count (units _grp);
	};
	if (_newDist < 50) exitWith 
	{
		_grp setCombatMode "RED";
		_grp setBehaviour "STEALTH";
	};
	
	{
		private _targetPos = (getPosATL (_target) vectorAdd [((random (_radius * 2)) - _radius), ((random (_radius * 2)) - _radius), 0]);
		
		_x doMove _targetPos;
		true;
	} count (units _grp);
};

if (!local _grp) exitWith {false};
if (_grp isEqualType objNull) then 
{
	_grp = group _grp;
};

_grp setBehaviour "AWARE";
_grp setFormation "WEDGE";
_grp setSpeedMode "FULL";
_grp setCombatMode "GREEN";
_grp enableAttack false;

{
	_x addEventHandler 
	["FiredNear", 
		{
			params ["_unit"];
			
			_unit setCombatMode "RED";
			(group _unit) enableAttack true;
			
			_unit removeEventHandler ["FiredNear", _thisEventHandler];
		}
	];
	true;
} count units _grp;

waitUntil 
{
	waitUntil {sleep 1; simulationEnabled leader _grp};
	
	private _target = [_grp, _range, _pos] call pca_fnc_findClosestTarget;
	
	if (!isNull _target) then 
	{
		[_grp, _target] call _fnc_shockOrderrs;
		sleep _cycle;
	}
	else
	{
		_grp setCombatMode "GREEN";
		sleep (_cycle * 4);
	};
	
	((units _grp) findIf {_x call pca_fnc_isAlive} == -1)
};

//end
true;
