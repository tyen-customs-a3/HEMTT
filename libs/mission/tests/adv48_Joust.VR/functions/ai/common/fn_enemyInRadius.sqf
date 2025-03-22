/**
	Checks if any enemies is within a given radius
	
	Parameters:
	0: the position <ARRAY>
	1: the check radius <NUMBER>
	2: friendly side <SIDE>
	3: excludes plane <BOOL>
	
	Example:
	[getPos bob, 500, east] call pca_fnc_enemyInRadius;
	
	Returns: 
	True if enemy is within the given radius, else false.
*/

params ["_pos", "_radius", "_friendlySide", ["_planeExclusion", true]];

private _nearestEnemies = [];
private _nearestUnits = _pos nearEntities [["Man", "LandVehicle", "Helicopter", "Ship"], _radius];

{
	if ((side _x getFriend (_friendlySide)) < 0.6 && {side _x != CIVILIAN}) then 
	{
		_nearestEnemies = _nearestEnemies + [_x];
	};
} forEach _nearestUnits;

private _enemyInRange = false;

if (count _nearestEnemies > 0) then 
{
	_enemyInRange = true;
};

//Return
_enemyInRange;