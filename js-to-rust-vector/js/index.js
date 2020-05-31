import('../pkg/index.js').then(mod => {
  const numbers = [12, 10, 2, 3];
  const sumOfNumbers = mod.add(numbers);
  console.log(`Sum of numbers: ${sumOfNumbers}`);

  const strings = ['Apples', 'Bananas', 'Oranges'];
  const totalNumberChars = mod.get_total_characters(strings);
  console.log(`Total number of characters: ${totalNumberChars}`);
});
