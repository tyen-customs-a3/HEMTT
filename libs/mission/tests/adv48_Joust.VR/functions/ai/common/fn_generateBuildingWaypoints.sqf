/**
	Assign waypoints to nearby buildings and returns the number of assigned buildings.
	
	Parameters:
	0: the group <GROUP>
	1: position used to search for nearby building <LOCATION>
	2: the search radius for buildings <NUMBER> (DEFAULT: 50)
	3: all nearby buildings within the search radius <OBJECT>
	4: move orders assigned to groups <WAYPOINT>
	
	Example:
	[bob, getPos bob, 50] call pca_fnc_generateBuildingWaypoints. 
	
	Returns: 
	the total building counts that are assigned a waypoint.
*/

params ["_group", "_pos", "_clearRadius", "_nearbyBuildings", "_waypoint"];

private _nearbyBuildings = (nearestObjects [_pos, ["HOUSE"], _clearRadius]);
private _availableBuildings = 0;

_group setSpeedMode "FULL";
_group setCombatMode "RED";

{
	_x setUnitPos "UP";
	_x allowFleeing 0;
} forEach units _group;

{
	private _i = 0;
	while {str (_x buildingPos _i) != "[0,0,0]"} do 
	{
		_i = _i + 1;
	};
	
	_waypoint = _group addWaypoint [(getPos _x), 0];
	_waypoint setWaypointType "MOVE";
	_waypoint setWaypointHousePosition random(_i);
	_waypoint waypointAttachObject _x;
	_availableBuildings = _availableBuildings + 1;
} forEach _nearbyBuildings;

//Return
_availableBuildings;