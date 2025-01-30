module.exports = {
    onPreBuild: async () => {
        console.log("Installing Trunk...");
        await require("child_process").execSync("cargo install trunk", { stdio: "inherit" });
    }
};
