/**
	Applies saved waypoint data
	
	Parameters: 
	0: the group <GROUP>
	1: list of waypoint <ARRAY>
	
	Example: 
	bob call pca_fnc_applyWaypoints;
	
	Returns: 
	NONE
*/
params ["_newGrp", "_waypointList"];

if (!(_waypointList isEqualTo [])) then 
{
	for "_waypointIndex" from 1 to (count (_waypointList) - 1) do 
	{
		_currentWaypoint = _waypointList select _waypointIndex;
		_newWaypoint = _newGrp addWaypoint [(_currentWaypoint select 0), _waypointIndex];
		
		_newWaypoint setWaypointType (_currentWaypoint select 1);
		_newWaypoint setWaypointBehaviour (_currentWaypoint select 2);
		_newWaypoint setWaypointCombatMode (_currentWaypoint select 3);
		_newWaypoint setWaypointFormation (_currentWaypoint select 4);
		_newWaypoint setWaypointSpeed (_currentWaypoint select 5);
		_newWaypoint setWaypointCompletionRadius (_currentWaypoint select 6);
		_newWaypoint setWaypointTimeout (_currentWaypoint select 7);
		_newWaypoint setWaypointStatements (_currentWaypoint select 8);
		_newWaypoint setWaypointScript (_currentWaypoint select 9);
	};
};