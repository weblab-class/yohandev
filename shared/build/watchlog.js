/**
 * [watchlog.js] Build script plugin for watch mode.
 */
import ora from "ora";
import chalk from "chalk";

export const WatchLog = { plugin };

/**
 * Creates an `esbuild` plugin that prints error messages in watch mode.
 * @returns {import("esbuild").Plugin}
 */
export function plugin({ name }) {
    // Pretty format a warning or error.
    function logError(err) {
        // Skip message formatting.
        if (err.detail?.rendered) {
            console.error(err.text);
            return;
        }
        // Error message:
        console.error("\n", chalk.redBright.bold(err.text), "\n");
        // Extra info:
        if (!err.location) return;
        const {
            file,
            line,
            column,
            lineText,
            length
        } = err.location;

        console.error("   ", `${file}:${line}:${column}`);
        console.error("      "
            + chalk.whiteBright(line, "│ ")
            + chalk.whiteBright(lineText.slice(0, column))
            + chalk.greenBright(lineText.slice(column, column + length))
            + chalk.whiteBright(lineText.slice(column + length))
        );
        console.error(
            " ".repeat(line.toString().length + 6),
            "╵",
            " ".repeat(column - 1),
            chalk.greenBright("^")
        );
    }
    return {
        name: "log",
        setup(build) {
            const anim = ora({
                text: `Building ${name ?? ""}`,
                color: "cyan",
                prefixText: name ? chalk.grey(`[${name}]`) : "",
            });
            // Override esbuild outputs.
            build.initialOptions.logLevel = "silent";

            build.onStart(() => {
                anim.start();
            });
            build.onEnd((res) => {
                if (res.errors.length !== 0) {
                    anim.fail("Build failed.");
                } else if (res.warnings.length !== 0) {
                    anim.warn("Build has warnings.");
                } else {
                    anim.succeed("Build succeeded.");
                }
                res.errors.concat(res.warnings).forEach((e) => {
                    logError(e)
                });
            });
        }
    };
}