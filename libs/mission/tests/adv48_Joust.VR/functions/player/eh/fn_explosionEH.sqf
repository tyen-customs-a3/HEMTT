/**
	Event handler that handles what happens when there's an explosion near player
	
	Parameters:
	0: <NONE>
	
	Returns: 
	NONE
*/

if (vehicle player != player || {!alive player}) exitWith {};

private _blur = ppEffectCreate ["dynamicBlur", 402];

_blur ppEffectEnable true;
_blur ppEffectAdjust [5];
_blur ppEffectCommit 0;
_blur ppEffectAdjust [0.0];
_blur ppEffectCommit 1.25;