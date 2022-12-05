"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.format = void 0;
var main_js_1 = require("./main.js");
var format = function (lines) {
    var words = lines[0].split(" ");
    return [parseInt(words[0]), parseInt(words[6])];
};
exports.format = format;
var CircularLinkedList = /** @class */ (function () {
    function CircularLinkedList(val, left, right) {
        if (left === void 0) { left = null; }
        if (right === void 0) { right = null; }
        this.val = val;
        if (right === null) {
            this.right = this;
            this.left = this;
        }
        else {
            this.right = right;
            this.left = left;
        }
    }
    CircularLinkedList.prototype.insert = function (val) {
        var newRight = new CircularLinkedList(this.val, this, this.right);
        this.right.left = newRight;
        this.right = newRight;
        this.val = val;
        if (this.left == this) {
            this.left = newRight;
        }
    };
    CircularLinkedList.prototype.pop = function () {
        // doesn't work with 1 item list
        var temp = this.val;
        this.val = this.right.val;
        this.right = this.right.right;
        this.right.right.left = this;
        return temp;
    };
    CircularLinkedList.prototype.rotateLeft = function (n) {
        if (n == 0) {
            return this;
        }
        else {
            return this.right.rotateLeft(n - 1);
        }
    };
    CircularLinkedList.prototype.rotateRight = function (n) {
        if (n == 0) {
            return this;
        }
        else {
            return this.left.rotateRight(n - 1);
        }
    };
    CircularLinkedList.prototype.current = function () {
        return this.val;
    };
    CircularLinkedList.prototype.print = function (pointer) {
        if (pointer === void 0) { pointer = null; }
        if (pointer == this) {
            console.log("\n");
            return;
        }
        if (pointer == null) {
            pointer = this;
        }
        process.stdout.write(this.val.toString() + " ");
        console.log(this.val.toString() + " ");
        this.right.print(pointer);
    };
    return CircularLinkedList;
}());
var get_best_score = function (_a) {
    var players_count = _a[0], marbles_count = _a[1];
    var list = new CircularLinkedList(0);
    var scores = new Array(players_count).fill(0);
    for (var step = 1; step < marbles_count + 1; step++) {
        // list.print();
        if (step % 23 == 0) {
            var player = (step - 1) % players_count;
            list = list.rotateRight(7);
            scores[player] += list.pop() + step;
        }
        else {
            list = list.rotateLeft(2);
            list.insert(step);
        }
    }
    return scores.reduce(function (a, b) { return (a > b ? a : b); }, 0);
};
var f0 = function (_a) {
    var players_count = _a[0], marbles_count = _a[1];
    return get_best_score([players_count, marbles_count]);
};
var f1 = function (_a) {
    var players_count = _a[0], marbles_count = _a[1];
    return get_best_score([players_count, marbles_count * 100]);
};
(0, main_js_1.test)([
    { f: get_best_score, expected: 32, file: "test_input.txt" },
    { f: get_best_score, expected: 8317, file: "test_input0.txt" },
    { f: get_best_score, expected: 146373, file: "test_input1.txt" },
]);
(0, main_js_1.benchmark)(f0, f1);
// const get_best_score_naive = ([players_count, marbles_count]) => {
//   let scores = new Array(players_count).fill(0);
//   let marbles = [0, 1];
//   let current = 1;
//   for (let i = 2; i < marbles_count + 1; i++) {
//     if (
//       current != get_current_and_length(i)[0] ||
//       marbles.length != get_current_and_length(i)[1]
//     ) {
//       console.log("wrong current");
//       console.log(current, get_current_and_length(i));
//     }
//     if (i % 23 === 0) {
//       const player_id = (i - 1) % players_count;
//       current = (current - 7 + marbles.length) % marbles.length;
//       const removed_marble = marbles.splice(current, 1)[0];
//       scores[player_id] += i + removed_marble;
//       if (removed_marble != get_removed_marble(i)) {
//         console.log("wrong removed at step", i);
//         console.log(removed_marble, get_removed_marble(i));
//       }
//     } else {
//       current = current + 2;
//       if (current > marbles.length) {
//         current = current - marbles.length;
//       }
//       marbles.splice(current, 0, i);
//     }
//   }
//   return scores.reduce((a, b) => (a > b ? a : b), 0);
// };
// const get_best_score_efficient = ([players_count, marbles_count]) => {
//   let scores = new Array(players_count).fill(0);
//   for (let i = 23; i < marbles_count + 1; i += 23) {
//     const player_id = (i - 1) % players_count;
//     scores[player_id] += get_removed_marble(i) + i;
//   }
//   return scores.reduce((a, b) => (a > b ? a : b), 0);
// };
// const get_best_score_efficient = ([players_count, marbles_count]) => {
//   let scores = new Array(players_count).fill(0);
//   let removed_track = [];
//   let [current, length] = get_current_and_length(marbles_count);
//   let step = marbles_count;
//   while (step > 0) {
//     if (step % 23 === 0) {
//       const player_id = (step - 2) % players_count;
//       scores[player_id] += step;
//       removed_track.push([player_id, (current - 7 + length) % length]);
//     }
//     for (let index = 0; index < removed_track.length; index++) {
//       if (current <= removed_track[index][1] && (step - 1) % 23 == 0) {
//         removed_track[index][1]++;
//       } else if (current < removed_track[index][1]) {
//         removed_track[index][1]--;
//       }
//     }
//     if ((step - 1) % 23 === 0) {
//       current = current + 7;
//       if (current > length + 1) {
//         current = current - (length + 1);
//       }
//       length++;
//     } else {
//       current = current - 2;
//       if (current < 0) {
//         current = current + length - 1;
//       }
//       length--;
//     }
//     step--;
//     if ((step - 1) % 23 !== 0) {
//       for (let index = 0; index < removed_track.length; index++) {
//         if (removed_track[index][1] === current) {
//           scores[removed_track[index][0]] += step - 1;
//           removed_track.splice(index, 1);
//           break;
//         }
//       }
//     }
//   }
//   return scores.reduce((a, b) => (a > b ? a : b), 0);
// };
//# sourceMappingURL=lib.js.map