/**
	Gives a unit unlimited magazine reload
	
	Parameters:
	0: the unit <OBJECT>
	
	Example:
	[bob] call pca_fnc_unlimitedAmmo;
	
	Returns: 
	NONE
*/

params ["_unit"];

_unit addEventHandler 
["Reloaded", 
	{
		params ["_unit", "", "", "", "_oldMagazine"];
		
		_oldMagazine params ["_type", "_roundsLeft"];
		
		if (isNil "_type" || {_roundsLeft > 0}) exitWith {};
		
		(_type call BIS_fnc_ItemType) params ["_magType", "_magLoadedWith"];
		
		if (_magType != "Magazine" || {!(_magLoadedWith in ["Bullet", "ShotgunShell"])}) exitWith {};
		_unit addMagazine _type;
	}
];

/* 
//Simple unlimited ammo
_unit addEventHandler ["Reloaded", 
{
	params ["_unit", "", "", "", "_oldMagazine"];
	
	_oldMagazine params ["_oldMagazine"];
	_unit addMagazine _oldMagazine;
}];
*/