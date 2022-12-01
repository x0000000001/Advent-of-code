"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.format = void 0;
var main_js_1 = require("./main.js");
var format = function (lines) {
    return Array.from(lines[0]).map(function (c) { return parseInt(c); });
};
exports.format = format;
var get_layers = function (digits, width, height) {
    if (width === void 0) { width = 25; }
    if (height === void 0) { height = 6; }
    var layers = [];
    var area = width * height;
    for (var k = 0; k < digits.length / area; k++) {
        var current_layer = Array.from({ length: height }, function () {
            return new Array(width).fill(0);
        });
        for (var i = 0; i < height; i++) {
            for (var j = 0; j < width; j++) {
                current_layer[i][j] = digits[area * k + width * i + j];
            }
        }
        layers.push(current_layer);
    }
    return layers;
};
var get_digit_count = function (layer, digit) {
    var count = 0;
    for (var i = 0; i < layer.length; i++) {
        for (var j = 0; j < layer[0].length; j++) {
            if (layer[i][j] === digit)
                count++;
        }
    }
    return count;
};
var f0 = function (input) {
    var layers = get_layers(input);
    var max_0s_layer_id = layers
        .map(function (l) { return get_digit_count(l, 0); })
        .reduce(function (_a, score, i) {
        var min_score = _a[0], i_min = _a[1];
        if (score < min_score) {
            return [score, i];
        }
        else {
            return [min_score, i_min];
        }
    }, [Number.MAX_SAFE_INTEGER, -1])[1];
    return (get_digit_count(layers[max_0s_layer_id], 1) *
        get_digit_count(layers[max_0s_layer_id], 2));
};
var print_messsage = function (layers) {
    var _a = [layers[0].length, layers[0][0].length], w = _a[0], h = _a[1];
    var image = Array.from({ length: w }, function () { return new Array(h).fill(2); });
    var get_pixel_value = function (i, j) {
        for (var k = 0; k < layers.length; k++) {
            var element = layers[k][i][j];
            if (element != 2)
                return element;
        }
        return 2;
    };
    image = image.map(function (line, i) { return line.map(function (_, j) { return get_pixel_value(i, j); }); });
    for (var i = 0; i < w; i++) {
        var line = "";
        for (var j = 0; j < h; j++) {
            switch (image[i][j]) {
                case 0:
                    line += "#";
                    break;
                case 1:
                    line += " ";
                    break;
                case 2:
                    line += " ";
                    break;
            }
        }
        console.log(line);
    }
};
var f1 = function (input) {
    var layers = get_layers(input);
    print_messsage(layers);
    return 0;
};
(0, main_js_1.test)([
    // { f: (x) => get_layers(x, 3, 2), expected: 0 },
    {
        f: function (x) { return print_messsage(get_layers(x, 2, 2)); },
        expected: undefined,
        file: "test_input1.txt",
    },
]);
// 1764 too low
// 2058 too high
(0, main_js_1.benchmark)(f0, f1);
//# sourceMappingURL=lib.js.map