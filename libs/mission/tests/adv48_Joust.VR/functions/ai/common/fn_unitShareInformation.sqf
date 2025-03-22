/**
	Modified from nkenny's information sharing script
	Unit shares knowledge about target with nearby allies
	
	Parameters:
	0: unit sharing information <OBJECT>
	1: enemy target <OBJECT>
	2: range to share information, default 500 <NUMBER>
	
	Example:
	[bob, joe, 500] call pca_fnc_unitShareInformation;
	
	Returns: 
	BOOL
*/

params ["_unit", ["_target", objNull], ["_range", 500]];

if !(_unit call pca_fnc_isAlive) exitWith {false};

if (isNull _target) then 
{
	_target = _unit findNearestEnemy _unit;
};

private _grp = group _unit;
private _side = side _grp;
private _grps = allGroups select 
{
	private _leader = leader _x;
	_leader distance2D _unit < _range 
	&& {simulationEnabled (vehicle _leader)}
	&& {((side _x) getFriend _side) > 0.6}
	&& {(behaviour _leader) isNotEqualTo "CARELESS"}
	&& {!isPlayer _leader}
	&& {_x isNotEqualTo _grp}
};

if !(isNull _target) then 
{
	private _knowsAbout = (_unit knowsAbout _target) min 4;
	
	{
		[_x, [_target, _knowsAbout]] remoteExec ["reveal", leader _x];
	} forEach _grps;
};

//end
true;