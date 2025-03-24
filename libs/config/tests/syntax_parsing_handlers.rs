use hemtt_config::{Class, Property, Value, Item, Config};

//------------------------------------------------------------------------------
// GUIDE: HOW TO ADD NEW TEST COVERAGE
//------------------------------------------------------------------------------
// 1. CAPTURE REAL SYNTAX PATTERNS:
//    - Tests should preserve the exact syntax structure found in real config files
//    - Transfer the pattern faithfully, maintaining spacing, structure, and style
//    - When adding a new test, examine actual config files to find unique patterns
//
// 2. USE GENERIC PROPERTY NAMES:
//    - Replace domain-specific names with generic ones (foo, bar, baz, etc.)
//    - Maintain the original syntax style (camelCase, snake_case, etc.)
//    - The focus is on testing the syntax pattern, not specific property names
//
// 3. TEST COVERAGE SHOULD BE MINIMAL BUT COMPLETE:
//    - Each test should focus on a single syntax pattern
//    - Include only the necessary code to test that pattern
//    - Be thorough - cover edge cases found in the wild
//
// 4. CREATING A NEW TEST:
//    - Use the test_config_parse! macro
//    - Name the test clearly (test_feature_being_tested)
//    - Provide a minimal example that isolates the syntax feature
//    - Include a descriptive error message
//
// 5. EXAMPLE PROCESS:
//    - Find interesting syntax in real config: `suspTravelDirection[] = SUSPTRAVELDIR_LEFT;`
//    - Create test with generic name: `directionArray[] = CONST_VALUE;`
//    - Preserve exact syntax structure while making names generic
//
//------------------------------------------------------------------------------

mod parse_helpers {
    use super::*;
    use std::sync::Arc;
    use hemtt_preprocessor::Processor;
    use hemtt_workspace::{LayerType, Workspace, reporting::WorkspaceFiles};
    use hemtt_common::config::PDriveOption;
    use std::path::PathBuf;
    use tempfile::tempdir;

    pub fn parse_config(content: &str) -> Result<Config, Vec<Arc<dyn hemtt_workspace::reporting::Code>>> {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let temp_path = PathBuf::from(temp_dir.path());
        
        let workspace = Workspace::builder()
            .physical(&temp_path, LayerType::Source)
            .finish(None, false, &PDriveOption::Disallow)
            .unwrap();

        let temp_file = temp_dir.path().join("test.hpp");
        std::fs::write(&temp_file, content).unwrap();

        let source = workspace.join("test.hpp").unwrap();
        let processed = Processor::run(&source).unwrap();
        println!("\nProcessed output:\n{}", processed.as_str());
        
        let parsed = hemtt_config::parse(None, &processed);
        let workspacefiles = WorkspaceFiles::new();

        // Print diagnostic output
        match parsed {
            Ok(config) => {
                if !config.codes().is_empty() {
                    println!("\nWarnings/Notes:");
                    for code in config.codes() {
                        if let Some(diag) = code.diagnostic() {
                            println!("{}", diag.to_string(&workspacefiles));
                        }
                    }
                }
                Ok(config.into_config())
            }
            Err(errors) => {
                println!("\nErrors:");
                for error in &errors {
                    if let Some(diag) = error.diagnostic() {
                        println!("{}", diag.to_string(&workspacefiles));
                    }
                }
                Err(errors)
            }
        }
    }
}

use parse_helpers::parse_config;

macro_rules! test_config_parse {
    ($test_name:ident, $config:expr, $message:expr) => {
        #[test]
        fn $test_name() {
            let config = $config;
            let result = parse_config(config);
            assert!(result.is_ok(), $message);
        }
    };
}

//==============================================================================
// BASIC EVENT HANDLER TESTS
//==============================================================================
// Tests for basic event handlers with various formatting styles
//==============================================================================

test_config_parse!(
    test_basic_event_handlers,
    r#"
    class Test {
        // Basic event handlers
        onLoad = "hint 'Dialog loaded';";
        onUnload = "hint 'Dialog closed';";
        
        // Event handlers with different formatting
        onMouseEnter="hint 'Mouse entered';";
        onMouseExit   =    "hint 'Mouse exited';"    ;
    };
    "#,
    "Failed to parse basic event handlers"
);

