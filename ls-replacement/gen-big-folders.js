const fs = require("fs");

try {
  fs.mkdirSync(`./testings`);
} catch (e) {
  console.log(e);
}
for (let i = 0; i < 100000; i++) {
  fs.writeFileSync(
    `./testings/${i}.js`,
    `
      console.log(${i})
      console.log(${i})
      console.log(${i})
      console.log(${i})
      console.log(${i})
      console.log(${i})
      
      `
  );
}
