/**
	Order group to move to nearby buildings within a given radius.
	
	Parameters:
	0: the group <GROUP>
	1: the position where the search radius originates from <LOCATION>
	2: the search radius for buildings <NUMBER> (DEFAULT: 50)
	3: total count of all assigned buildings  <NUMBER>
	
	Example:
	[bob, joe, 50] call pca_fnc_clearBuildings;
	
	Returns: 
	NONE
*/

params ["_group", "_pos", ["_clearRadius", 50], "_totalAvailableWaypoints"];

_totalAvailableWaypoints = 0;
_totalAvailableWaypoints = [_group, _pos, _clearRadius] call pca_fnc_generateBuildingWaypoints; 

if (_totalAvailableWaypoints == 0) then 
{
	[_group, [(getPos (leader _group)), _clearRadius, _clearRadius, 0, false]] call CBA_fnc_taskSearchArea;
};

((count (waypoints _group)) - currentWaypoint _group);