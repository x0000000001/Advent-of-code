"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.format = void 0;
const main_js_1 = require("./main.js");
const format = (lines) => {
    let lists = [];
    const get_list = (l) => {
        if (l[0] != "[") {
            return {
                val: parseInt(l),
                children: [],
            };
        }
        let brackets_index = -1;
        let split_indexes = [0];
        for (let i = 0; i < l.length; i++) {
            switch (l[i]) {
                case "[":
                    brackets_index += 1;
                    break;
                case "]":
                    brackets_index -= 1;
                    break;
                case ",":
                    if (brackets_index === 0) {
                        split_indexes.push(i);
                    }
                    break;
                default:
                    break;
            }
        }
        split_indexes.push(l.length - 1);
        let children = [];
        for (let i = 0; i < split_indexes.length - 1; i++) {
            if (split_indexes[i] + 1 === split_indexes[i + 1]) {
                continue;
            }
            children.push(get_list(l.substring(split_indexes[i] + 1, split_indexes[i + 1])));
        }
        return { val: undefined, children };
    };
    for (let i = 0; i < lines.length; i++) {
        lists.push(get_list(lines[i]));
    }
    return { lists };
};
exports.format = format;
const list_to_string = (l) => {
    if (l.val !== undefined) {
        return String(l.val);
    }
    else {
        return "[" + l.children.map(list_to_string).join(",") + "]";
    }
};
const compare_lists = (l0, l1) => {
    // console.log(list_to_string(l0), list_to_string(l1));
    let is_list0 = l0.val === undefined;
    let is_list1 = l1.val === undefined;
    if (!is_list0 && !is_list1) {
        return l0.val > l1.val ? 1 : l1.val > l0.val ? -1 : 0;
    }
    else if (!is_list0 && is_list1) {
        return compare_lists({ val: undefined, children: [l0] }, l1);
    }
    else if (is_list0 && !is_list1) {
        return compare_lists(l0, { val: undefined, children: [l1] });
    }
    else {
        let i = 0;
        let result_if_out_of_items = l0.children.length > l1.children.length
            ? 1
            : l1.children.length > l0.children.length
                ? -1
                : 0;
        while (true) {
            if (i === l0.children.length || i === l1.children.length) {
                return result_if_out_of_items;
            }
            let temp_res = compare_lists(l0.children[i], l1.children[i]);
            switch (temp_res) {
                case 0:
                    break;
                default:
                    return temp_res;
            }
            i++;
        }
    }
};
const f0 = ({ lists }) => {
    let index = 0;
    return Array.from({ length: lists.length / 2 }, (v, i) => i * 2)
        .map((i) => {
        index++;
        if (compare_lists(lists[i], lists[i + 1]) == -1) {
            return index;
        }
        else {
            return 0;
        }
    })
        .reduce((a, b) => a + b, 0);
};
const f1 = ({ lists }) => {
    let packet2 = {
        val: undefined,
        children: [{ val: undefined, children: [{ val: 2, children: [] }] }],
    };
    lists.push(structuredClone(packet2));
    let packet6 = {
        val: undefined,
        children: [{ val: undefined, children: [{ val: 6, children: [] }] }],
    };
    lists.push(structuredClone(packet6));
    lists.sort((l0, l1) => compare_lists(l0, l1));
    let index_packet2 = lists
        .map((l, i) => [compare_lists(l, packet2), i])
        .filter(([comp, _]) => comp === 0)[0][1] + 1;
    let index_packet6 = lists
        .map((l, i) => [compare_lists(l, packet6), i])
        .filter(([comp, _]) => comp === 0)[0][1] + 1;
    return index_packet2 * index_packet6;
};
(0, main_js_1.test)([
    { f: f0, expected: 13 },
    { f: f1, expected: 140 },
]);
(0, main_js_1.benchmark)(f0, f1);
//# sourceMappingURL=lib.js.map