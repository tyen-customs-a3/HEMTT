/**
	Check and returns a list of buildings and building positions near a given position.
	
	Parameters:
	0: the position <POSITION>
	1: the check radius <NUMBER>
	
	Example:
	[getPos bob, 500] call pca_fnc_nearbyBuildings;
	
	Returns: 
	A list Buildings and building positions that are within the search area
*/

params ["_centerPos", "_radius"];

private _buildings = [];
_numOfBuildingsPos = 0;

{
	private _buildingPositions = _x buildingPos -1;
	
	if (count _buildingPositions > 0) then 
	{
		_buildings pushBack [_buildingPositions, _x];
		_numOfBuildingsPos = _numOfBuildingsPos + count _buildingPositions;
	};
} forEach nearestObjects [_centerPos, ["Fortress", "House", "House_Small", "Ruins_F", "BagBunker_base_F", "Stall_base_F", "Shelter_base_F"], _radius];

[_buildings, _numOfBuildingsPos];