//==============================================================================
// COMPLEX EVENT HANDLERS WITH SQF CODE
//==============================================================================
// Tests for event handlers containing more complex SQF code blocks
//==============================================================================

test_config_parse!(
    test_complex_event_handlers,
    r#"
    class Test {
        // Complex SQF with parameter handling
        onLoad = "params ['_display']; _display setVariable ['var', true]; hint 'Loaded';";
        
        // SQF with conditional logic
        onUnload = "
            if (!isNull player) then {
                hint 'Dialog closed';
            } else {
                diag_log 'No player';
            };
        ";
    };
    "#,
    "Failed to parse complex event handlers with SQF code"
);

//==============================================================================
// EVENT HANDLERS WITH IRREGULAR FORMATTING
//==============================================================================
// Tests for event handlers with unusual formatting and spacing patterns
//==============================================================================

test_config_parse!(
    test_irregular_event_handler_formatting,
    r#"
    class   EventTest    {
        // Event with excessive spaces around equals sign
        onLoad     =      "hint 'Loaded';"     ;
        
        // Event handler with no spaces
        onUnload="hint 'Unloaded';";
        
        // Event handler with tab indentation
        	onEnable = "hint 'Enabled';";
            	onDisable="hint 'Disabled';";
        
        // Event handler with excessive line breaks
        onMouseEnter
            = 
                "hint 'Mouse entered';";
                
        // Event handler with no space before semicolon
        onMouseExit="hint 'Mouse exited'"    ;
        
        // Event handlers on the same line with mixed spacing
        onFocus="hint 'Focus';";  onKillFocus  =  "hint 'Lost focus';"    ;
    };
    "#,
    "Failed to parse event handlers with irregular formatting"
);

//==============================================================================
// EXTREME EVENT HANDLER FORMATTING TESTS
//==============================================================================
// Tests for event handlers with extremely unusual formatting patterns
//==============================================================================

test_config_parse!(
    test_extreme_event_handler_formatting,
    r#"
    class 
    
    
    ExtremeEventHandlers 
    
    
    {
        // Event handler on multiple lines with excessive whitespace
        onLoad
        
        
        
        = 
        
        
        "hint 'Loaded';"
        
        
        ;
        
        // Event handlers with no space and chained on same line
        onUnload="hint 'Unloaded';";onEnable="hint 'Enabled';";
        
        // Event handler with multiline SQF and irregular indentation
        onButtonClick = "
            private 
                _display 
                    = 
                        findDisplay 
                            12345;
            
            if (
                true
            ) then {
                hint 
                    
                    'Button clicked';
                
                } else {
                    
                    hint 
                        'Not clicked';
            };
        ";
        
        // Mixed event handlers with extreme variations in style
        onMouseButtonDown="hint 'Down';";    onMouseButtonUp     =     "hint 'Up';"   ;
        
        // Event handler with mixed quotes and escaped characters
        onChar = "hint ""Character entered: \"" + _this + \"\"""    ;
    };
    "#,
    "Failed to parse event handlers with extreme formatting variations"
);

//==============================================================================
// EVENT HANDLER ARRAY SYNTAX TESTS
//==============================================================================
// Tests for event handlers that use array syntax with irregular formatting
//==============================================================================

test_config_parse!(
    test_event_handler_arrays,
    r#"
    class Test {
        // Event handler with simple array
        onLoad[] = {"hint 'Loaded';"};
        
        // Event handler array with irregular spacing
        onUnload    [   ]     =     {    "hint 'Unloaded';"    }    ;
        
        // Multiple handler array
        eventHandlers[] = {
            {"Fired", "hint 'Unit fired';"},
            {"Killed", "hint 'Unit killed';"},
            {"Reloaded", "hint 'Unit reloaded';"}
        };
        
        // Irregular array formatting
        mouseEvents[]={"MouseEnter","hint 'Mouse entered';",
            
            "MouseExit",
            
            "hint 'Mouse exited';"};
        
        // Extreme array formatting
        keyEvents
        [
        
        ]
        =
        {
            {
                "KeyDown"    ,     "hint 'Key down';"
            }    ,
            
            
            {
                "KeyUp"
                ,
                "hint 'Key up';"
            }
        }
        ;
    };
    "#,
    "Failed to parse event handler arrays with irregular formatting"
);

