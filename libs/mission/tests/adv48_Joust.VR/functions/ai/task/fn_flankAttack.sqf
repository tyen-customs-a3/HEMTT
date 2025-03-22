/**
	Order the group to perform a flanking attack on a target.
	
	Parameters:
	0: the attacking group <GROUP>
	1: the position of the target <XYZ, OBJECT or GROUP>
	2: angle in which the group will flank the target <NUMBER> <DEFAULT: 30>
	3: a randomized radius the waypoint will be picked from its original position <NUMBER> <DEFAULT: 0>
	4: distance between each waypoint order <NUMBER> <DEFAULT: 100>
	
	Example:
	[bob, joe] spawn pca_fnc_flankAttack;
	
	Returns: 
	NONE
*/

//waypointDistance should be set at values exceeding 50 minimum.
//too low values will creates unnecessary or otherwise redundant move waypoints.
params ["_group", "_targetPos", ["_flankAngle", 30], ["_radius", 0], ["_waypointDistance", 100]];

[_group] call CBA_fnc_clearWaypoints;

private _pos = (position leader _group);
private _coverPos = [];
private _unitDir = 0;
private _angle = selectRandom [(_flankAngle - (_flankAngle * 2)), 0, _flankAngle];

if !(surfaceIsWater _TargetPos) then 
{
	while {(_pos distance _targetPos) > _waypointDistance} do 
	{
		sleep 1;
		
		_unitDir = [_pos, _targetPos] call BIS_fnc_dirTo;
		_pos = [_pos, (((_pos distance _targetPos) / 2) max _waypointDistance), (_unitDir - (_angle))] call BIS_fnc_relPos;
		_coverPos = selectBestPlaces [_pos, 30, "forest+tree+2*hills+houses", 1, 1];
		
		if ((count (_coverPos)) > 0) then 
		{
			_coverPos = (_coverPos select 0 select 0);
			_pos = _coverPos;
		};
		
		if !(surfaceIsWater _pos) then 
		{
			[_group, _pos, _radius, "MOVE", "AWARE", "YELLOW", "FULL", "WEDGE"] call CBA_fnc_addWaypoint;
		};
	};
	
	[_group, _targetPos, _radius, "SAD", "AWARE", "YELLOW", "FULL", "WEDGE"] call CBA_fnc_addWaypoint;
};

((count (waypoints _group)) - currentWaypoint _group);
