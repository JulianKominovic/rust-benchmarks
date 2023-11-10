const fs = require("fs");

try {
  fs.mkdirSync(`./testings`);
} catch (e) {
  console.log(e);
}
for (let i = 0; i < 100_000; i++) {
  fs.writeFileSync(`./testings/${i}.js`, "");
}

