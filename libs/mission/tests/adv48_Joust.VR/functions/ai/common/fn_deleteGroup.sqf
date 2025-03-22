/**
	Deletes a group safely — including crewed vehicle if present.
	
	Parameters:
	0: the group <GROUP>
	
	Example:
	[bob] call pca_fnc_deleteGroup;
	
	Returns: 
	NONE
*/

params ["_grp"];

{
	deleteVehicle (vehicle _x);
	deleteVehicle _x;
} forEach units (_grp);

deleteGroup _grp;