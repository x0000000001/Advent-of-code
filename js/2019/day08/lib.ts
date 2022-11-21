import { benchmark, test } from "./main.js";

export const format = (lines: string[]) => {
  return Array.from(lines[0]).map((c) => parseInt(c));
};

const get_layers = (digits, width = 25, height = 6) => {
  let layers = [];
  const area = width * height;
  for (let k = 0; k < digits.length / area; k++) {
    let current_layer = Array.from({ length: height }, () =>
      new Array(width).fill(0)
    );

    for (let i = 0; i < height; i++) {
      for (let j = 0; j < width; j++) {
        current_layer[i][j] = digits[area * k + width * i + j];
      }
    }

    layers.push(current_layer);
  }

  return layers;
};

const get_digit_count = (layer, digit) => {
  let count = 0;

  for (let i = 0; i < layer.length; i++) {
    for (let j = 0; j < layer[0].length; j++) {
      if (layer[i][j] === digit) count++;
    }
  }

  return count;
};

const f0 = (input) => {
  const layers = get_layers(input);
  const max_0s_layer_id = layers
    .map((l) => get_digit_count(l, 0))
    .reduce(
      ([min_score, i_min], score, i) => {
        if (score < min_score) {
          return [score, i];
        } else {
          return [min_score, i_min];
        }
      },
      [Number.MAX_SAFE_INTEGER, -1]
    )[1];
  return (
    get_digit_count(layers[max_0s_layer_id], 1) *
    get_digit_count(layers[max_0s_layer_id], 2)
  );
};

const print_messsage = (layers) => {
  let [w, h] = [layers[0].length, layers[0][0].length];
  let image = Array.from({ length: w }, () => new Array(h).fill(2));
  const get_pixel_value = (i, j) => {
    for (let k = 0; k < layers.length; k++) {
      const element = layers[k][i][j];
      if (element != 2) return element;
    }
    return 2;
  };
  image = image.map((line, i) => line.map((_, j) => get_pixel_value(i, j)));
  for (let i = 0; i < w; i++) {
    let line = "";
    for (let j = 0; j < h; j++) {
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

const f1 = (input) => {
  const layers = get_layers(input);
  print_messsage(layers);
  return 0;
};

test([
  // { f: (x) => get_layers(x, 3, 2), expected: 0 },
  {
    f: (x) => print_messsage(get_layers(x, 2, 2)),
    expected: undefined,
    file: "test_input1.txt",
  },
]);

// 1764 too low
// 2058 too high

benchmark(f0, f1);
