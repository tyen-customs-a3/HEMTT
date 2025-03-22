/**
	Caches a given group of units
	
	Parameters:
	0: a unit <OBJECT>, group <GROUP>, or a list of units <ARRAY>
	1: a unique ID given to the cache <STRING>
	
	Example:
	[group bob, "cached1"] call pca_fnc_unitCache;
	
	Returns: 
	NONE
*/

if (!isServer) exitWith {};

params [["_units", [], ["", objNull, grpNull, []]], ["_uniqueID", "", [""]]];

if (isNil "pca_cachedGroups") then 
{
	pca_cachedGroups = [[], []] call CBA_fnc_hashCreate;
};

private _isReCache = _units isEqualType "";

if (_isReCache) then 
{
	_uniqueID = _units;
	_units = [pca_cachedGroups, _uniqueID] call CBA_fnc_hashGet;
}
else
{
	if (!(_units isEqualType [])) then 
	{
		if (_units isEqualType objNull) exitWith 
		{
			_units = [_units];
		};
		if (_units isEqualType grpNull) then 
		{
			_units = units _units;
		};
	};
};

if (_uniqueID == "" || {count _units == 0}) exitWith {false};

{
	_x allowDamage false;
	_x hideObjectGlobal true;
	_x enableSimulationGlobal false;
	
	private _veh = vehicle _x;
	if (_veh != _x && {simulationEnabled _veh}) then 
	{
		_veh enableSimulationGlobal false;
		_veh hideObjectGlobal true;
	};
} forEach (_units select {alive _x && {simulationEnabled _x} && {!isPlayer _x}});

if (_isReCache) exitWith {true};

private _existingUnits = [pca_cachedGroups, _uniqueID] call CBA_fnc_hashGet;

if (count _existingUnits == 0) exitWith 
{
	[pca_cachedGroups, _uniqueID, _units] call CBA_fnc_hashSet;
	true;
};

{
	_existingUnits pushBackUnique _x;
} forEach _units;

[pca_cachedGroups, _uniqueID, _existingUnits] call CBA_fnc_hashSet;

//end
true;
