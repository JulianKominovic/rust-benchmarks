const fs = require("fs");

for (let i = 0; i < 10000; i++) {
  fs.writeFileSync(`./${i}.js`, `console.log(${i})`);
}
