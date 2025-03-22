/**
	Order units to patrol in a polygon pattern based on the number of waypoint
	
	Parameters:
	0: the group <GROUP> or unit <OBJECT> performing the patrol
	1: starting patrol position  <ARRAY>
	2: the patrol radius <NUMBER>
	3: amount of waypoint, default 4 <NUMBER>
	4: prioritize waypoints on nearby roads, default true <BOOL>
	
	Example:
	[bob, getPos bob, 200, 4, true] call pca_fnc_patrolAreaPolygon;
	
	Returns: 
	NONE
*/

params ["_grp", "_pos", "_radius", "_numOfWaypoints", "_tryOnRoad"];

//POLYGON
private _roads = [];
private _index = 1;

for "_angle" from 0 to 360 step (360 / _numOfWaypoints) do 
{
	private _point = _pos vectorAdd [_radius * (cos _angle), _radius * (sin _angle), 0];
	private _emptyPos = _point findEmptyPosition [0, 50];
	
	if (count _emptyPos > 0) then 
	{
		_point = _emptyPos;
	};
	
	if (_tryOnRoad) then 
	{
		private _road = [_point, 50, _roads] call BIS_fnc_nearestRoad;
		
		if (!isNull _road) then 
		{
			_point = getPos _road;
			_roads pushBack _road;
		};
	};
	
	private _waypointType = "MOVE";
	
	if (_index == _numOfWaypoints + 1) then 
	{
		_waypointType = "CYCLE";
	};
	
	private _wp = [_grp, _point, 0, _waypointType] call CBA_fnc_addWaypoint;
	
	if (_index == 1) then 
	{
		_wp setWaypointBehaviour "SAFE";
		_wp setWaypointFormation "STAG COLUMN";
		_wp setWaypointSpeed "LIMITED";
	};
	
	_index = _index + 1;
};

/*
//SQUARE
if (_type == 1) then 
{
	_onRoad = _tryOnRoad;
	
	private _quickFunc = 
	{
		if (_onRoad) then 
		{
			private _road = [_this, 50] call BIS_fnc_nearestRoad;
			if (!isNull _road) then 
			{
				_this = getPos _road;
			};
		};
		
		_this
	};
	
	private _topLeft = (_pos vectorAdd [-(_radius / 2), -(_radius / 2), 0]) call _quickFunc;
	private _topRight = (_pos vectorAdd [-(_radius / 2), (_radius / 2), 0]) call _quickFunc;
	private _bottomLeft = (_pos vectorAdd [(_radius / 2), -(_radius / 2), 0]) call _quickFunc;
	private _bottomRight = (_pos vectorAdd [(_radius / 2), (_radius / 2), 0]) call _quickFunc;
	
	private _waypointType = "MOVE";
	private _wp = [_grp, _topLeft, 0, _waypointType] call CBA_fnc_addWaypoint;
	_wp setWaypointBehaviour "SAFE";
	_wp setWaypointFormation "STAG COLUMN";
	_wp setWaypointSpeed "LIMITED";
	
	private _wp = [_grp, _topRight, 0, _type] call CBA_fnc_addWaypoint;
	private _wp = [_grp, _bottomRight, 0, _type] call CBA_fnc_addWaypoint;
	private _wp = [_grp, _bottomLeft, 0, _type] call CBA_fnc_addWaypoint;
	private _wp = [_grp, _bottomLeft, 0, "CYCLE"] call CBA_fnc_addWaypoint;
};
*/
