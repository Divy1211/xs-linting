include "xslc_core.xs";

int typesString = -1;

void _sendRes(float amount = 0, int type = -1, int player = -1) {
    if(amount == 0)
        return;
    
    if(type == -1)
        return;
    
    string msg = ">>> Given " +amount+ " "+xsArrayGetString(typesString, type)+" to ";

    if(player != -1)
        msg = msg + "player " + player;
    else
        msg = msg + "all players";

    xsEffectAmount(cModResource, type, cAttributeAdd, amount, player);

    xsChatData(msg);
}

void sendResources(int _ = -1) {
    int player = getInt("player");
    _sendRes(getFloat("food"), cAttributeFood, player);
    _sendRes(getFloat("wood"), cAttributeWood, player);
    _sendRes(getFloat("stone"), cAttributeStone, player);
    _sendRes(getFloat("gold"), cAttributeGold, player);
}

void sendMessage(int _ = -1) {
    static int l = 0;
    string msg = getString("message");
    if(msg == "")
        return;

    msg = ">>> "+msg;
    if(l%2 == 0)
        msg = "<GREY>"+msg;

    xsChatData(msg);
}

rule _starter__main
    active
    runImmediately
    highFrequency
    priority 1000
{
    typesString = xsArrayCreateString(4, "", "typeString");
    xsArraySetString(typesString, cAttributeFood, "food");
    xsArraySetString(typesString, cAttributeWood, "wood");
    xsArraySetString(typesString, cAttributeStone, "stone");
    xsArraySetString(typesString, cAttributeGold, "gold");
    xsDisableSelf();
}