//==============================================================================
// DISPLAY EVENT HANDLER TESTS
//==============================================================================
// Tests for DisplayEventHandler specific syntax with formatting variations
//==============================================================================

test_config_parse!(
    test_display_event_handlers,
    r#"
    class Test {
        // Basic display event handler
        onLoad="(_this select 0) displayAddEventHandler ['KeyDown', {if((_this select 1)==1)then{true}else{false}}];";
        
        // Display event handler with irregular spacing
        onUnload   =   "missionNamespace    setVariable    ['display',    nil];"   ;
        
        // Display event handler with excessive line breaks and indentation
        onChildDestroyed
            =
                "
                params
                    [
                        '_display',
                        
                        '_child'
                    ];
                    
                systemChat 
                    str 
                        _child;
                ";
                
        // Multiple event handlers on the same object
        childEvents = "
            (_this select 0) displayAddEventHandler ['KeyDown',{hint 'Key down';}];
            (_this select 0)displayAddEventHandler['KeyUp',{hint 'Key up';}];
            (_this select 0)  displayAddEventHandler  ['MouseButtonDown',  {  hint 'Mouse down';  }  ]  ;
        ";
    };
    "#,
    "Failed to parse display event handlers with formatting variations"
);

//==============================================================================
// CONTROL EVENT HANDLER TESTS
//==============================================================================
// Tests for control-specific event handlers with formatting variations
//==============================================================================

test_config_parse!(
    test_control_event_handlers,
    r#"
    class Test {
        // Basic control event handlers
        class Control {
            idc = 1234;
            
            // Regular formatting
            onMouseEnter = "(_this select 0) ctrlSetTextColor [1,1,1,1];";
            
            // Compact formatting
            onMouseExit="(_this select 0)ctrlSetTextColor[0.5,0.5,0.5,1];";
            
            // Excessive spacing
            onButtonClick    =    "    hint str _this;    [_this select 0]    call    fnc_onClick;    "    ;
            
            // Multiline with irregular indentation
            onButtonDblClick = "
                [
                    _this
                        select
                            0
                ] spawn {
                    params ['_ctrl'];
                    
                    _ctrl    ctrlSetText    'Clicked';
                    sleep   2;
                    _ctrl  ctrlSetText  '';
                };
            ";
            
            // Multiple handlers in a single string with varying formatting
            onEvents = "
                (_this select 0) ctrlAddEventHandler ['MouseEnter',{(_this select 0)ctrlSetTextColor[1,1,1,1];}];
                (_this select 0)ctrlAddEventHandler['MouseExit',{(_this select 0) ctrlSetTextColor [0.5,0.5,0.5,1];}];
            ";
        };
    };
    "#,
    "Failed to parse control event handlers with formatting variations"
);

//==============================================================================
// MISSION EVENT HANDLER TESTS
//==============================================================================
// Tests for mission event handlers with non-uniform formatting
//==============================================================================

test_config_parse!(
    test_mission_event_handlers,
    r#"
    class Test {
        // Map event handler with regular formatting
        onMapSingleClick = "_shift = _this select 4; if (_shift) then {deleteMarker 'marker1';};";
        
        // Key handler with compact format
        onKeyDown="params['_ctrl','_key','_shift','_ctrlKey','_alt'];if(_key==28)then{true}else{false};";
        
        // Draw handler with excessive spacing
        onDraw    =    "    []    call    fnc_drawMap;    "    ;
        
        // Complex handler with mixed formatting and indentation
        onPlayerConnected
            =
                "
                params
                    [
                      '_id'
                        ,
                            '_uid'
                    ,           '_name'
                    ]            ;
                    
                systemChat format['Player connected: %1', _name];
                if(
                    _uid in 
                        adminUIDs
                )then{
                    [_name]call fnc_giveAdmin;
                }else{
                    [
                        _name
                    ]
                        call
                            fnc_welcome
                                ;
                };
                "
                ;
        
        // Multiple handlers on same line with different styles
        onPlayerDisconnected="hint 'Player left';";   onPlayerKilled  =  "hint 'Player died';"   ;
    };
    "#,
    "Failed to parse mission event handlers with non-uniform formatting"
);

