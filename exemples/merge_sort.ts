
const merge_sort = (arr: number[]): number[] => {
  if (arr.length <= 1) return arr;

  const middle = Math.floor(arr.length / 2);
  const left = arr.slice(0, middle);
  const right = arr.slice(middle);

  return _merge(merge_sort(left), merge_sort(right));
}


const _merge = (left: number[], right: number[]): number[] => {
  let result: number[] = [];
  let leftIndex: number = 0;
  let rightIndex: number = 0;

  while (leftIndex < left.length && rightIndex < right.length) {
    if (left[leftIndex] < right[rightIndex]) {
      result.push(left[leftIndex]);
      leftIndex++;
    } else {
      result.push(right[rightIndex]);
      rightIndex++;
    }
  }

  return result.concat(left.slice(leftIndex)).concat(right.slice(rightIndex));
};