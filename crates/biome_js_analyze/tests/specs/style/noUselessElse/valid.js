/* should not generate diagnostics */
function f(x) {
    if (x < 0) {

    } else {
        return 0
    }
}

function f(x) {
    if (x < 0) {
        g();
    } else {
        return 0
    }
}

function f(x) {
    if (x < 0) {
        g();
        if(x === -1) {
            return 1;
        }
    } else {
        return 0
    }
}

function f(x) {
    if (x < 0) {
        throw new RangeError();
    }
    return x;
}

function f(x) {
    if (x < 0) {
        while(x < 0) {
            x += f(x);
        }
    } else {
        return 0
    }
}

function f (x) {
    if (x > 0 && x < 5) {
        switch (x) {}
    } else {
        return x;
    }
}

function f (x) {
    if (x > 0 && x < 5) {
        switch (x) {
            case 0:
            case 1:
                return 0;
            default:
        }
    } else {
        return x;
    }
}

function f (x) {
    if (x > 0 && x < 5) {
        switch (x) {
            case 0:
                break;
            default:
                break;
        }
    } else {
        return x;
    }
}

function f (x) {
    if (x) {
        // do nothing
    } else if (x) {
        return true;
    } else {
        // do nothing
    }
}
