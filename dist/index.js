#!/usr/bin/env node
import fs from "fs";
import path from "path";
import chalk from "chalk";
import { program } from "commander";
// @ts-ignore
import * as glslTypes from "./pkg/glsl_types.cjs";
// @ts-ignore
global.logln = function (message) { return console.log(message); };
// @ts-ignore
global.log = function (message) { return process.stdout.write(message); };
// @ts-ignore
global.log_with_color = function (message, color) {
    // @ts-ignore
    process.stdout.write(chalk[color](message));
};
// @ts-ignore
global.read_file = function (file) { return fs.readFileSync(file, "utf8"); };
// @ts-ignore
global.canonicalize = function (file) { return path.resolve(file); };
// @ts-ignore
global.file_exists = function (file) { return fs.existsSync(file); };
// @ts-ignore
global.create_dir_all = function (dir) { return fs.mkdirSync(dir, { recursive: true }); };
// @ts-ignore
global.write_file = function (file, content) { return fs.writeFileSync(file, content); };
program
    .option("-i, --input <input>", "Input directory", "./shaders")
    .option("-o, --output <output>", "Output directory", "./output")
    .option("-f, --file <file>", "File to process")
    .option("-w, --watch", "Watch for changes", false);
program.parse();
var SHADER_EXTENSIONS = [".vert", ".frag", ".vs", ".fs", ".glsl"];
var options = program.opts();
if (options.watch) {
    process.stdout.write(chalk["green"]("Watching for changes\n"));
    fs.watch(options.input, { recursive: true }, function (eventType, filename) {
        if (!filename)
            return;
        filename = path.resolve(options.input, filename);
        if (SHADER_EXTENSIONS.includes(path.extname(filename))) {
            glslTypes.start_cli(filename, options.input, options.output);
        }
    });
}
else {
    if (!options.file) {
        console.error("Please provide a file to process");
        process.exit(1);
    }
    if (!fs.existsSync(options.file)) {
        console.error("File ".concat(options.file, " does not exist"));
        process.exit(1);
    }
    glslTypes.start_cli(options.file, options.input, options.output);
}
