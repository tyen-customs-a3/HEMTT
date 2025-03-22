/**
	Saves the data for use later
	
	Parameters: 
	0: the unit <OBJECT>
	
	Example: 
	bob call pca_fnc_saveUnit;
	
	Returns: 
	an array of unit data
*/

if (!isServer) exitWith {};

params ["_unit"];

private _unitData = [];

_unitData pushBack (typeOf _unit);
_unitData pushBack (getPosATL _unit);
_unitData pushBack (_unit getVariable ["tmf_assignGear_role", nil]);
_unitData pushBack (_unit getVariable ["tmf_assignGear_faction", nil]);
_unitData pushBack (vehicleVarName _unit);

//return
_unitData;