//==============================================================================
// COMPLEX EVENT HANDLERS WITH NESTED EXPRESSIONS
//==============================================================================
// Tests for complex event handlers with nested expressions and formatting variations
//==============================================================================

test_config_parse!(
    test_complex_event_handlers_with_nesting,
    r#"
    class   Test    {
        // Complex handler with nested expressions and varying bracket spacing
        onLoad = "
            0 fadeSound 0;    0 fadeMusic 0;
            
            [ player,   [ 'Animation',    {   params [ '_unit' ];   _unit playMove 'AmovPercMstpSrasWrflDnon';   }   ]   ] call BIS_fnc_addEventHandler;
            
            {   
                player removeAction _x   
            } forEach  (  player getVariable  [  'actions',  []  ]  );
            
            enableEnvironment   false  ;   enableRadio   false  ;
        ";
        
        // Handler with oddly formatted nested blocks
        onUnload="
            5
                fadeSound
                    1;
            5
                fadeMusic    
                    1;
            
            {
                if(
                    _x isEqualType
                        0
                )then{
                    player removeAction 
                        _x
                    ;
                };
            }
                forEach
                    (
                        player
                            getVariable
                                [
                                    'actions',
                                    []
                                ]
                    );
            
            enableEnvironment   true  ;   enableRadio   true  ;
        ";
        
        // Event handler with extremely spaced function call syntax
        action = "
            [
                player
                ,
                'target'
                ,
                    [
                            'param1'
                        ,
                            'param2'
                    ]
            ]
                call
                    BIS_fnc_action
            ;
        ";
    };
    "#,
    "Failed to parse complex event handlers with nested expressions and formatting variations"
);

//==============================================================================
// EVENT HANDLERS WITH SQF CONTROL STRUCTURES
//==============================================================================
// Tests for event handlers with SQF control structures and irregular formatting
//==============================================================================

test_config_parse!(
    test_event_handlers_with_sqf_control_structures,
    r#"
    class Test {
        // Event handler with for loop and irregular spacing
        onLoad = "
            for '_i' from 0 to 10 do
            {
                systemChat
                    str
                        _i;
            };
            
            for    '_j'    from    0    to    5    do    {    hint str _j;    };
            
            for'_k'from 0 to 3 do{diag_log str _k;};
        ";
        
        // Event handler with switch statement and varied formatting
        onDraw = "
            switch    (    _type    )    do
            {
                case    'unit'    :    {    hint 'unit';    };
                case    'vehicle'    :
                {
                    hint    'vehicle';
                };
                default
                {
                    hint
                        'default';
                };
            };
            
            switch(_value)do{case 1:{hint '1';};case 2:{hint '2';};default{hint 'other';}};
        ";
        
        // Event handler with while loop and formatting variation
        onUpdate = "
            _i = 0;
            while    {    _i < 10    }    do
            {
                hint    str    _i;
                _i    =    _i + 1;
            };
            
            _j=0;while{_j<5}do{diag_log str _j;_j=_j+1;};
        ";
        
        // Event handler with foreach and irregular spacing
        onInit = "
            {
                _x setDamage 0;
            }    forEach    _units;
            
            {    _x setDir 0;    }forEach _vehicles;
            
            {
                _x hideObject true;
                _x    enableSimulation    false;
            }
                forEach
                    _objects;
                    
            {_x setVariable['isReady',true];}forEach _players;
        ";
    };
    "#,
    "Failed to parse event handlers with SQF control structures and irregular formatting"
);

//==============================================================================
// EVENT HANDLERS WITH WAIT COMMANDS
//==============================================================================
// Tests for event handlers with wait commands and formatting variations
//==============================================================================

test_config_parse!(
    test_event_handlers_with_wait_commands,
    r#"
    class   Test    {
        // Event handler with waitUntil and irregular spacing
        onLoad = "
            waitUntil    {    !isNull    player    };
            waitUntil{!isNull(findDisplay 46)};
            
            waitUntil
            {
                time    >    5
            };
        ";
        
        // Event handler with sleep commands and formatting variations
        onAction = "
            hint 'Starting action';
            sleep    2;
            hint    'Action in progress';
            sleep
                1;
            hint'Action complete';
        ";
        
        // Event handler with spawn and formatting variations
        onButtonClick = "
            []    spawn    {
                hint    'Starting';
                sleep    1;
                hint    'Complete';
            };
            
            [_target]spawn{
                params['_unit'];
                _unit setDamage 0;
            };
            
            [
                _object
                ,
                _value
            ]
                spawn
                    {
                        params
                            [
                                '_obj'
                                ,
                                '_val'
                            ];
                        _obj setVariable ['var',_val];
                    };
        ";
    };
    "#,
    "Failed to parse event handlers with wait commands and formatting variations"
);

