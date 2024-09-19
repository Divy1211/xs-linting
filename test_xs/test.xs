void test(float f = 0.0) {
    int cp = xsGetContextPlayer();
    xsChatData("cp: %d", cp);
}

void test(float f = 0.0) {
    int b = "Oops";
}

void main() {
    test(1);

    float c = 4 + 5.5;
}