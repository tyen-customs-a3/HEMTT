/* ================================================================================
	GENERAL BRIEFING NOTES
	 - Uses HTML style syntax. All supported tags can be found here - https://community.bistudio.com/wiki/createDiaryRecord
	 - For images use <img image='FILE'></img> (for those familiar with HTML note it is image rather than src).
	 - Note that using the " character inside the briefing block is forbidden use ' instead of ".
*/

/* ================================================================================
	SITUATION
	 - Outline of what is going on, where we are we and what has happened before the mission has started? This needs to contain any relevant background information.
	 - Draw attention to friendly and enemy forces in the area. The commander will make important decisions based off this information.
	 - Outline present weather conditions, players will typically assume that it is daylight with sunny weather.
*/

private _rules = ["diary", ["RULES","
<br/>
<font size='18'>SITUATION</font>
<br/>
The red and blue team will compete to be the last ones standing.
<br/><br/>
- Only fire the RPG from the gunners position of the vehicles.
<br/><br/>
- Each team will enter the arena once the admin begins the game.
<br/><br/>
- Once eliminated, a team can return to the menu and occupy new slots. The game is over once all the slots on one team are eliminated.
<br/><br/>
- Only fire the RPG from the gunners position of the vehicles.
<br/><br/>
- There is a special weapon in the middle of the arena, Only take 1 per vehicle.
"]];

player createDiaryRecord _rules;