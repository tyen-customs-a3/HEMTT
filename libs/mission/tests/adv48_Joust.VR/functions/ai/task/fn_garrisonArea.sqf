/**
	Modified from nkenny's Garrison Script
	Order units to occupy buildings near the target
	
	Parameters:
	0: group to garrison the area, either unit or group <OBJECT> 
	1: area to occupy <ARRAY> or <OBJECT>
	2: the search radius for buildings, default 100 <NUMBER>
	2: teleport units to position <BOOL>
	3: sort based on height <BOOL>
	4: exit conditions that breaks a unit free, (-2 Random, -1 All, 0 Hit, 1 Fired, 2 Fired Near), default -2 <NUMBER>
	5: patrols the area in addition to garrisons <BOOL>
	
	Example:
	[bob, getPos bob, 100] call pca_fnc_garrisonArea;
	
	Returns: 
	BOOL
*/

if (canSuspend) exitWith {[pca_fnc_garrisonArea, _this] call CBA_fnc_directCall;};

params 
[
	["_grp", grpNull, [grpNull, objNull]],
	["_pos", []],
	["_radius", 100],
	["_teleport", true],
	["_sortBasedOnHeight", false], 
	["_exitCondition", -2],
	["_patrol", false]
];

if (!local _grp) exitWith {false};

if (_grp isEqualTo objNull) then 
{
	_grp = group _grp;
};

if (_pos isEqualTo []) then 
{
	_pos = _grp;
};

_pos = _pos call CBA_fnc_getPos;

private _potentialPos = [_pos, _radius, true, false] call pca_fnc_findBuildings;
_potentialPos = _potentialPos select {lineIntersects [AGLToASL _x, (AGLToASL _x) vectorAdd [0, 0, 6]]};

[_potentialPos, true] call CBA_fnc_shuffle;

if (_sortBasedOnHeight) then 
{
	_potentialPos = [_potentialPos, [], {_x select 2}, "DESCEND"] call BIS_fnc_sortBy;
};

_grp setBehaviour "SAFE";
_grp enableAttack false;

private _units = (units _grp) select {!isPlayer _x && {isNull objectParent _x}};
reverse _units;

if (_patrol) then 
{
	private _patrolGrp = createGroup [(side _grp), true];
	[_units deleteAt 0] join _patrolGrp;
	
	if (count _units > 4) then 
	{
		[_units deleteAt 0] join _patrolGrp;
	};
	
	_patrolGrp setGroupIDGlobal [format ["Patrol (%1)", groupId _patrolGrp]];
	[_patrolGrp, _pos, _radius, 4, nil, true] call pca_fnc_patrolLoop;
};

_units = _units - [objNull];

if (count _units > count _potentialPos) then 
{
	_units resize (count _potentialPos);
};

private _fnc_addEventHandler = 
{
	params ["_unit", "_type"];
	
	if (_type == 0) exitWith {};
	if (_type == -2) then 
	{
		_type = floor (random 4);
	};
	
	private _eventHandlers = _unit getVariable ["garrisonEventHandlers", []];
	switch (_type) do 
	{
		case 1: 
		{
			private _handle = _unit addEventHandler 
			["Hit", 
				{
					params ["_unit"];
					
					[_unit, "PATH"] remoteExec ["enableAI", _unit];
					_unit setCombatMode "RED";
					[_unit, _unit getVariable ["garrisonEventHandlers", []]] call pca_fnc_removeEventHandlers;
					_unit setVariable ["garrisonEventHandlers", nil];
				}
			];
			
			_eventHandlers pushBack ["Hit", _handle];
		};
		case 2: 
		{
			private _handle = _unit addEventHandler 
			["Fired", 
				{
					params ["_unit"];
					
					[_unit, "PATH"] remoteExec ["enableAI", _unit];
					_unit setCombatMode "RED";
					[_unit, _unit getVariable ["garrisonEventHandlers", []]] call pca_fnc_removeEventHandlers;
					_unit setVariable ["garrisonEventHandlers", nil];
				}
			];
			
			_eventHandlers pushBack ["Fired", _handle];
		};
		case 3: 
		{
			private _handle = _unit addEventHandler 
			["FiredNear", 
				{
					params ["_unit", "_shooter", "_distance"];
					
					if (side _unit != side _shooter && {_distance < (10 + random 15)}) then 
					{
						[_unit, "PATH"] remoteExec ["enableAI", _unit];
						_unit doMove (getPosATL _shooter);
						_unit setCombatMode "RED";
						[_unit, _unit getVariable ["garrisonEventHandlers", []]] call pca_fnc_removeEventHandlers;
						_unit setVariable ["garrisonEventHandlers", nil];
					};
				}
			];
			
			_eventHandlers pushBack ["FiredNear", _handle];
		};
		case 4: 
		{
			private _handle = _unit addEventHandler 
			["Suppressed", 
				{
					params ["_unit"];
					
					[_unit, "PATH"] remoteExec ["enableAI", _unit];
					_unit setCombatMode "RED";
					[_unit, _unit getVariable ["garrisonEventHandlers", []]] call pca_fnc_removeEventHandlers;
					_unit setVariable ["garrisonEventHandlers", nil];
				}
			];
			
			_eventHandlers pushBack ["Suppressed", _handle];
		};
	};
	
	_unit setVariable ["garrisonEventHandlers", _eventHandlers];
};

{
	doStop _x;
	
	private _house = _potentialPos deleteAt 0;
	
	if (_teleport) then 
	{
		if (surfaceIsWater _house) then 
		{
			_x doFollow (leader _x);
		}
		else
		{
			_x setPos _house;
			_x disableAI "PATH";
			_x setUnitPos selectRandomWeighted ["UP", 0.75, "MIDDLE", 0.25];
		};
	}
	else
	{
		if (surfaceIsWater _house) exitWith 
		{
			_x doFollow (leader _x);
		};
		
		_x doMove _house;
		
		[
			{
				params ["_unit", ""];
				
				unitReady _unit;
			}, 
			{
				params ["_unit", "_target"];
				
				if (surfaceIsWater (getPos _unit) || (_unit distance _target > 1.5)) exitWith 
				{
					_unit doFollow (leader _unit);
				};
				
				_unit disableAI "PATH";
				_unit setUnitPos selectRandomWeighted ["UP", 0.75, "MIDDLE", 0.25];
			}, [_x, _house]
		] call CBA_fnc_waitUntilAndExecute;
	};
	
	if (_exitCondition == -1) then 
	{
		for "_i" from 0 to 4 do 
		{
			[_x, _i] call _fnc_addEventHandler;
		};
	}
	else
	{
		[_x, _exitCondition] call _fnc_addEventHandler;
	};
	
} forEach _units;

//end
true;
