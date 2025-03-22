/**
	Saves the data for use later
	
	Parameters: 
	0: the unit <OBJECT>
	
	Example: 
	bob call pca_fnc_saveCrew;
	
	Returns: 
	an array of the unit, unit role, cargo index, and turret path
*/

if (!isServer) exitWith {};

params ["_crew"];

_crew params ["_unit", "_role", "_cargoIndex", "_turretPath"];

private _crewData = [];

_crewData pushBack ([_unit] call pca_fnc_saveUnit);
_crewData pushBack _role;
_crewData pushBack _cargoIndex;
_crewData pushBack _turretPath;

//return
_crewData;