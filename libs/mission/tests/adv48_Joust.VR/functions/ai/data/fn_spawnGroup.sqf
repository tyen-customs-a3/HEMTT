/**
	Retrieves data and assign group data
	
	Parameters: 
	0: group data <ARRAY>
	
	Example: 
	[<groupData>] call pca_fnc_spawnGroup;
	
	Returns: 
	the spawned group
*/

if (!isServer) exitWith {};

params ["_grpData", "_spawnPos", ["_isStationary", false]];

_grpData params ["_unitSide", "_vehicleData", "_infantryData", "_waypoints"];

_newGrp = createGroup _unitSide;

waitUntil {!isNull _newGrp};

{
	_relativePos = (_x # 1) vectorDiff (_vehicleData # 0 # 1);
	_newVehiclePos = _spawnPos vectorAdd _relativePos;
	_newVehicle = [_x, _newGrp, _newVehiclePos] call pca_fnc_spawnVehicle;
	waitUntil {alive _newVehicle};
} forEach _vehicleData;

{
	private _relativePos = [];
	
	if (_vehicleData isEqualTo []) then 
	{
		_relativePos = (_x # 1) vectorDiff (_infantryData # 0 # 1);
	}
	else
	{
		_relativePos = (_x # 1) vectorDiff (_vehicleData # 0 # 1);
	};
	
	_newUnitPos = _spawnPos vectorAdd _relativePos;
	[_x, _newGrp, _newUnitPos, _isStationary] call pca_fnc_spawnUnit;
} forEach _infantryData;

[_newGrp, _waypoints] call pca_fnc_applyWaypoints;

//return
_newGrp;