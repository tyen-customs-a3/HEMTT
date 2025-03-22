/**
	Event handler that handles what happens when the player gets hit
	
	Parameters:
	0: <NONE>
	
	Returns: 
	NONE
*/

if (vehicle player != player || {!alive player}) exitWith {};

private _blur = ppEffectCreate ["dynamicBlur", 401];

_blur ppEffectEnable true;
_blur ppEffectAdjust [1];
_blur ppEffectCommit 0;
_blur ppEffectAdjust [0.0];
_blur ppEffectCommit 1;

if (20 > random 100) then 
{
	private _blink = ppEffectCreate ["colorCorrections", 1501];
	_blink ppEffectEnable true;
	_blink ppEffectAdjust [1, 1, 0, [0,0,0,1], [1,1,1,1], [0.25,0.25,0.25,0], [0.1,0.1,0,0,0,0,4]];
	_blink ppEffectCommit 0;
	
	private _h1 = 0.09;
	private _h2 = 0.09;
	
	for "_i" from 0 to 75 step 1 do 
	{
		_h1 = _h1 + 0.01;
		_h2 = _h2 + 0.01;
		
		_blink ppEffectAdjust [1, 1, 0, [0,0,0,1], [1,1,1,1], [0.25,0.25,0.25,0], [_h1,_h2,0,0,0,0,4]];
		_blink ppEffectCommit 0;
		
		sleep 0.01;
	};
	
	_blink ppEffectAdjust [1,1,0, [0,0,0,0], [1,1,1,1], [0.25,0.25,0.25,0], [0,0,0,0,0,0,4]];
	_blink ppEffectCommit 0;
};