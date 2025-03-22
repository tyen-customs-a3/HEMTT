/**
	Modified from jokoho482
	Finds suitable overwatch positions
	
	Parameters:
	0: target's position <ARRAY>
	1: max range from target <NUMBER>
	2: min range from target  <NUMBER>
	3: min height in relation to target <NUMBER>
	4: position to start looking from <ARRAY>
	
	Example:
	[getPos bob, 100, 50, 8, getPos joe] call pca_fnc_findOverwatch;
	
	Returns: 
	An array of possible overwatch positions.
*/

scriptName "findOverwatch";
scopeName "findOverwatch";

params 
[
	["_targetPos", [0, 0, 0], [[]]],
	["_maxRange", 50, [0]],
	["_minRange", 10, [0]],
	["_minHeight", 8, [0]],
	["_centerPos", [0, 0, 0], [[]]]
];

private _refObject = nearestObject [_targetPos, "All"];
private _result = [];
private _selectedPositions = [];

private _quickFnc = 
{
	private _heightSorted = _selectedPositions apply {[(_refObject worldToModel _x) select 2, _x]};
	_heightSorted sort false;
	
	_result = (_heightSorted param [0]) param [1, _centerPos];
	_result breakOut "findOverwatch";
};

for "_i" from 0 to 300 do 
{
	private _checkPos = [_centerPos, 0, _maxRange, 3, 0, 50, 0, [], []] call BIS_fnc_findSafePos;
	private _height = (_refObject worldToModel _checkPos) select 2;
	private _dist = _checkPos distance _targetPos;
	
	private _terrainBlocked = terrainIntersect [_targetPos, _checkPos];
	
	private _distCheck = (_dist > _minRange);
	if (_result isEqualTo [] && _distCheck) then 
	{
		if !(_terrainBlocked) then 
		{
			_result = _checkPos;
		};
	};
	
	if ((_height > _minHeight) && _distCheck) then 
	{
		if !(_terrainBlocked) then 
		{
			_selectedPositions pushBack _checkPos;
		};
	};
	
	if (count _selectedPositions >= 5) then 
	{
		call _quickFnc;
	};
};

if (_selectedPositions isNotEqualTo []) then 
{
	call _quickFnc;
}
else
{
	if (_result isEqualTo []) then 
	{
		_result = _centerPos;
	};
};

//return
_result;