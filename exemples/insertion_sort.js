function insertionSort(arr) {
  for (let i = 1; i < arr.length; i++) {
    let current = arr[i];
    let j = i - 1;
    while (j >= 0 && arr[j] > current) {
      arr[j + 1] = arr[j];
      j--;
    }
    arr[j + 1] = current;
  }
  return arr;
}

let test = [5, 3, 8, 4, 2, 1, 9, 7, 6];
let result = insertionSort(test);

// first number 
console.log(result[0]);