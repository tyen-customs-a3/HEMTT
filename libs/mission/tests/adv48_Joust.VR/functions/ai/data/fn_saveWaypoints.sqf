/**
	Saves the data for use later
	
	Parameters: 
	0: the group <GROUP>
	
	Example: 
	(group bob) call pca_fnc_saveWaypoints;
	
	Returns: 
	list of waypoints of the group
*/

params ["_grp"];

_waypointList = [];

if (count (waypoints _grp) > 1) then 
{
	for "_waypointIndex" from 1 to (count (waypoints _grp) - 1) do 
	{
		_currentWaypoint = [];
		
		_currentWaypoint pushBack waypointPosition [_grp, _waypointIndex];
		_currentWaypoint pushBack waypointType [_grp, _waypointIndex];
		_currentWaypoint pushBack waypointBehaviour [_grp, _waypointIndex];
		_currentWaypoint pushBack waypointCombatMode [_grp, _waypointIndex];
		_currentWaypoint pushBack waypointFormation [_grp, _waypointIndex];
		_currentWaypoint pushBack waypointSpeed [_grp, _waypointIndex];
		_currentWaypoint pushBack waypointCompletionRadius [_grp, _waypointIndex];
		_currentWaypoint pushBack waypointTimeout [_grp, _waypointIndex];
		_currentWaypoint pushBack waypointStatements [_grp, _waypointIndex];
		_currentWaypoint pushBack waypointScript [_grp, _waypointIndex];
		
		_waypointList set [_waypointIndex, _currentWaypoint];
	};
};

//return
_waypointList;