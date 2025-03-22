/**
	Continuously search and hunt the players until the hunter(s) are dead or the target(s) are out of range.
	Each hunter will move to a randomized position picked within an area of the closest target.
	
	Parameters:
	0: list of hunter(s) <ARRAY>
	1: hunt position if no suitable target were found, default [0,0,0] <ARRAY>
	2: the target's hunted radius, default 100 <NUMBER>
	3: maximum range for target search, default 5000 <NUMBER>
	4: list of hunted targets <ARRAY>
	
	MUST be done in a scheduled environment (SPAWN)
	
	Example:
	[units group bob, [0,0,0], 200, 5000, units group joe] spawn pca_fnc_huntTargets;
	
	Returns: 
	NONE
*/

params ["_hunters", ["_position", [0,0,0]], ["_radius", 100], ["_range", 5000], ["_targets", []]];

private _continue = true;

while {_continue} do 
{
	_targets = (_targets select {alive _x}) - [objNull];
	
	{
		private _unit = _x;
		private _closestTarget = _unit findNearestEnemy _unit;
		
		if (isNull _closestTarget) then 
		{
			private _closestFloat = _range * 3;
			
			{
				private _targets = _x;
				private _distanceToTarget = _targets distance _unit;
				
				if (_distanceToTarget < _closestFloat) then 
				{
					_closestTarget = _targets;
					_closestFloat = _distanceToTarget;
				};
			} forEach _targets;
			
			if (isNull _closestTarget) then 
			{
				if (_unit distance _position < _range) then 
				{
					private _dir = [_unit, _position] call BIS_fnc_dirTo;
					_dir = _dir + (((round (random 1)) - 0.5) * 2) * (random 1 * random 1 * 180);
					_unit doMove (getPos (_unit) vectorAdd [(sin(_dir) * 40), (cos(_dir) * 40), 0]);
				}
				else
				{
					_unit doMove _position;
				};
			};
		}
		else
		{
			if ((_targets pushBackUnique _closestTarget) != -1) then 
			{
				{
					_x reveal _closestTarget;
				} forEach _hunters;
			};
		};
		
		if (!isNull _closestTarget) then 
		{
			private _targetsPosition = (getPosATL (_closestTarget) vectorAdd [((random (_radius * 2)) - _radius), ((random (_radius * 2)) - _radius), 0]);
			_unit doMove _targetsPosition;
		};
		
	} forEach _hunters;
	
	sleep 5;
	
	_hunters = _hunters select {alive _x};
	
	if (count _hunters == 0) exitWith {_continue = false;};
	
	sleep 10;
};
