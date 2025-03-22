/**
	Modified from nkenny's Patrol Script
	Order units to patrol a given area
	
	Parameters:
	0: the group <GROUP> or unit <OBJECT> performing the patrol
	1: starting patrol position  <ARRAY>
	2: the patrol radius <NUMBER>
	3: amount of waypoint, default 4 <NUMBER>
	4: loops the patrol if true, default true <BOOL>
	
	Example:
	[bob, getPos bob, 500] call pca_fnc_patrolArea;
	
	Returns: 
	BOOL
*/

if (canSuspend) exitWith {[pca_fnc_patrolArea, _this] call CBA_fnc_directCall;};

params [["_grp", grpNull, [grpNull, objNull]], ["_pos", [], [[]]], ["_radius", 200, [0]], ["_waypointCount",4], ["_continue", true]];

if (!local _grp) exitWith {};

if (_grp isEqualType objNull) then 
{
	_grp = group _grp;
};

if (_pos isEqualTo []) then 
{
	_pos = _grp;
};

_pos = _pos call CBA_fnc_getPos;

[_grp] call CBA_fnc_clearWaypoints;

_grp setBehaviour "SAFE";
_grp setSpeedMode "LIMITED";
_grp setCombatMode "YELLOW";
_grp setFormation "STAG COLUMN";
_grp enableGunLights "forceOn";

if (isNil "pca_fnc_patrolWaypointStatement") then 
{
	pca_fnc_patrolWaypointStatement = 
	{
		private _grp = group this;
		private _radius = _grp getVariable ["pca_patrol_radius", 200];
		private _pos = _grp getVariable ["pca_patrol_position", getPos (leader _group)];
		
		{
			if ((currentWaypoint _grp) != (_x select 1)) then 
			{
				private _nextPos = _pos getPos [_radius * (1 - abs random [-1, 0, 1]), random 360];
				
				if (surfaceIsWater _nextPos) then 
				{
					_nextPos = _pos;
				};
				
				_x setWPPos _nextPos;
			};
		} forEach waypoints _grp;
	};
};

private _waypoint = nil;
private _firstWaypointID = 0;

for "_i" from 1 to _waypointCount do 
{
	private _nextPos = _pos getPos [_radius * (1 - abs random [-1, 0, 1]), random 360];
	
	if (surfaceIsWater _nextPos) then 
	{
		_nextPos = _pos;
	};
	
	_waypoint = _grp addWaypoint [_nextPos, 10];
	_waypoint setWaypointType "MOVE";
	_waypoint setWaypointTimeout [8, 10, 15];
	_waypoint setWaypointCompletionRadius 10;
};

_grp setVariable ["pca_patrol_radius", _radius, true];
_grp setVariable ["pca_patrol_position", _pos, true];

if (_continue) then 
{
	_waypoint setWaypointStatements ["true", format ["if (local this) then {(group this) setCurrentWaypoint [(group this), %1]; call %2;};", _firstWaypointID, "pca_fnc_patrolWaypointStatement"]];
}
else
{
	_waypoint setWaypointStatements ["true", format ["if (local this) then {(group this) setCurrentWaypoint [(group this), %1];};", _firstWaypointID]];
};

//end
true;