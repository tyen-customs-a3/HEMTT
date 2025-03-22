/**
	Modified from nkenny's Assault Script
	Finds nearby buildings and returns an array of buildings or building positions
	
	Parameters:
	0: unit checking <OBJECT> or position <POSITION>
	1: the check radius, default 100 <NUMBER>
	2: should house positions be returned, default false <BOOL>
	3: should only indoor positions be returned, default false <BOOL>
	
	Example:
	[bob, 50, true, true] call pca_fnc_findBuildings;
	
	Returns: 
	An array of available buildings or building positions.
*/

params 
[
	["_unit", objNull, [objNull, []]],
	["_range", 100, [0]],
	["_useHousePos", false, [false]],
	["_onlyIndoor", false, [false]],
	["_findDoors", false, [false]]
];

private _houses = nearestObjects [_unit, ["House", "Strategic", "Ruins"], _range, true];
_houses = _houses select {((_x buildingPos -1) isNotEqualTo [])};

if (!_useHousePos) exitWith {_houses};

private _housePos = [];

{
	private _house = _x;
	_housePos append (_house buildingPos -1);
	
	if (_findDoors) then 
	{
		{
			if ("door" in toLower (_x)) then 
			{
				_housePos pushBack (_house modelToWorld (_house selectionPosition _x));
			};
		} forEach (selectionNames _house);
	};
} forEach _houses;

if (_onlyIndoor) then 
{
	_housePos = _housePos select 
	{
		private _pos = AGLToASL _x;
		lineIntersects [_pos, _pos vectorAdd [0, 0, 6]];
	};
};

//return
_housePos;