import * as path from "path";
import { Opts } from "./opts";

export enum Operation {
    Print,
    Add,
    Remove,
}

export type Config = {
    args: string[],
    operation: Operation,
    config: string, // actual path
    pwd: string,
}

export default function getConfig(opts: Opts): Config {
    return {
        args: getArgs(opts),
        config: configFromOpts(opts),
        pwd: getPwd(opts),
        operation: getOperation(opts),
    }
}

function getOperation(opts: Opts): Operation {
    if (!opts.args || opts.args.length === 0) { return Operation.Print; }

    switch (opts.args[0]) {
        case "add": return Operation.Add;
        case "remove": return Operation.Remove;
        default: return Operation.Print;
    }
}

function getPwd(opts: Opts): string {
    if (opts.pwd) {
        return opts.pwd;
    }
    return process.cwd();
}

function configFromOpts(opts: Opts): string {
    if (opts.config) {
        return opts.config;
    }

    const home = process.env["HOME"];
    const location = process.env["XDG_CONFIG_HOME"] || home;
    if (!location) {
        throw new Error("unable to determine config location");
    }

    if (location === home) { return path.join(location, ".projector.json"); }
    return path.join(location, "projector", "projector.json");
}

function getArgs(opts: Opts): string[] {
    const args = opts.args;
    if (!args || args.length === 0) { return []; };

    const operation = getOperation(opts);
    switch (operation) {
        case Operation.Print:
            if (args.length > 1) {
                throw new Error(
                    `expected 0 or 1 positional argument, got ${args.length}`);
            }
            return args;
        case Operation.Add:
            if (args.length !== 3) {
                throw new Error(
                    `expected 2 positional arguments, got ${args.length - 1}`);
            }
            return args.slice(1);
        default: // Operation.Remove
            if (args.length !== 2) {
                throw new Error(
                    `expected 1 positional arguments, got ${args.length - 1}`);
            }
            return args.slice(1);
    }
}

