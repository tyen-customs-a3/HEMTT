/**
	Assign TMF faction and role for targeted units.
	
	Parameters:
	0: unit <OBJECT>
	1: unit's role <STRING>
	
	Example:
	[bob, "myFaction", "rm"] call pca_fnc_assignFactionRole;
	
	Returns: 
	NONE
*/

params ["_unit", "_faction", "_role"];

private _faction = pca_opfor_faction;

if (side _unit == west) then 
{
	_faction = pca_blufor_faction;
};

if (_faction != "") then 
{
	_unit setVariable ["tmf_assignGear_faction", _faction];
};

if (!isNil "_role") then 
{
	_unit setVariable ["tmf_assignGear_role", _role];
}
else
{
	_unit setVariable ["tmf_assignGear_role", "rm"];
};

_unit call tmf_assignGear_fnc_assignGear;