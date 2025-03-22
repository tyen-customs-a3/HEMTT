/**
	Checks if player is within a given radius
	
	Parameters:
	0: the position <POSITION>
	1: the check radius <NUMBER>
	
	Example:
	[getPos bob, 500] call pca_fnc_playerInRadius;
	
	Returns: 
	True if player is within the given radius, else false.
*/

params ["_pos", "_radius"];


private _playerInRange = false;

{
	if ((_x distance2D _pos) < _radius) exitWith 
	{
		_playerInRange = true;
	};
} forEach playableUnits;

//Return
_playerInRange;