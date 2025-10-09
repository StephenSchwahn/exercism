export function find<T>(
  input: T[],
  target: T,
  startPointer = 0,
  endPointer = input.length
): number {
  const partition = Math.floor((endPointer - startPointer) / 2) + startPointer;
  if (target === input[partition]) {
    return partition;
  } else if (endPointer - startPointer <= 1) {
    throw new Error('Value not in array')
  } else if (target > input[partition]) {
    return find(input, target, partition, endPointer);
  } else {
    return find(input, target, startPointer, partition);
  }
}