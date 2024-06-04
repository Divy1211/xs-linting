// for logging
const bool verbose = true;

// current execution tick for synchronisation
int tick = 0;

// the tick on which the command should run
int cmdExecTick = -1;
string cmdFunction = ""; // dangerous undefined behaviour
int paramCount = -1;

// all of these are arrays
int intParams = -1;      // each command parameter is stored in the array of its type
int floatParams = -1;    // each command parameter is stored in the array of its type
int boolParams = -1;     // each command parameter is stored in the array of its type
int stringParams = -1;   // each command parameter is stored in the array of its type
int vectorParams = -1;   // each command parameter is stored in the array of its type
int paramNames = -1;     // the name of each parameter
int paramPos = -1;       // the indices of the paramNames in the above array in the array of their types

// we search for the index of the given name in paramNames
// once found, the corresp. index is looked at in the paramPos array
// the element at the position obtained is returned from the array of that type
int getInt(string name = "") {
    for(i = 0; < paramCount) {
        if(xsArrayGetString(paramNames, i) == name)
            return (xsArrayGetInt(intParams, xsArrayGetInt(paramPos, i)));
    }
    return (-1);
}
float getFloat(string name = "") {
    for(i = 0; < paramCount) {
        if(xsArrayGetString(paramNames, i) == name)
            return (xsArrayGetFloat(floatParams, xsArrayGetInt(paramPos, i)));
    }
    return (-1.0);
}
bool getBool(string name = "") {
    for(i = 0; < paramCount) {
        if(xsArrayGetString(paramNames, i) == name)
            return (xsArrayGetBool(boolParams, xsArrayGetInt(paramPos, i)));
    }
    return (false);
}
string getString(string name = "") {
    for(i = 0; < paramCount) {
        if(xsArrayGetString(paramNames, i) == name)
            return (xsArrayGetString(stringParams, xsArrayGetInt(paramPos, i)));
    }
    return ("");
}

// so vector arrays cannot be resized in xs atm, thx DE.
vector getVector(string name = "") {
    for(i = 0; < paramCount) {
        if(xsArrayGetString(paramNames, i) == name)
            return (xsArrayGetVector(vectorParams, xsArrayGetInt(paramPos, i)));
    }
    return (cInvalidVector);
}

// increment tick, run loaded commands
rule _ext_core__updateTick
    active
    runImmediately
    minInterval 3
    maxInterval 3
    priority 8
{
    if(verbose)
        xsChatData(">>> Tick: " + tick);

    if (tick == cmdExecTick) {
        xsAddRuntimeEvent("Scenario Triggers", cmdFunction, -1);
        xsAddRuntimeEvent("Random Map", cmdFunction, -1);
    }

    tick = tick + 1;
}

// writes the tick to the file for the client to read, runs a queued command and loads the next command (if any)
rule _ext_core__writeTickToFile
    active
    runImmediately
    minInterval 3
    maxInterval 3
    priority 10
{
    bool createSuccess = xsCreateFile(true);
    if (createSuccess) {
        xsSetFilePosition(0);
        bool writeSuccess = xsWriteInt(tick);
        xsCloseFile();
    }
}

string _ext_core__toStr(bool boolean = false) {
    if(boolean)
        return ("true");
    return ("false");
}

bool _ext_core__toBool(int integer = 0) {
    if(integer == 0)
        return (false);
    return (true);
}

