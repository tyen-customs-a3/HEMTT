/**
	Spawns existing groups that are synchronized to a trigger in EDEN editor
	
	Parameters: 
	0: the group leader <OBJECT>
	1: number of times the group should be spawned, default 1 <NUMBER>
	2: when should the group be first spawned in seconds, default 0 <NUMBER>
	3: delay cycles between spawn if ticket is more than 1, default 30 <NUMBER>
	4: force the units to be stationary, default false <BOOL>
	5: uses additional spawn positions through markers, default [] <ARRAY>
	6: pause the spawn if enemies are within a given proximity, default 0 <NUMBER>
	7: stop the spawn if exit trigger is activated <TRIGGER>
	8: initializes any addition codes <STRING>
	
	MUST be done in a scheduled environment (SPAWN)
	Units should have their TMF Assign Gear Attributes set
	
	Example: 
	[bob, 2, 10, 30, false, [], 100] spawn pca_fnc_createGroupSync;
	
	Returns: 
	NONE
*/

waitUntil {time > 0};
if (!isServer) exitWith {};

params 
[
	["_unit", objNull],
	["_spawnTicket", 1],
	["_spawnInitial", 0],
	["_spawnDelay", 30],
	["_isStationary", false],
	["_spawnMarkers", []],
	["_pauseRadius", 0],
	["_exitTrigger", objNull],
	["_codeBlock", ""]
];

if (typeName _unit == "GROUP") then 
{
	_unit = leader _unit;
};

private _spawnPos = getPosATL _unit;
private _unitSide = side _unit;
private _groupID = groupId (group _unit);
private _displayName = format ["%1 - %2", _unitSide, _groupID];

private _syncObjectsList = [];
private _spawnPosList = [];
private _spawnPosList pushBack _spawnPos;
private _spawnMarkersList = [];

{
	_spawnPosList pushBack (getMarkerPos _x);
	_spawnMarkersList pushBack (getMarkerPos _x);
} forEach _spawnMarkers;

if (typeName _spawnTicket == "ARRAY") then 
{
	_minTicket = _spawnTicket select 0;
	_maxTicket = _spawnTicket select 1;
	_spawnTicket = _minTicket + floor random (_maxTicket - _minTicket);
};

if (typeName _spawnInitial == "ARRAY") then 
{
	_minInitial = _spawnInitial select 0;
	_maxInitial = _spawnInitial select 1;
	_spawnInitial = _minInitial + floor random (_maxInitial - _minInitial);
};

private ["_trigger"];
_syncObject = synchronizedObjects _unit;

{
	if (_x isKindOf "EmptyDetector") then 
	{
		_trigger = _x;
	} 
	else
	{
		_syncObjectsList append [_x];
	};
} forEach _syncObject;

_grpData = [_unit] call pca_fnc_saveGroup;
[group _unit] call pca_fnc_deleteGroup;

private _firstLoop = true;
private _tempSpawnPos = [];

while {_spawnTicket != 0} do 
{
	sleep 5;
	
	if (!isNil "_trigger") then 
	{
		waitUntil 
		{
			sleep 5;
			
			(triggerActivated _trigger);
		};
	};
	
	if (_firstLoop && _spawnInitial > 0) then 
	{
		sleep _spawnInitial;
		
		_firstLoop = false;
	};
	
	if (count _spawnMarkers > 0) then 
	{
		_tempSpawnPos = selectRandom _spawnMarkersList;
	}
	else 
	{
		_tempSpawnPos = _spawnPos;
	};
	
	while {[_tempSpawnPos, _pauseRadius, _unitSide] call pca_fnc_enemyInRadius} do 
	{
		sleep 10;
	};
	
	private _newGrp = [_grpData, _tempSpawnPos, _isStationary] call pca_fnc_spawnGroup;
	_newGrp setGroupIDGlobal [_groupID];
	_displayName = format ["%1 - %2", side _newGrp, groupId _newGrp];
	
	_newGrp deleteGroupWhenEmpty true;
	_newGrp allowFleeing 0;
	
	_proxy = leader _newGrp;
	
	{
		_proxy synchronizeObjectsAdd [_x];
	} forEach _syncObjectsList;
	
	call compile format [_codeBlock];
	
	sleep 1;
	
	if (typeName _spawnDelay == "ARRAY") then 
	{
		_minTime = _spawnDelay select 0;
		_maxTime = _spawnDelay select 1;
		
		_tempSpawnDelay = _minTime + random (_maxTime - _minTime);
		sleep _tempSpawnDelay;
	}
	else 
	{
		sleep _spawnDelay;
	};
	
	if (!isNil "_exitTrigger") then 
	{
		if (triggerActivated _exitTrigger) then 
		{
			_spawnTicket = 1;
		};
	};
	
	_spawnTicket = _spawnTicket - 1;
	
	deleteGroup _newGrp;
};
