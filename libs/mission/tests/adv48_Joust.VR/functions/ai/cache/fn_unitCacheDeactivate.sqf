/**
	UnCaches all units with a given unique ID.
	
	Parameters:
	0: unique ID STRING <STRING>
	
	Example:
	["cached1"] call pca_fnc_unitCacheDeactivate;
	
	Returns: 
	NONE
*/

if (!isServer || {isNil "pca_cachedGroups"}) exitWith {false};
if (!params [["_uniqueID", "", [""]]]) exitWith {false};

private _units = [pca_cachedGroups, _uniqueID] call CBA_fnc_hashGet;

{
	_x allowDamage true;
	_x enableSimulationGlobal true;
	_x hideObjectGlobal false;
	
	private _veh = vehicle _x;
	if (_veh != _x && {!simulationEnabled _veh}) then 
	{
		_veh enableSimulationGlobal true;
		_veh hideObjectGlobal false;
	};
} forEach _units;

//end
true;
