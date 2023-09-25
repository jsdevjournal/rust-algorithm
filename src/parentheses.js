
var MAP_CODE = {
    123: 125,
    91: 93,
    40: 41,
}

var log = (val) => {
    console.log(val);
    return val;
}

/**
 * @param {string} s
 * @return {boolean}
 */
var isValid = function(s) {
    if (s == "") return true;
    if (s.length % 2 == 1) return false;
    
    var closeChar = [s]
        .map(val => val.charCodeAt(0))
        .map(val => MAP_CODE[val])
        .map(val => String.fromCharCode(val))[0];


    var index = [s]
        .map(val => {
            for (var i = 0; i < val.length; i++) {
                if (val[i] === closeChar) {
                    if (i % 2 == 1) {
                        return i;
                    }
                }
            }
            return -1;
        })[0];
        
    return index != -1 && isValid(s.slice(1, index) + s.slice(index + 1));
};

console.log(isValid("()"), `isValid("()")`);
console.log(isValid("()[]{}"), `isValid("()[]{}")`);
console.log(isValid("(]"), `isValid("(]")`);
console.log(isValid("[(])"), `isValid("[(])")`);
console.log(isValid("([{()}])[]"), `isValid("([{()}])")`);