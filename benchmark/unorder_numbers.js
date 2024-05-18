const { unorderedSetNew, unorderedSetAdd, unorderedSetHas } = require('../index.node');

const set = unorderedSetNew();

const iteration = 10_000_000;

for (let i = 0; i < iteration; i++) {
    unorderedSetAdd.call(set, i);
}

for (let i = 0; i < iteration; i++) {
    unorderedSetHas.call(set, i);
}