// File layout:
// 1:       Execution Tick Number (int32)
// 2:       cmdFunction (str: int32 followed by that many bytes)
// 3:       paramCount (int32)
// 4-...:   Params (str paramName, int32 paramType, <type> paramValue)
rule _ext_core__loadCommand
    active
    runImmediately
    minInterval 3
    maxInterval 3
    priority 9
{
    bool success = xsOpenFile("xslc.commands");
    if (success == false) {
        xsCloseFile();
        return;
    }

    int newCmdExecTick = xsReadInt();

    // if the new exec tick is the same as the one read previously, then we're reading the same
    // command. return
    // If the new exec tick is different but the current tick has tick passed it, return as well
    if (newCmdExecTick == cmdExecTick || tick > newCmdExecTick) {
        xsCloseFile();
        return;
    }

   cmdExecTick = newCmdExecTick;

   // read the relevant command data
   cmdFunction = xsReadString();
   paramCount = xsReadInt();

    if(verbose) {
        xsChatData(">>> Command Received");
        xsChatData(">>> Tick: "+cmdExecTick);
        xsChatData(">>> Function: "+cmdFunction);
        xsChatData(">>> # Params: "+paramCount);
    }
    
    xsArrayResizeInt(intParams, 0);
    xsArrayResizeFloat(floatParams, 0);
    xsArrayResizeBool(boolParams, 0);
    xsArrayResizeString(stringParams, 0);
    xsArrayResizeVector(vectorParams, 0); // CURRENTLY BROKEN

    xsArrayResizeString(paramNames, paramCount);
    xsArrayResizeInt(paramPos, paramCount);

    for(i = 0; < paramCount) {
        string log = ">>> ";
        string name = xsReadString();
        int type = xsReadInt();


        // types:
        // 0: int
        // 1: float
        // 2: bool
        // 3: string
        // 4: vector
        // add each param name to paramNames and add the actual value of the param to
        // the array of its type and store its index in paramPos
        switch(type) {
            case 0 : {
                int sizeInt = xsArrayGetSize(intParams);
                xsArrayResizeInt(intParams, sizeInt+1);
                
                int valueInt = xsReadInt();
                xsArraySetInt(intParams, sizeInt, valueInt);

                xsArraySetInt(paramPos, i, sizeInt);
                xsArraySetString(paramNames, i, name);
                
                log = log + "int "+name+" = "+valueInt+";";
            }
            case 1 : {
                int sizeFloat = xsArrayGetSize(floatParams);
                xsArrayResizeFloat(floatParams, sizeFloat+1);
                
                float valueFloat = xsReadFloat();
                xsArraySetFloat(floatParams, sizeFloat, valueFloat);

                xsArraySetInt(paramPos, i, sizeFloat);
                xsArraySetString(paramNames, i, name);
                
                log = log + "float "+name+" = "+valueFloat+";";
            }
            case 2 : {
                int sizeBool = xsArrayGetSize(boolParams);
                xsArrayResizeInt(boolParams, sizeBool+1);
                
                bool valueBool = _ext_core__toBool(xsReadInt());
                xsArraySetBool(boolParams, sizeBool, valueBool);

                xsArraySetInt(paramPos, i, sizeBool);
                xsArraySetString(paramNames, i, name);
                
                log = log + "bool "+name+" = "+_ext_core__toStr(valueBool)+";";
            }
            case 3 : {
                int sizeString = xsArrayGetSize(stringParams);
                xsArrayResizeString(stringParams, sizeString+1);
                
                string valueString = xsReadString();
                xsArraySetString(stringParams, sizeString, valueString);
                
                xsArraySetInt(paramPos, i, sizeString);
                xsArraySetString(paramNames, i, name);
                
                log = log + "string "+name+" = "+valueString+";";
            }
            case 4 : {
                int sizeVector = xsArrayGetSize(vectorParams);
                xsArrayResizeVector(vectorParams, sizeVector+1); // CURRENTLY BROKEN

                vector valueVector = xsReadVector();
                xsArraySetVector(vectorParams, sizeVector, valueVector);

                xsArraySetInt(paramPos, i, sizeVector);
                xsArraySetString(paramNames, i, name);

                log = log + "vector "+name+" = vector"+valueVector+";";
            }
        }
        if(verbose)
            xsChatData(log);
    }

    xsCloseFile();
}

// initialise the arrays
rule _ext_core__main
    active
    runImmediately
    highFrequency
    priority 1000
{
    intParams = xsArrayCreateInt(0, 0, "intParams");
    floatParams = xsArrayCreateFloat(0, 0, "floatParams");
    boolParams = xsArrayCreateBool(0, false, "boolParams");
    stringParams = xsArrayCreateString(0, "", "stringParams");
    vectorParams = xsArrayCreateVector(0, cInvalidVector, "vectorParams");
    paramPos = xsArrayCreateInt(0, 0, "paramPos");
    paramNames = xsArrayCreateString(0, "", "paramNames");
    xsDisableSelf();
}