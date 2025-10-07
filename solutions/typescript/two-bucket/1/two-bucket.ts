class BucketError extends Error {
  constructor(protected reason: string) {
    super()
  }  
}

class BothBucketsTooSmallError extends BucketError {
  constructor() {
    super('Neither bucket has the capacity to hold requested amount');
  }
}

class GCDInvalidError extends BucketError {
  constructor() {
    super('GCD of buckets was not compatable with the provided goal');
  }
}

class NotYetCalculatedError extends BucketError {
  constructor() {
    super('Must run the moves() function to generate the answers');
  }
}

type BucketType = 'one' | 'two';
type Bucket = { which: BucketType, capacity: number, waterLevel : number };

type NotStarted = { type: 'not_started', error: NotYetCalculatedError }; 
type InvalidState = { type: 'invalid_state', error: BucketError };
type ValidResult = { type: 'valid', numberOfMoves: number, goalBucket: BucketType, otherBucket: number };
type Result = NotStarted | InvalidState | ValidResult;

function isValidResult(result: Result): result is ValidResult {
  return result.type === 'valid';
}

function gcd(a: number, b: number): number {
  return b === 0 ? a : gcd(b, a % b);
}

export class TwoBucket {
  private currentResult: Result = { type: 'not_started', error: new NotYetCalculatedError() };
  constructor(
    private readonly bucketOneSize: number,
    private readonly bucketTwoSize: number,
    private readonly goal: number,
    private readonly startingBucket: BucketType,
  ) { }

  moves(): number {
    if (this.goal > this.bucketOneSize && this.goal > this.bucketTwoSize) {
      const error = new BothBucketsTooSmallError();
      this.currentResult = { type: 'invalid_state', error: error };
      throw error;
    }
    else if (this.goal % gcd(this.bucketOneSize, this.bucketTwoSize)) {
      const error = new GCDInvalidError();
      this.currentResult = { type: 'invalid_state', error: error };
      throw error;
    }

    let moves = 0;
    const startBucket: Bucket = { 
      which: this.startingBucket, 
      capacity: this.startingBucket === 'one' ? this.bucketOneSize : this.bucketTwoSize, 
      waterLevel: 0 
    };
    const otherBucket: Bucket = { 
      which: this.startingBucket === 'one' ? 'two' : 'one',
      capacity: this.startingBucket === 'one' ? this.bucketTwoSize : this.bucketOneSize,
      waterLevel: 0 
    };
    
    while (!isValidResult(this.currentResult)) {
      if (startBucket.waterLevel === 0) { // Case fillBucket
        startBucket.waterLevel = startBucket.capacity;
      } else if (otherBucket.waterLevel === otherBucket.capacity) { // case emptyBucket
        otherBucket.waterLevel = 0;
      } else if (otherBucket.capacity === this.goal) { // fillOtherBucket
        otherBucket.waterLevel = otherBucket.capacity
      } else {
        const pouredAmount = Math.min(startBucket.waterLevel, otherBucket.capacity - otherBucket.waterLevel);
        startBucket.waterLevel -= pouredAmount;
        otherBucket.waterLevel += pouredAmount;
      }
      moves += 1;
      if (startBucket.waterLevel === this.goal) {
        this.currentResult = { type: 'valid', numberOfMoves: moves, goalBucket: startBucket.which, otherBucket: otherBucket.waterLevel };
      } else if (otherBucket.waterLevel === this.goal) {
        this.currentResult = { type: 'valid', numberOfMoves: moves, goalBucket: otherBucket.which, otherBucket: startBucket.waterLevel };
      }
    } 

    return moves;
    
  }

  get goalBucket(): BucketType {
    if (!isValidResult(this.currentResult)) {
      throw this.currentResult.error;
    }
    return this.currentResult.goalBucket;
  }

  get otherBucket(): number {
    if (!isValidResult(this.currentResult)) {
      throw this.currentResult.error;
    }
    return this.currentResult.otherBucket;
  }
}
