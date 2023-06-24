const fs = require("node:fs");
const { join } = require("node:path");

(async () => {
	fs.rmSync(join(__dirname, "/../packages/backend/built"), {
		recursive: true,
		force: true,
	});
	fs.rmSync(join(__dirname, "/../packages/backend/native-utils/built"), {
		recursive: true,
		force: true,
	});
	fs.rmSync(join(__dirname, "/../packages/client/built"), {
		recursive: true,
		force: true,
	});
	fs.rmSync(join(__dirname, "/../packages/sw/built"), {
		recursive: true,
		force: true,
	});
	fs.rmSync(join(__dirname, "/../built"), { recursive: true, force: true });
})();
