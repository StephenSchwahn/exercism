export default class CircularBuffer<T> {
  private buffer: (T | null)[];
  private oldestPointer: number;
  private newestPointer: number;
  private _size: number;

  constructor(private readonly capacity: number) {
    this.buffer = Array.from({ length: capacity }, () => null);

    // Oldest pointer always points at the oldest value
    this.oldestPointer = 0;

    // Newest pointer always points to the newest value, but
    // in a zero-length buffer, we want it to insert at index 0
    // even when there's no value. To account for the 0 case, we
    // just set the pointer to the end and set size to 0
    this.newestPointer = this.capacity - 1;
    this._size = 0;
  }

  write(value: T): void {
    if (this._size >= this.capacity) {
      throw new BufferFullError();
    }

    this.newestPointer = (this.newestPointer + 1) % this.capacity;
    this.buffer[this.newestPointer] = value;
    this._size++;
  }

  read(): T {
    if (this._size === 0) {
      throw new BufferEmptyError();
    }

    const value = this.buffer[this.oldestPointer];
    this.oldestPointer = (this.oldestPointer + 1) % this.capacity;
    this._size--;

    // Typescript yelling that this could be null because we havent coerced the type
    // but since _size is non-zero, we know that oldestPointer MUST be at a valid value.
    return value!;
  }

  forceWrite(value: T): void {
    if (this._size !== this.capacity) {
      return this.write(value);
    }
    // Force writing is logically the same as read() followed by write(). Using those helpers will
    // prevent nightmares with pointer math.
    this.read();
    this.write(value);
  }

  clear(): void {
    this.buffer = Array.from({ length: this.capacity }, () => null);
    this.oldestPointer = 0;
    this.newestPointer = this.capacity - 1;
    this._size = 0;
  }

  size(): number {
    return this._size;
  }
}

class CircularBufferError extends Error {}
class BufferFullError extends CircularBufferError {
  constructor() {
    super("Cannot Write to a full Buffer");
  }
}

class BufferEmptyError extends CircularBufferError {
  constructor() {
    super("Cannot read from an empty buffer");
  }
}