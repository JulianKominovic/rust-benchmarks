const fs = require("fs");

try {
  fs.mkdirSync(`./testings`);
} catch (e) {
  console.log(e);
}
for (let i = 0; i < 1_000_000; i++) {
  fs.writeFileSync(`./testings/${i}.js`, "");
}
