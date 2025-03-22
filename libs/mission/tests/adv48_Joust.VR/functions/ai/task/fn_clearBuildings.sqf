/**
	Modified from nkenny's CQB Script
	Order units to clear out buildings in a given area until all buildings is checked.
	
	Parameters:
	0: the group <GROUP> or unit <OBJECT> 
	1: the target position <ARRAY>
	2: the search radius, default 100 <ARRAY>
	3: delay between each clear cycles, default 20 <NUMBER>
	4: use existing waypoints? default false <BOOL>
	
	MUST be done in a scheduled environment (SPAWN)
	
	Example:
	[bob, getPos joe, 100] spawn pca_fnc_clearBuildings;
	
	Returns: 
	BOOL
*/

if !(canSuspend) exitWith 
{
	_this spawn pca_fnc_clearBuildings;
};

params 
[
	["_grp", grpNull, [grpNull, objNull]],
	["_pos", [0, 0, 0]],
	["_radius", 100, [0]],
	["_cycle", 20, [0]],
	["_useWaypoint", false, [false]]
];

private _fnc_findBuildings = 
{
	params ["_pos", "_radius", "_grp"];
	
	private _building = nearestObjects [_pos, ["house", "strategic", "ruins"], _radius, true];
	_building = _building select {(_x buildingPos -1) isNotEqualTo []};
	_building = _building select {count (_x getVariable [format ["%1_%2", "pca_buildingCleared", str (side _grp)], [0, 0]]) > 0};
	
	if (_building isEqualTo []) exitWith {objNull};
	
	private _nearestBuildings =_building apply {[(leader _grp) distance2D _x, _x]};
	
	_nearestBuildings sort true;
	(_nearestBuildings param [0, [0, objNull]]) param [1, objNull];
};

private _fnc_findTargets = 
{
	params ["_building", "_grp"];
	
	private _pos = [getPos _building, getPos leader _grp] select isNull _building;
	private _target = (leader _grp) findNearestEnemy _pos;
	if (isNull _target || {_pos distance2D _target < 25}) exitWith {_target};
	objNull;
};

private _fnc_compileAct =
{
	params ["_target", "_grp", "_building"];
	
	private _units = (units _grp) select {isNull objectParent _x && {_x call pca_fnc_isAlive}};
	
	if (!isNull _target) exitWith 
	{
		doStop _units;
		
		private _buildingPos = ((nearestBuilding _target) buildingPos -1) select {_x distance _target < 5};
		_buildingPos pushBack (getPosATL _target);
		
		private _buildingPosSelected = selectRandom _buildingPos;
		
		{
			_x forceSpeed 4;
			_x doMove _buildingPosSelected;
			_x lookAt _target;
			true;
		} count _units;
	};
	
	private _buildingPos = _building getVariable [format ["%1_%2", "pca_buildingCleared", str (side _grp)], (_building buildingPos -1) select {lineIntersects [AGLToASL _x, (AGLToASL _x) vectorAdd [0, 0, 10]]}];
	
	{
		private _buildingPosSelected = _buildingPos param [0, []];
		
		if ((_buildingPos isNotEqualTo []) && {unitReady _x}) then 
		{
			_x setUnitPos "UP";
			_x doMove (_buildingPosSelected vectorAdd [0.5 - random 1, 0.5 - random 1, 0]);
			
			if (_x distance _buildingPosSelected < 30 && {(leader _grp isEqualTo _x)}) then 
			{
				_buildingPos deleteAt 0;
			}
			else
			{
				if (_x call pca_fnc_isIndoor && {_x distance _buildingPosSelected > 45} && {!([_x, 50] call CBA_fnc_nearPlayer)}) then 
				{
					_x setVehiclePosition [getPos _x, [], 3.5];
				};
			};
		}
		else 
		{
			if (unitReady _x && {!(_x call pca_fnc_isIndoor)}) then 
			{
				//[_x, getPosASL _building] call pca_fnc_doSuppress;
				_x doFollow leader _x;
			};
		};
		
		true;
	} count _units;
	
	_building setVariable [format ["%1_%2", "pca_buildingCleared", str (side _grp)], _buildingPos];
};

if (!local _grp) exitWith {false};

if (_grp isEqualType objNull) then 
{
	_grp = group _grp;
};

if (_useWaypoint) then 
{
	_pos = [_grp, (currentWaypoint _grp) min ((count waypoints _grp) - 1)];
};

_grp setSpeedMode "FULL";
_grp setFormation "WEDGE";
_grp enableAttack false;
_grp allowFleeing 0;

{
	_x disableAI "SUPPRESSION";
	true;
} count (units _grp);

waitUntil 
{
	waitUntil {sleep 1; simulationEnabled (leader _grp)};
	
	private _wPos = _pos call CBA_fnc_getPos;
	private _building = [_wPos, _radius, _grp] call _fnc_findBuildings;
	private _target = [_building, _grp] call _fnc_findTargets;
	
	if (isNull _building && {isNull _target}) exitWith {false};
	[_target, _grp, _building] call _fnc_compileAct;
	
	sleep _cycle;
	
	((units _grp) findIf {_x call pca_fnc_isAlive} == -1)
};

{
	_x setUnitPos "AUTO";
	_x doFollow (leader _x);
} forEach units _grp;

//end
true;