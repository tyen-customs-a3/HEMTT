/**
	Saves the data for use later
	
	Parameters: 
	0: the unit <OBJECT>
	
	Example: 
	bob call pca_fnc_saveGroup;
	
	Returns: 
	an array of group data
*/

if (!isServer) exitWith {};

params ["_unit"];

if (typeName _unit == "GROUP") then 
{
	_unit = leader _unit;
};

_groupData = [];
_infantryData = [];
_vehicleData = [];
_vehicles = [];
_waypoints = [];

_unitSide = side _unit;
_unitGroup = (group _unit);
_unitsInGroup = units _unitGroup;
_waypoints = [_unitGroup] call pca_fnc_saveWaypoints;

{
	_x disableAI "ALL";
	_x enableSimulationGlobal false;
	(vehicle _x) enableSimulationGlobal false;
} forEach _unitsInGroup;

{
	if ((vehicle _x) == _x) then 
	{
		_infantryData pushBack ([_x] call pca_fnc_saveUnit);
	}
	else
	{
		if (driver (vehicle _x) == _x) then 
		{
			_vehicleData pushBack ([vehicle _x] call pca_fnc_saveVehicle);
		}
		else
		{
			if ((gunner (vehicle _x) == _x) && ((vehicle _x) isKindOf "StaticWeapon")) then 
			{
				_vehicleData pushBack ([vehicle _x] call pca_fnc_saveVehicle);
			};
		};
	};
} forEach _unitsInGroup;

_groupData = [_unitSide, _vehicleData, _infantryData, _waypoints];

//return
_groupData;