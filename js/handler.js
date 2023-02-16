//@ts-check
"use strict";

const sum_numbers = (count, loop) => {
  const nums = [...Array(count).keys()];

  const total = nums.reduce((acc, num) => {
    return acc + num;
  }, 0);
  console.log(`Sum: ${total} | loop ID: ${loop}`);
};

module.exports.sum = async (event) => {
  const { count, loop } = event;

  const arr = Array(loop).fill(0);
  arr.map((_, i) => {
    sum_numbers(count, i);
  });

  return;
};
