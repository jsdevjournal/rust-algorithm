// you can write to stdout for debugging purposes, e.g.
// console.log('this is a debug message');
const inRange = (x: number, min: number, max: number, exclusive: boolean) => {
    const res = (x - min) * (x - max);
    return exclusive ? res < 0 : res <= 0;
};

function solution(Y: number[]): number {
    // Implement your solution here
    const results: Record<number, number> = {};
    const maxY: number = Math.max(...Y);
    const minY: number = Math.min(...Y);

    for (let currentY: number = minY; currentY <= maxY; currentY += 1) {
        results[currentY] = 0;
        results[currentY + 0.5] = 0;

        for (let i: number = 0; i < Y.length; i++) {
            if (inRange(currentY + 0.5, Y[i], Y[i + 1], false)) {
                results[currentY + 0.5] += 1;
            }

            if (currentY === Y[i]) {
                results[currentY] += 1;
            } else if (inRange(currentY, Y[i], Y[i - 1], true)) {
                results[currentY] += 1;
            }
        }
    }
    return Math.max(...Object.values(results));
}


function solution2(Y: number[]): number {
    // Implement your solution here
    const results: Record<any, number> = {};

    for (let i: number = 0; i < Y.length; i++) {
        if (i === 0) {
            results[i] = 1;
        } else {
            const val: number = Y[i];
            const prevVal: number = Y[i - 1];
            const step: number = prevVal >= val ? -0.5 : 0.5;
            for (let j = prevVal + step; j !== val + step; j += step) {
                results[j] = (results[j] || 0) + 1;
            }
        }
    }
    return Math.max(...Object.values(results));
}

function solution3(Y: number[]): number {
    // Implement your solution here
    const results: Record<any, number> = {};

    for (let i: number = 0; i < Y.length; i++) {
        const val: number = Y[i];
        if (i === 0) {
            results[val] = 1;
        } else {
            const prevVal: number = Y[i - 1];
            const step: number = prevVal >= val ? -1 : 1;
            const halfStep: number = step > 0 ? -0.5 : 0.5;
            for (let j = prevVal + step; j !== val + step;) {
                results[j] = (results[j] || 0) + 1;
                results[j + halfStep] = (results[j + halfStep] || 0) + 1;
                j += step;
            }
        }
    }
    return Math.max(...Object.values(results));
}

function generateLargeArray(size) {
    const arr: number[] = [];
    for (let i = 0; i < size; i++) {
        arr.push(Math.floor(Math.random() * 100)); // Adjust the range as needed
    }
    return arr;
}

const largeArray = generateLargeArray(10000); // Generate a large array with 1 million elements

console.time('solution');
solution(largeArray);
solution(largeArray);
solution(largeArray);
solution(largeArray);
solution(largeArray);
solution(largeArray);
console.timeEnd('solution');

console.time('solution2');
solution2(largeArray);
solution2(largeArray);
solution2(largeArray);
solution2(largeArray);
solution2(largeArray);
solution2(largeArray);
console.timeEnd('solution2');

console.time('solution3');
solution3(largeArray);
solution3(largeArray);
solution3(largeArray);
solution3(largeArray);
solution3(largeArray);
solution3(largeArray);
console.timeEnd('solution3');

