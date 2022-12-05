"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.format = void 0;
const main_js_1 = require("./main.js");
const string_to_mask = (s) => {
    return Array.from(s).map((c) => {
        switch (c) {
            case "0":
                return 0;
            case "1":
                return 1;
            default:
                return 2;
        }
    });
};
const format = (lines) => {
    return lines.map((l) => {
        if (l.charAt(1) === "a") {
            return ["mask", string_to_mask(l.split(" ")[2])];
        }
        else {
            let words = l.split(" ");
            let temp = words[0].split("[");
            return [
                "mem",
                parseInt(temp[1].slice(0, temp[1].length - 1)),
                parseInt(words[2]),
            ];
        }
    });
};
exports.format = format;
const number_to_bits36 = (x) => {
    let bits = new Array(36).fill(false);
    for (let i = 0; i < 36; i++) {
        if (x % 2 === 1) {
            bits[35 - i] = true;
            x = (x - 1) / 2;
        }
        else {
            x /= 2;
        }
    }
    return bits;
};
const bits36_to_number = (bits) => {
    let x = 0;
    for (let i = 0; i < 36; i++) {
        x *= 2;
        if (bits[i]) {
            x++;
        }
    }
    return x;
};
const apply_mask = (bits, mask) => {
    return new Array(36)
        .fill(false)
        .map((_, i) => (mask[i] === 2 ? bits[i] : mask[i] === 1));
};
const f0 = (input) => {
    let mem = {};
    let current_mask = null;
    for (let i = 0; i < input.length; i++) {
        const command = input[i][0];
        switch (command) {
            case "mask":
                current_mask = input[i][1];
                break;
            case "mem":
                mem[input[i][1]] = apply_mask(number_to_bits36(input[i][2]), current_mask);
                break;
        }
    }
    return Object.values(mem).reduce((a, b) => a + bits36_to_number(b), 0);
};
const next_combination = (l, index = 0) => {
    if (index === l.length || index === -1)
        return false;
    if (!next_combination(l, index + 1)) {
        if (l[index] === false) {
            for (let k = l.length - 1; k > index; k--) {
                l[k] = false;
            }
            l[index] = true;
            return true;
        }
        else {
            return false;
        }
    }
    else {
        return true;
    }
};
const apply_mask1 = (bits, mask) => {
    return new Array(36)
        .fill(false)
        .map((_, i) => (mask[i] === 1 ? true : bits[i]));
};
const possible_addresses = (address, mask) => {
    let possibilities = [];
    let representation = apply_mask1(number_to_bits36(address), mask);
    let floating_bits_addresses = mask
        .map((x, i) => [x, i])
        .filter(([x, _]) => x === 2)
        .map(([_, i]) => i);
    let current_combination = new Array(floating_bits_addresses.length).fill(false);
    while (true) {
        let possibility = structuredClone(representation);
        for (let i = 0; i < current_combination.length; i++) {
            possibility[floating_bits_addresses[i]] = current_combination[i];
        }
        possibilities.push(bits36_to_number(possibility));
        if (!next_combination(current_combination)) {
            break;
        }
    }
    return possibilities;
};
const f1 = (input) => {
    let mem = {};
    let current_mask = null;
    for (let i = 0; i < input.length; i++) {
        const command = input[i][0];
        switch (command) {
            case "mask":
                current_mask = input[i][1];
                break;
            case "mem":
                const addresses = possible_addresses(input[i][1], current_mask);
                for (let k = 0; k < addresses.length; k++) {
                    mem[addresses[k]] = number_to_bits36(input[i][2]);
                }
                break;
        }
    }
    return Object.values(mem).reduce((a, b) => a + bits36_to_number(b), 0);
};
(0, main_js_1.test)([
    { f: f0, expected: 165 },
    { f: f1, expected: 208, file: "test_input1.txt" },
]);
(0, main_js_1.benchmark)(f0, f1);
//# sourceMappingURL=lib.js.map