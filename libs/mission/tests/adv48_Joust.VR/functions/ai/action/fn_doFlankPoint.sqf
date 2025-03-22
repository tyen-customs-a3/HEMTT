/**
	Modified from nkenny's Flanking Script
	actualizes the flank cycle
	
	Parameters:
	0: the number of cycles to repeat <NUMBER> 
	1: unit list <ARRAY>
	2: list of positions to utilize <ARRAY>
	3: destination <ARRAY>
	4: radius added to overwatch's position <NUMBER>
	
	Example:
	[15, units bob] call pca_fnc_doFlankPoint;
	
	Returns: 
	BOOL
*/

params ["_cycle", "_units", "_pos", "_overwatch", "_radius"]; 

_units = _units select {(_x call pca_fnc_isAlive) && {_x distance2D _overwatch > 12} && {!isPlayer _x}};

if (_radius > 0) then 
{
	_overwatch = _overwatch vectorAdd [((random (_radius * 2)) - _radius), ((random (_radius * 2)) - _radius), 0];
};

{
	_x doMove _overwatch;
	_x setDestination [_overwatch, "LEADER PLANNED", true];
} forEach _units;

if !(_cycle <= 1 || {_units isEqualTo []}) then 
{
	[
		{_this call pca_fnc_doFlankPoint},
		[_cycle - 1, _units, _pos, _overwatch],
		10 + random 10
	] call CBA_fnc_waitAndExecute;
};

//end
true;