//==============================================================================
// EVENT HANDLERS WITH COMPLETE FUNCTIONS
//==============================================================================
// Tests for event handlers containing complete function definitions with formatting variations
//==============================================================================

test_config_parse!(
    test_event_handlers_with_functions,
    r#"
    class Test {
        // Event handler with function definition and irregular spacing
        onLoad = "
            _fn_updateUI = {
                params    ['_display'];
                
                _ctrlTitle    =    _display    displayCtrl    1000;
                _ctrlTitle    ctrlSetText    'Updated Title';
                
                _ctrlContent = _display displayCtrl 1001;
                _ctrlContent    ctrlSetText    'Updated Content';
            };
            
            [_this select 0] call _fn_updateUI;
        ";
        
        // Event handler with multiple function definitions and varying formats
        onAction="
            _fn_doAction={
                params['_unit'];
                _unit setDamage 0;
                _unit setVariable['actionDone',true];
            };
            
            _fn_logAction    =    {
                params    ['_unit'];
                diag_log    format['Action performed on: %1',name _unit];
            };
            
            _fn_notify
                =
                    {
                        params['_msg'];
                        systemChat _msg;
                    };
                    
            [player]call _fn_doAction;
            [player]call _fn_logAction;
            ['Action complete']    call    _fn_notify;
        ";
    };
    "#,
    "Failed to parse event handlers with function definitions and formatting variations"
);

//==============================================================================
// EVENT HANDLERS WITH PREPROCESSOR MACROS
//==============================================================================
// Tests for event handlers containing preprocessor macros with varying formats
//==============================================================================

test_config_parse!(
    test_event_handlers_with_macros,
    r#"
    #define ACTION_HINT(text) hint text
    #define LOG_FORMAT(msg) diag_log format['Log: %1', msg]
    #define EXEC_FUNC(func, arg) [arg] call func
    
    class Test {
        // Event handler with simple macro usage
        onLoad = "ACTION_HINT('Loaded');";
        
        // Event handler with macros and spacing variations
        onUnload    =    "    ACTION_HINT    (    'Unloaded'    )    ;    ";
        
        // Event handler with nested macros
        onAction = "
            ACTION_HINT('Starting action');
            LOG_FORMAT('Action started');
            EXEC_FUNC    (    fnc_action,    player    );
            LOG_FORMAT    (    'Action completed'    );
        ";
        
        // Event handler with token concatenation macros
        onButtonClick = "
            LOG##_ACTION('Button');
            EXEC_##FUNC(fnc_##button, player);
        ";
    };
    "#,
    "Failed to parse event handlers with preprocessor macros and formatting variations"
);

//==============================================================================
// EVENT HANDLERS WITH QUOTED STRING VARIATIONS
//==============================================================================
// Tests for event handlers with different quoted string styles and formatting
//==============================================================================

test_config_parse!(
    test_event_handlers_with_quoted_strings,
    r#"
    class Test {
        // Event handler with nested quotes
        onLoad = "hint ""Dialog with ""nested"" quotes"";";
        
        // Event handler with multiple quote styles and formatting
        onUnload    =    "    hint    ""Dialog with \"nested\" quotes""    ;    ";
        
        // Event handler with complex nested quotes
        onButtonClick = "
            hint ""Button with ""multiple ""nested"" quotes"" clicked"";
            systemChat ""System ""chat"" message"";
        ";
        
        // Event handler with mixed quote styles and irregular spacing
        onAction    =    "
            _text    =    ""Action ""performed"" with 'mixed' quotes"";
            hint    _text;
            
            _message=""Another ""message"" with 'quotes'."";
            systemChat_message;
        ";
    };
    "#,
    "Failed to parse event handlers with quoted string variations and formatting"
);