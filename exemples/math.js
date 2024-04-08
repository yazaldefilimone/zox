const NUM_ITERATIONS = 1000000000;

// Teste com Math.random()
let sum1 = 0;
for (let i = 0; i < NUM_ITERATIONS; i++) {
  sum1 += Math.random()
}
console.log(sum1)