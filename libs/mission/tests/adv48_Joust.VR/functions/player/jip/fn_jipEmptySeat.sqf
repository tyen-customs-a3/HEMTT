/**
	check target vehicle for empty seats
	
	Parameters:
	0: the target <OBJECT>
	
	Example:
	bob call pca_fnc_jipEmptySeat;
	
	Returns: 
	true if there's available seats in vehicle
*/

params [["_target", objNull, [objNull]]];

private _freeSeat = false;

if (isNull _target) exitWith {_freeSeat};

private _hasDriver = vehicle _target emptyPositions "driver";
private _hasCommander = vehicle _target emptyPositions "commander";
private _hasGunner = vehicle _target emptyPositions "gunner";
private _hasTurret = vehicle _target emptyPositions "turret";
private _hasCargo = vehicle _target emptyPositions "cargo";

if (_hasDriver > 0 || {_hasCommander > 0 || {_hasGunner > 0 || {_hasTurret > 0 || {_hasCargo > 0}}}}) then 
{
	_freeSeat = true;
};

//return
_freeSeat;