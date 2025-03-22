/**
	Removes Event Handlers
	
	Parameters:
	0: unit to remove Event Handlers <POSITION>
	
	Example:
	[bob] call pca_fnc_removeEventHandlers;
	
	Returns: 
	NONE
*/

params ["_unit", "_eventHandlers"];

{
	_unit removeEventHandler _x;
} forEach _eventHandlers;