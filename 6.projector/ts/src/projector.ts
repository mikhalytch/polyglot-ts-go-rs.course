
import { Config } from "./config"
import * as fs from "fs";
import * as path from "path";

export type Data = {
    projector: {
        [key: string]: { // pwd
            [key: string]: string // key -> value
        }
    }
}

const defaultData = {
    projector: {}
}

export default class Projector {

    constructor(private config: Config, private data: Data) { }

    save() {
        const configPath = path.dirname(this.config.config);
        if (!fs.existsSync(configPath)) {
            fs.mkdirSync(configPath, { recursive: true });
        }
        // just overwrite the file if it's exists
        fs.writeFileSync(this.config.config, JSON.stringify(this.data));
    }

    getValueAll(): { [key: string]: string } {
        let curr = this.config.pwd;
        let prev = "";

        const paths: string[] = [];
        do {
            prev = curr;
            paths.push(curr);
            curr = path.dirname(curr);
        } while (curr != prev);

        return paths.reverse().reduce((acc, path) => {
            const value = this.data.projector[path];
            if (value) { Object.assign(acc, value) }

            return acc;
        }, {})
    }
    getValue(key: string): string | undefined {
        let curr = this.config.pwd;
        let prev = "";

        let out: string | undefined = undefined;
        do {

            const value = this.data.projector[curr]?.[key];
            if (value) { out = value; break; }

            prev = curr;
            curr = path.dirname(curr);

        } while (curr != prev);

        return out;
    }
    setValue(key: string, value: string) {
        let dir = this.data.projector[this.config.pwd];
        if (!dir) {
            dir = this.data.projector[this.config.pwd] = {};
        }
        dir[key] = value;
    }
    removeValue(key: string) {
        const dir = this.data.projector[this.config.pwd];
        if (dir) {
            delete dir[key];
        }
    }

    static fromConfig(config: Config): Projector {
        if (fs.existsSync(config.config)) {

            let data: Data /* | undefined */ /* = undefined */;
            try {
                data = JSON.parse(fs.readFileSync(config.config).toString());
            } catch (e) {
                data = defaultData;
            }
            return new Projector(config, data);
        }
        return new Projector(config, defaultData);
    }
}
