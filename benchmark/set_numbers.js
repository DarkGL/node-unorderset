const set = new Set();

const iteration = 10_000_000;

for (let i = 0; i < iteration; i++) {
    set.add(i);
}

for (let i = 0; i < iteration; i++) {
    set.has(i);
}
